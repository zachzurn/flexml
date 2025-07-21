use std::collections::{HashMap, HashSet};
use crate::strings::Chars;
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
    forwarders: HashMap<StyleId, Vec<StyleId>>,
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
            forwarders: HashMap::new(),
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

            if let StyleValue::Forward = style_value {
                // Set up the empty style definition with a forwarder
                self.definitions.insert(style_id, vec![]);
                self.forwarders.insert(style_id, vec![atomic_style_id]);
            } else {
                // Set up the style definition with an atomic style
                self.definitions.insert(style_id, vec![AtomicStyle{ id: atomic_style_id, value: style_value.clone() }]);
            }
        }

        // The next style registered after this is the first custom style
        self.first_custom_style = self.names.len()
    }

    /// Registers or updates a style definition. This always returns a StyleId
    /// and an indication of whether the returned style definition is atomic.
    pub fn register_style(&mut self, style_name: &str, entries: Vec<AtomicStyle>, forwards: Vec<StyleId>) -> RegisteredStyle {
        let style_id = self.intern_name(style_name);

        // This is an atomic style, cannot be overwritten, we return as such
        if style_id < self.first_style {
            return RegisteredStyle { style_id, atomic: true, overwrote: false, builtin: true };
        }

        // We add or overwrite the style
        let overwrote = self.definitions.insert(style_id, entries).is_some();

        // Store forwarding info
        self.forwarders.insert(style_id, forwards);

        let builtin = style_id < self.first_custom_style;

        RegisteredStyle {
            style_id,
            atomic: false,
            overwrote,
            builtin,
        }
    }

    pub fn register_raw_style(&mut self, style_name: &str, entries: Vec<RawStyle>) -> RegisteredStyle {
        let (atomics, forwards) = self.expand_raw_styles(&entries);
        self.register_style(style_name, atomics, forwards)
    }

    /// Expands a list of raw style entries like "bold", "italic", "fontSize" with value "3", "customStyle"
    /// Into a list of atomic styles like fontWeight: bold, fontSize: 3.0, etc
    pub fn expand_raw_styles(&mut self, entries: &Vec<RawStyle>) -> (Vec<AtomicStyle>, Vec<StyleId>) {
        let mut atomic_set : HashSet<StyleId> = HashSet::new();
        let mut forwards : Vec<StyleId> = vec![];
        let mut atomic_styles = vec![];

        // We expand the raw style entries into definitions
        // Atomic styles are inserted with parsed values
        // Defined styles are expanded from already expanded styles
        // We reverse the order so the later styles get precedence
        entries.into_iter().rev().for_each(|raw| {
            // Names starting with > are considered forwarded
            let forward = raw.name.starts_with(Chars::FORWARD);
            let clean_name = if forward { raw.name.trim_start_matches(Chars::FORWARD) } else { raw.name };
            let id = self.intern_name(clean_name);

            // Everything after this deals with the style value
            if id < self.first_style {
                // Forwards are allowed only on atomics
                if forward && !forwards.contains(&id) { forwards.push(id); }

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

                if let Some(styles) = existing_style {
                    styles.into_iter().rev().for_each(|entry| {

                        // Insert if it hasn't already been defined
                        if !atomic_set.contains(&entry.id) {
                            atomic_set.insert(entry.id);
                            atomic_styles.push(entry.clone());
                        }
                    })
                }

                // When a value is passed to a style definition it is forwarded
                // to its own atomic entries. values are separated with ">"
                if let Some(raw_value) = raw.value {
                    if let Some(forwarders) = self.forwarders.get(&id) {
                        for (i, value) in raw_value.split(Chars::FORWARD).enumerate() {
                            if let Some(forwards_to) = forwarders.get(i) {
                                if self.is_atomic(forwards_to) && !atomic_set.contains(forwards_to) {
                                    let forward_value = self.builtins[*forwards_to].parser.parse(raw_value);
                                    atomic_set.insert(*forwards_to);
                                    atomic_styles.push(AtomicStyle { id: *forwards_to, value: forward_value });
                                }
                            }
                        }
                    }
                }

            }
        });

        atomic_styles.reverse();
        forwards.reverse();
        (atomic_styles, forwards)
    }

    /// Retrieves the definition for a given alias StyleId.
    /// Returns `Some(&Vec<StyleId>)` if the ID corresponds to a registered alias, `None` otherwise.
    pub fn get_definition(&self, id: StyleId) -> Option<&Vec<AtomicStyle>> {
        self.definitions.get(&id)
    }

    pub fn is_atomic(&self, id: &StyleId) -> bool {
        return id < &self.first_style;
    }

    /// Is this style id a builtin
    pub fn is_custom(&self, id: &StyleId) -> bool {
        return id >= &self.first_custom_style;
    }
}