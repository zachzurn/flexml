use std::collections::{HashMap, HashSet};
use crate::styles::builtin::{BuiltInStyle, DEFAULT_BUILTINS};
use super::style::{AtomicStyle, RawStyle, StyleId, StyleValue};

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
    first_style: usize,
    first_custom_style: usize,
    builtins_registered: bool,
    builtins: Vec<&'static BuiltInStyle>,
}

pub struct RegisteredStyle {
    /// The style id for the registered style
    pub(crate) style_id: StyleId,

    /// Whether the style was an atomic style
    pub(crate) atomic: bool,

    /// Whether we overwrote an existing style.
    /// Will always be false for atomic styles.
    pub(crate) overwrote: bool,

    /// Is the style a builtin style
    pub(crate) builtin: bool,
}

impl StyleRegistry {

    fn new() -> StyleRegistry {
        StyleRegistry {
            names: vec![],
            names_map: HashMap::new(),
            definitions: HashMap::new(),
            builtins: Vec::new(),
            first_style: 0,
            first_custom_style: 0,
            builtins_registered: false,
        }
    }

    pub fn with_builtins() -> StyleRegistry {
        let mut registry = Self::new();
        registry.register_builtins(DEFAULT_BUILTINS);
        registry
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

    /// Call this once to register built in atomic styles along with their
    /// defined styles (like "bold" for fontWeight: "bold" or an alias with a Forward)
    fn register_builtins(&mut self, builtins: &[&'static BuiltInStyle]) {
        let mut styles = vec![];

        //Register atomics
        // These are fundamental style entries that
        // cannot be overwritten
        for builtin in builtins {
            let atomic_style_id = self.intern_name(builtin.name);
            self.builtins.push(builtin);

            for (style_name, style_value) in builtin.styles.iter() {
                styles.push((atomic_style_id, style_name, style_value));
            }
        }

        self.builtins_registered = true;
        self.first_style = builtins.len();

        // Register styles
        // These are things like "bold" which alias to the atomic style fontWeight
        for (atomic_style_id, style_name, style_value) in styles {
            let style_id = self.intern_name(style_name);

            // Prevent overwriting atomics
            if style_id < self.first_style {
                continue;
            }

            // Forward style values need to point to the atomic_style_value
            // Instead of 0, which is a temporary style_id
            let style_value_forwarded = match style_value {
                // forward to the atomic style's styleId
                StyleValue::Forward(_) => StyleValue::Forward(atomic_style_id),
                _ => style_value.clone(),
            };

            // Directly insert the defined style into the definitions
            self.definitions.insert(style_id, vec![AtomicStyle{ id: atomic_style_id, value: style_value_forwarded }]);
        }

        // The next style registered after this is the first custom style
        self.first_custom_style = self.names.len()
    }

    /// Registers or updates a style definition. This always returns a StyleId
    /// and an indication of whether the returned style definition is atomic.
    ///
    /// Calling this will also lock atomic style registration
    ///
    /// This way we never overwrite built in atomic styles but return the atomic.
    pub fn register_style(&mut self, style_name: &str, entries: Vec<AtomicStyle>) -> RegisteredStyle {
        // First style definition locks atomics
        if !self.builtins_registered {
            if self.first_style == 0 { panic!("Trying to define a style with no atomic styles"); }
            self.builtins_registered = true;
            self.first_style -= 1;
        }

        let style_id = self.intern_name(style_name);

        // This is an atomic style, cannot be overwritten, we return as such
        if style_id < self.first_style {
            return RegisteredStyle { style_id, atomic: true, overwrote: false, builtin: true };
        }

        // This is a custom style, turn into expanded atomic styles
        //let style_entries = self.expand_raw_styles(entries);

        // We add or overwrite the style
        let overwrote = self.definitions.insert(style_id, entries).is_some();
        let builtin = style_id < self.first_custom_style;

        RegisteredStyle {
            style_id,
            atomic: false,
            overwrote,
            builtin,
        }
    }

    pub fn register_raw_style(&mut self, style_name: &str, entries: Vec<RawStyle>) -> RegisteredStyle {
        let atomics = self.expand_raw_styles(&entries);
        self.register_style(style_name, atomics)
    }

    /// Expands a list of raw style entries like "bold", "italic", "fontSize" with value "3", "customStyle"
    /// Into a list of atomic styles like fontWeight: bold, fontSize: 3.0, etc
    pub fn expand_raw_styles(&mut self, entries: &Vec<RawStyle>) -> Vec<AtomicStyle> {
        let mut atomic_set : HashSet<StyleId> = HashSet::new();
        let mut atomic_styles = vec![];

        // We expand the raw style entries into definitions
        // Atomic styles are inserted with parsed values
        // Defined styles are expanded from already expanded styles
        // We reverse the order so the later styles get precedence
        entries.into_iter().rev().for_each(|raw| {
            let id = self.intern_name(raw.name);

            if id <= self.first_style {
                // Atomic style, we parse and add
                let parser = &self.builtins[id].parser;

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