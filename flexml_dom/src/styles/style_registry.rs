use super::style::{AtomicStyle, PathId, PathType, RawStyle, StyleId, StyleValue, StyleValueParser};
use crate::strings::{Chars, ValueErrors, ValueHelp};
use crate::styles::builtin::{BuiltInStyle, DEFAULT_BUILTINS, ROOT_STYLE_NAME};
use crate::styles::context::StyleContext;
use crate::styles::style::StyleValue::{Directory, Font, Image};
use std::collections::{HashMap, HashSet};
use std::path::{Component, Path, PathBuf};
use crate::styles::files::{gather_fonts, FontFamily};

#[derive(Debug)]
pub enum PathValidation {
    Missing,
    File,
    Dir,
}

/// Style Registry is the central place
/// to register atomic styles (Things like fontSize, fontWeight, etc.)
/// and to register built in styles (Things like bold, italic, flex)
///
/// It also allows overwriting styles while protecting atomic styles
/// from being overwritten
pub struct StyleRegistry {
    names: Vec<String>,
    names_map: HashMap<String, StyleId>,

    paths: Vec<PathBuf>,
    paths_validation: Vec<PathValidation>,
    paths_map: HashMap<PathBuf, PathId>,

    font_families: HashMap<PathId, FontFamily>,

    definitions: HashMap<StyleId, Vec<AtomicStyle>>,
    forwarders: HashMap<StyleId, Vec<StyleId>>,

    first_style: usize,
    first_custom_style: usize,

    builtins_registered: bool,
    builtins: Vec<&'static BuiltInStyle>,
    base_path: PathBuf,
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
            paths_validation: vec![],
            paths_map: HashMap::new(),

            font_families: Default::default(),

            definitions: HashMap::new(),
            forwarders: HashMap::new(),
            builtins: Vec::new(),
            first_style: 0,
            first_custom_style: 0,
            builtins_registered: false,
            base_path: PathBuf::from("/"),
        }
    }

    pub fn set_base_path(&mut self, path: &PathBuf) {
        self.base_path = path.clone();
    }

    pub fn with_builtins() -> StyleRegistry {
        let mut registry = Self::new();
        registry.register_builtins(DEFAULT_BUILTINS);
        registry
    }

    pub fn debug_style_definition(&self, id: StyleId) -> String {
        let definition = self.get_definition(id);

        let mut info = vec![self.resolve_name(id).unwrap_or("No Style Defined = ").to_string()];

        if let Some(atomics) = definition {
            for atomic in atomics {
                info.push(format!("{:?}: {:?}", self.resolve_name(atomic.id).unwrap_or("No atomic"), atomic.value));
            }

            info.join(", ")
        } else {
            "".to_string()
        }
    }

    pub fn display_style_definition(&self, id: StyleId) -> String {
        let definition = self.get_definition(id);

        let style_name = self.resolve_name(id).unwrap_or("No Style Defined = ").to_string();

        if let Some(atomics) = definition {
            format!("{} = {}", style_name, self.display_atomics(atomics))
        } else {
            "".to_string()
        }
    }

    pub fn debug_atomics(&self, atomics: &Vec<AtomicStyle>) -> String {
        let mut info = vec![];
        for atomic in atomics {
            info.push(format!("{}: {:?}", self.resolve_name(atomic.id).unwrap_or("No atomic"), atomic.value));
        }
        info.join(", ")
    }

    pub fn display_atomics(&self, atomics: &Vec<AtomicStyle>) -> String {
        let mut info = vec![];
        for atomic in atomics {
            match atomic.value {
                Font(path_id) => {
                    let family = &self.font_families.get(&path_id);

                    if let Some(family) = *family {
                        info.push(format!("{}:{:?}", self.resolve_name(atomic.id).unwrap_or("No name!"), family));
                    } else {
                        info.push(format!("{}:{}", self.resolve_name(atomic.id).unwrap_or("No name!"), "No family!"));
                    }
                },
                _ => {
                    info.push(format!("{}:{}", self.resolve_name(atomic.id).unwrap_or("No name!"), atomic.value));
                }
            }
        }
        info.join(" • ")
    }

    #[allow(dead_code)]
    pub fn print_atomics(&self) {
        for builtin in &self.builtins {
            let description = match builtin.parser {
                StyleValueParser::PositiveNumber => {
                    "positive number (u16 or percent float)"
                }
                StyleValueParser::Float => {
                    "float value f32"
                }
                StyleValueParser::Color => {
                    "color RGBA struct"
                }
                StyleValueParser::Match(matches) => {
                    &format!("One of: {}", matches.join(", "))
                },
                StyleValueParser::Number => {
                    "number (i32 or percent float)"
                }
                StyleValueParser::MatchOrFloat(matches) => {
                    &format!("Float value or one of: {}", matches.join(", "))
                },
                StyleValueParser::Path(kind) => {
                    match kind {
                        PathType::Image => "Image",
                        PathType::Font => "Font",
                        PathType::Directory => "Folder",
                    }
                },
            };

            println!("{}: {}", builtin.name, description)
        }
    }

    /// Apply any root styles that were defined
    /// This is where we ensure the root style  has concrete resolved
    /// dimensions that are necessary for layout
    ///
    /// Users can set properties on the root style and
    /// some properties would break page layout if not resolved
    pub fn resolve_root_style(&self, root: &mut StyleContext) {
        // Mark the root style as being the root style
        // This way atomic builtins that only appy to root
        // will apply.
        root.set_as_root();

        // ROOT_STYLE_NAME is "PAGE". Users can set page specific settings in a style definition
        if let Some(styles) = self.names_map.get(ROOT_STYLE_NAME)
            .and_then(|&id| self.definitions.get(&id)) {
            for atomic in styles {
                (self.builtins[atomic.id].apply_style)(&atomic.value, root);
            }
        }

        // Prepares the root style for cascading
        root.prepare_root();
    }

    /// Resolve local atomic styles and cascade
    /// styles from the parent style
    pub fn resolve_style(&self, parent: &StyleContext, styles: &[AtomicStyle]) -> StyleContext {

        let mut context = StyleContext::default();

        // First set the styles
        for atomic in styles {
            (self.builtins[atomic.id].apply_style)(&atomic.value, &mut context);
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

    /// Interns a string path and returns its PathId.
    /// If the path already exists, returns its existing PathId.
    /// These paths are normalized so that paths that end in the same
    /// place all have the same url.
    ///
    /// Paths are also validated as existing at time of interning.
    pub fn intern_path(&mut self, path: &PathBuf) -> PathId {
        let abs_path = self.base_path.join(path);
        let normalized_path = self.normalize_path(&abs_path);

        if let Some(&id) = self.paths_map.get(&normalized_path) {
            return id;
        }

        let id = self.paths.len();
        let exists = normalized_path.try_exists().unwrap_or(false);
        let is_dir = normalized_path.is_dir();
        let validation = if exists { if is_dir { PathValidation::Dir } else { PathValidation::File } } else { PathValidation::Missing };
        self.paths_validation.push(validation);
        self.paths.push(normalized_path.clone());
        self.paths_map.insert(normalized_path, id);
        id
    }

    /// We do non-filesystem canonicalization
    /// Which can create issues when referencing
    /// symbolic links.
    fn normalize_path(&self, path: &Path) -> PathBuf {
        let mut result = PathBuf::new();

        for component in path.components() {
            match component {
                Component::CurDir => {
                    // Skip "." components
                }
                Component::ParentDir => {
                    // Go up one directory by popping the last component
                    result.pop();
                }
                _ => {
                    result.push(component.as_os_str());
                }
            }
        }

        result
    }

    /// Resolves a StyleId back to its string name.
    /// Useful for debugging
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn register_raw_style(&mut self, style_name: &str, entries: Vec<RawStyle>) -> RegisteredStyle {
        let (atomics, forwards) = self.expand_raw_styles(&entries);
        self.register_style(style_name, atomics, forwards)
    }

    /// Expands a list of raw style entries like "bold", "italic", "fontSize" with value "3", "customStyle"
    /// Into a list of atomic styles like fontWeight: bold, fontSize: 3.0, etc
    pub fn expand_raw_styles(&mut self, entries: &Vec<RawStyle>) -> (Vec<AtomicStyle>, Vec<StyleId>) {
        // This set lets us prioritize merged styles, so that later defined styles take precedence over
        // previously defined styles. Later down we iterate style expansions in reverse.
        let mut atomic_set : HashSet<StyleId> = HashSet::new();

        // A list of style ids that are forwarded, essentially making them parameters of the style
        // If you define a style with a forward {myStyle = >fontWeight + >color: #FF0000 }
        // Forwards would contain the style ids for fontWeight and color
        let mut forwards : Vec<StyleId> = vec![];

        // A list of atomic styles that this set of raw styles has been expanded to
        let mut atomic_styles = vec![];

        // We expand the raw style entries into definitions
        // Atomic styles are inserted with parsed values
        // Defined styles are expanded from already expanded styles
        // We reverse the order so the later styles get precedence
        entries.iter().rev().for_each(|raw| {
            // Names starting with > are considered forwarded or proxied
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

                // Gather atomic styles and set
                if let Some(styles) = existing_style {
                    styles.iter().rev().for_each(|entry| {

                        // Insert if it hasn't already been defined
                        if !atomic_set.contains(&entry.id) {
                            atomic_set.insert(entry.id);
                            atomic_styles.push(entry.clone());
                        }
                    })
                }

                // When a value is passed to a style definition it is forwarded
                // to its own atomic entries. values are separated with ">"
                // forwarded values are raw unparsed strings
                if let (Some(raw_value), Some(fwd)) = (raw.value, self.forwarders.get(&id).cloned()) {
                    for (i, value) in raw_value.split(Chars::FORWARD).enumerate() {
                        if let Some(&forwards_to) = fwd.get(i)
                            && self.is_atomic(&forwards_to)
                        {
                            let forward_value = self.builtins[forwards_to].parser.parse(value);

                            // If it's already in the set, we need to UPDATE it, not skip it
                            if atomic_set.contains(&forwards_to) {
                                // Find and update the existing atomic style
                                if let Some(existing) = atomic_styles.iter_mut().find(|a| a.id == forwards_to) {
                                    existing.value = self.transform_value(forward_value);
                                }
                            } else {
                                // Add new
                                atomic_set.insert(forwards_to);
                                atomic_styles.push(AtomicStyle {
                                    id: forwards_to,
                                    value: self.transform_value(forward_value),
                                });
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
    /// interning and storing here.
    ///
    /// Fonts, Images and Directory paths are normalized and interned.
    ///
    /// Style values need to be quick to copy, thus all of our
    /// file based style values hold path_ids of some kind or another.
    fn transform_value(&mut self, value: StyleValue) -> StyleValue {
        match value {
            StyleValue::FontPath(path) => {
                let path_id = self.intern_path(&path);
                let font_family = self.font_families.get(&path_id);

                let normalized = &self.paths[path_id];
                eprintln!("   Base path: {}", self.base_path.display());
                eprintln!("   Normalized path: {}", normalized.display());

                let font_count = match self.font_families.get(&path_id) {
                    Some(family) => {
                        family.faces.len()
                    }
                    None => {
                        let normalized = &self.paths[path_id];
                        let family = gather_fonts(normalized);
                        let font_count = family.faces.len();
                        self.font_families.insert(path_id, family);
                        font_count
                    }
                };

                if font_count > 0 {
                    Font(path_id)
                } else {
                    StyleValue::Invalid(ValueErrors::FONT_EMPTY, ValueHelp::FONT_EMPTY)
                }
            }

            StyleValue::ImagePath(path) => {
                let image_path_id = self.intern_path(&path);
                let validation = &self.paths_validation[image_path_id];

                if matches!(validation, PathValidation::File) {
                    Image(image_path_id)
                } else {
                    StyleValue::Invalid(ValueErrors::FILE, ValueHelp::FILE)
                }
            }

            StyleValue::DirectoryPath(path) => {
                let dir_path_id = self.intern_path(&path);
                let validation = &self.paths_validation[dir_path_id];

                if matches!(validation, PathValidation::Dir) {
                    Directory(dir_path_id)
                } else {
                    StyleValue::Invalid(ValueErrors::DIRECTORY, ValueHelp::DIRECTORY)
                }
            }
            _ => value
        }
    }

    /// Retrieves the definition for a given alias StyleId.
    /// Returns `Some(&Vec<StyleId>)` if the ID corresponds to a registered alias, `None` otherwise.
    #[allow(dead_code)]
    pub fn get_definition(&self, id: StyleId) -> Option<&Vec<AtomicStyle>> {
        self.definitions.get(&id)
    }

    /// Checks if the style is a base atomic style
    #[inline(always)]
    pub fn is_atomic(&self, id: &StyleId) -> bool {
        id < &self.first_style
    }

    /// Checks if the style id is a builtin style definition (not an atomic)
    #[inline(always)]
    pub fn is_custom(&self, id: &StyleId) -> bool {
        id >= &self.first_custom_style
    }
}