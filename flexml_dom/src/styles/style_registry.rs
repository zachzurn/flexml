use std::collections::{HashMap, HashSet};
use super::style::{AtomicStyle, RawStyle, StyleId, StyleValue, StyleValueParser};

/// Style Registry is the central place
/// to register atomic styles (Things like fontSize, fontWeight, etc)
/// and to register built in styles (Things like bold, italic, flex)
///
/// It also allows overwriting styles while protecting atomic styles
/// from being overwritten
pub struct StyleRegistry {
    names: Vec<String>,
    names_map: HashMap<String, StyleId>,
    definitions: HashMap<StyleId, Vec<AtomicStyle>>,
    last_atomic_style: usize,
    atomic_styles_locked: bool,
    atomic_parsers: Vec<StyleValueParser>
}

pub struct RegisteredStyle {
    pub(crate) style_id: StyleId,
    pub(crate) atomic: bool,
    pub(crate) overwrote: bool,
}

impl StyleRegistry {

    pub fn new() -> StyleRegistry {
        StyleRegistry {
            names: vec![],
            names_map: HashMap::new(),
            definitions: HashMap::new(),
            atomic_parsers: Vec::new(),
            last_atomic_style: 0,
            atomic_styles_locked: false,
        }
    }

    /// Interns a string name and returns its StyleId.
    /// If the name already exists, returns its existing StyleId.
    pub fn intern_name(&mut self, name: &str) -> StyleId {
        if let Some(&id) = self.names_map.get(name) {
            return id;
        }
        let id = self.names.len();
        self.names.push(name.to_string());
        self.names_map.insert(name.to_string(), id);
        id
    }

    /// Resolves a StyleId back to its string name.
    pub fn resolve_name(&self, id: StyleId) -> Option<&str> {
        self.names.get(id).map(|s| s.as_str())
    }

    /// Must be called only *before* any regular styles are registered.
    /// Registers an atomic style (a base style like "fontSize") which has no definition
    /// but which is a style on its own
    pub fn register_atomic_style(&mut self, name: &str, parser: StyleValueParser) -> Result<StyleId, String> {
        if self.atomic_styles_locked {
            return Err(format!("Cannot register atomic style '{}': Regular styles have already been registered.", name));
        }

        if self.names_map.contains_key(name) {
            return Err(format!("Atomic style '{}' is already defined.", name));
        }

        self.last_atomic_style += 1;

        self.atomic_parsers.push(parser);
        Ok(self.intern_name(name))
    }

    /// Registers or updates a style definition. This always returns a StyleId
    /// and an indication of whether the returned style definition is atomic.
    ///
    /// Calling this will also lock atomic style registration
    ///
    /// This way we never overwrite built in atomic styles but return the atomic.
    pub fn register_style(&mut self, style_name: &str, entries: Vec<RawStyle>) -> RegisteredStyle {
        // First style definition locks atomics
        if !self.atomic_styles_locked {
            if self.last_atomic_style == 0 { panic!("Trying to define a style with no atomic styles"); }
            self.atomic_styles_locked = true;
            self.last_atomic_style -= 1;
        }

        let style_id = self.intern_name(style_name);

        // This is an atomic style, cannot be overwritten, we return as such
        if style_id <= self.last_atomic_style {
            return RegisteredStyle { style_id, atomic: true, overwrote: false };
        }

        // This is a custom style, turn into expanded atomic styles
        let style_entries = self.expand_raw_styles(entries);

        // We add or overwrite the style
        let overwrote = self.definitions.insert(style_id, style_entries).is_some();

        RegisteredStyle {
            style_id,
            atomic: false,
            overwrote
        }
    }

    /// Expands a list of raw style entries like "bold", "italic", "fontSize" with value "3", "customStyle"
    /// Into a list of atomic styles like fontWeight: bold, fontSize: 3.0, etc
    pub fn expand_raw_styles(&mut self, entries: Vec<RawStyle>) -> Vec<AtomicStyle> {
        let mut atomic_set : HashSet<StyleId> = HashSet::new();
        let mut atomic_styles = vec![];

        // We expand the raw style entries into definitions
        // Atomic styles are inserted with parsed values
        // Defined styles are expanded from already expanded styles
        // We reverse the order so the later styles get precedence
        entries.into_iter().rev().for_each(|raw| {
            let id = self.intern_name(raw.name);

            if id <= self.last_atomic_style {
                // Atomic style, we parse and add
                let parser = &self.atomic_parsers[id];

                // Insert if it hasn't already been defined
                if !atomic_set.contains(&id) {
                    let value = if let Some(raw_value) = raw.value { parser.parse(raw_value) } else { StyleValue::Empty };
                    atomic_set.insert(id);
                    atomic_styles.push(AtomicStyle { id, value });
                }
            } else {
                let existing_style = self.definitions.get(&id);

                // We do not need to worry about the order of these atomic entries
                // Since when they were defined we already gave precedence
                if let Some(styles) = existing_style {
                    styles.into_iter().rev().for_each(|entry| {

                        // Insert if it hasn't already been defined
                        if !atomic_set.contains(&entry.id) {
                            atomic_set.insert(entry.id);
                            atomic_styles.push(entry.clone());
                        }
                    })
                }
            }
        });

        atomic_styles.reverse();
        atomic_styles
    }

    /// Retrieves the definition for a given alias StyleId.
    /// Returns `Some(&Vec<StyleId>)` if the ID corresponds to a registered alias, `None` otherwise.
    pub fn get_definition(&self, id: StyleId) -> Option<&Vec<AtomicStyle>> {
        self.definitions.get(&id)
    }
}