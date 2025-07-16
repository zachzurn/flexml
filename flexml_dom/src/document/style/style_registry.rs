use std::collections::HashMap;
use crate::document::style::style::{StyleValue, StyleValueParser};

pub type StyleId = usize;

#[derive(Debug, PartialEq, Clone)]
pub struct AtomicStyleEntry {
    style_id: StyleId,
    style_value: StyleValue,
}

pub struct RawStyleEntry<'a> {
    name: &'a str,
    value: Option<&'a str>,
}

/// Style Registry is the central place
/// to register atomic styles (Things like fontSize, fontWeight, etc)
/// and to register built in styles (Things like bold, italic, flex)
///
/// It also allows overwriting styles while protecting atomic styles
/// from being overwritten
pub struct StyleRegistry {
    names: Vec<String>,
    names_map: HashMap<String, StyleId>,
    definitions: HashMap<StyleId, Vec<AtomicStyleEntry>>,
    last_atomic_style: usize,
    atomic_parsers: Vec<StyleValueParser>
}

pub struct RegisteredStyle {
    style_id: StyleId,
    atomic: bool,
    overwrote: bool,
}


impl StyleRegistry {

    pub fn new() -> StyleRegistry {
        StyleRegistry {
            names: vec![],
            names_map: HashMap::new(),
            definitions: HashMap::new(),
            atomic_parsers: Vec::new(),
            last_atomic_style: 0,
        }
    }

    /// Interns a string name and returns its StyleId.
    /// If the name already exists, returns its existing StyleId.
    fn intern_name(&mut self, name: &str) -> StyleId {
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
    ///
    /// Built ins are registered by functions that are responsible for providing style patches.
    pub fn register_atomic_style(&mut self, name: &str, parser: StyleValueParser) -> Result<StyleId, String> {
        if self.last_atomic_style > 0 {
            return Err(format!("Cannot register atomic style '{}': Regular styles have already been registered.", name));
        }

        if self.names_map.contains_key(name) {
            return Err(format!("Atomic style '{}' is already defined.", name));
        }

        let atomic_style = self.intern_name(name);
        self.atomic_parsers.push(parser);

        Ok(self.intern_name(name))
    }

    /// Registers or updates a style definition. This always returns a StyleId
    /// and an indication of whether the returned style definition is atomic.
    ///
    /// Calling this will also lock atomic style registration
    ///
    /// This way we never overwrite built in atomic styles but return the atomic.
    pub fn register_style(&mut self, style_name: &str, definitions: Vec<RawStyleEntry>) -> RegisteredStyle {
        if self.last_atomic_style == 0 { self.last_atomic_style = self.names.len() - 1 }

        let style_id = self.intern_name(style_name);

        // This is an atomic style, cannot be overwritten
        if style_id <= self.last_atomic_style { return RegisteredStyle { style_id, atomic: true, overwrote: false }; }

        // These are going to all be atomic styles
        let mut style_map : HashMap<StyleId, AtomicStyleEntry> = HashMap::new();

        // We expand the raw style entries into definitions
        // Atomic styles are inserted with parsed values
        // Defined styles are expanded up the chain
        // We reverse the order so the later styles get precedence
        definitions.into_iter().rev().for_each(|raw| {
            let style_id = self.intern_name(raw.name);

            if style_id <= self.last_atomic_style {
                // Atomic style, we parse and add
                let parser = &self.atomic_parsers[style_id];

                // Insert if it hasn't already been defined
                if !style_map.contains_key(&style_id) {
                    let value = if let Some(raw_value) = raw.value { parser.parse(raw_value) } else { StyleValue::None };
                    style_map.insert(style_id,AtomicStyleEntry { style_id, style_value: value });
                }
            } else {
                // Defined style we insert only if it doesn't already exist
                let existing_style = self.definitions.get(&style_id);

                if let Some(styles) = existing_style {
                    styles.into_iter().rev().for_each(|entry| {

                    // Insert if it hasn't already been defined
                    if !style_map.contains_key(&style_id) {
                        style_map.insert(style_id,entry.clone());
                    }
                    })
                }
            }
        });

        let style_entries = style_map.into_values().collect::<Vec<_>>();

        // We add or overwrite the style
        let overwritten = self.definitions.insert(style_id, style_entries);

        RegisteredStyle {
            style_id,
            atomic: false,
            overwrote: overwritten.is_some()
        }
    }

    /// Retrieves the definition for a given alias StyleId.
    /// Returns `Some(&Vec<StyleId>)` if the ID corresponds to a registered alias, `None` otherwise.
    pub fn get_definition(&self, id: StyleId) -> Option<&Vec<AtomicStyleEntry>> {
        self.definitions.get(&id)
    }

}