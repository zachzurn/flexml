use std::collections::{HashMap, HashSet};
use url::Url;
use crate::strings::{Chars, ValueErrors, ValueHelp};
use crate::styles::builtin::{BuiltInStyle, DEFAULT_BUILTINS};
use crate::styles::context::StyleContext;
use crate::styles::style::StyleValue::{Font, Image};
use super::style::{AtomicStyle, FileId, FontId, ImageId, RawStyle, StyleId, StyleValue, StyleValueParser, UrlType};

/// Style Registry is the central place
/// to register atomic styles (Things like fontSize, fontWeight, etc)
/// and to register built in styles (Things like bold, italic, flex)
///
/// It also allows overwriting styles while protecting atomic styles
/// from being overwritten
pub struct StyleRegistry {
    names: Vec<String>,
    names_map: HashMap<String, StyleId>,

    paths: Vec<String>,
    paths_map: HashMap<String, StyleId>,

    definitions: HashMap<StyleId, Vec<AtomicStyle>>,
    forwarders: HashMap<StyleId, Vec<StyleId>>,
    first_style: usize,
    first_custom_style: usize,
    builtins_registered: bool,
    builtins: Vec<&'static BuiltInStyle>,

    base_path: Url,
    image_urls: HashMap<ImageId, Url>,
    font_urls: HashMap<FontId, Url>,
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

            paths: vec![],
            paths_map: HashMap::new(),

            definitions: HashMap::new(),
            forwarders: HashMap::new(),
            builtins: Vec::new(),
            first_style: 0,
            first_custom_style: 0,
            builtins_registered: false,

            font_urls: HashMap::new(),
            image_urls: HashMap::new(),
            base_path: Url::parse("flexml://").expect("Default base url is invalid."),
        }
    }

    pub fn set_file_base_path(&mut self, base_path: &Url) {
        self.base_path = base_path.clone();
    }

    pub fn with_builtins() -> StyleRegistry {
        let mut registry = Self::new();
        registry.register_builtins(DEFAULT_BUILTINS);
        registry
    }

    pub fn print_atomics(&self) {
        for builtin in &self.builtins {
            let description = match builtin.parser {
                StyleValueParser::PositiveNumberParser => {
                    "positive number (u16 or percent float)"
                },
                StyleValueParser::FloatParser => {
                    "float value f32"
                },
                StyleValueParser::ColorParser => {
                    "color RGBA struct"
                },
                StyleValueParser::MatchParser(matches) => {
                    &format!("One of: {}", matches.join(", "))
                },
                StyleValueParser::NumberParser => {
                    "number (i32 or percent float)"
                },
                StyleValueParser::MatchOrFloatParser(matches) => {
                    &format!("Float value or one of: {}", matches.join(", "))
                },
                StyleValueParser::UrlParser(kind) => {
                    match kind {
                        UrlType::Image => "image url",
                        UrlType::Font => "font url",
                    }
                },
            };

            println!("{}: {}", builtin.name, description)
        }
    }

    pub fn resolve_style(&self, parent: &StyleContext, styles: &[AtomicStyle]) -> StyleContext {
        let mut context = StyleContext::default();

        // First set the styles
        for atomic in styles {
            &(self.builtins[atomic.id].apply_style)(&atomic.value, &mut context);
        }
        
        // Cascade styles from the parent that were not explicitly set above
        context.cascade_from(parent);

        context
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

    /// Interns a string name and returns its StyleId.
    /// If the name already exists, returns its existing StyleId.
    fn intern_path(&mut self, path: &str) -> FileId {
        if let Some(&id) = self.paths_map.get(path) {
            return id;
        }
        let id = self.paths.len();
        self.names.push(path.to_string());
        self.names_map.insert(path.to_string(), id);
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
        if self.is_atomic(&style_id) {
            return RegisteredStyle { style_id, atomic: true, overwrote: false, builtin: true };
        }

        // We add or overwrite the style
        let overwrote = self.definitions.insert(style_id, entries).is_some();

        // Store forwarding info
        self.forwarders.insert(style_id, forwards);

        let builtin = !self.is_custom(&style_id);

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
                    atomic_styles.push(AtomicStyle { id, value: self.transform_value(value) });
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
                        let fwd = forwarders.clone();
                        for (i, value) in raw_value.split(Chars::FORWARD).enumerate() {
                            if let Some(forwards_to) = fwd.get(i) {
                                if self.is_atomic(forwards_to) && !atomic_set.contains(forwards_to) {
                                    let forward_value = self.builtins[*forwards_to].parser.parse(value);
                                    atomic_set.insert(*forwards_to);
                                    atomic_styles.push(AtomicStyle { id: *forwards_to, value: self.transform_value(forward_value) });
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

    /// File type style values need to hold a file reference, so we handle
    /// interning and storing here
    fn transform_value(&mut self, value: StyleValue) -> StyleValue {
        match value {
            StyleValue::FontUrl(font_url) => {
                let file_id = self.intern_path(&font_url);

                if let Ok(url) = self.base_path.join(&font_url){
                    self.font_urls.insert(file_id, url);
                    Font(file_id)
                } else {
                    StyleValue::Invalid(ValueErrors::URL, ValueHelp::URL)
                }
            },
            StyleValue::ImageUrl(image_url) => {
                let file_id = self.intern_path(&image_url);

                if let Ok(url) = self.base_path.join(&image_url){
                    self.image_urls.insert(file_id, url);
                    Image(file_id)
                } else {
                    StyleValue::Invalid(ValueErrors::URL, ValueHelp::URL)
                }
            },
            _ => value
        }
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