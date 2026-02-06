use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FontFace {
    pub path: PathBuf,
    pub weight: u16,
    pub italic: bool,
}

#[derive(Debug)]
pub struct FontFamily {
    pub name: String,
    pub faces: Vec<FontFace>,
}

// Ordered from most specific to least specific to avoid substring conflicts
pub static WEIGHT_MATCHES: &[(&[&str], u16)] = &[
    (&["ExtraBlack", "1000"], 1000),
    (&["Black", "900"], 900),
    (&["ExtraBold", "Heavy", "800"], 800),
    (&["Bold", "700"], 700),
    (&["SemiBold", "DemiBold", "600"], 600),
    (&["Medium", "500"], 500),
    (&["Regular", "Normal", "Book", "400"], 400),
    (&["Light", "300"], 300),
    (&["ExtraLight", "UltraLight", "200"], 200),
    (&["Thin", "Hairline", "100"], 100),
];

pub static ITALIC_MATCHES: &[&str] = &["Italic", "Oblique", "Inclined"];

pub static FONT_EXTENSIONS: &[&str] = &[".ttf", ".otf", ".woff", ".woff2", ".ttc"];

/// Expand a path with at most one star
fn expand_single_star(pattern: &Path) -> Vec<PathBuf> {
    let s = pattern.to_string_lossy();
    if !s.contains('*') {
        return vec![pattern.to_path_buf()];
    }

    let parts: Vec<&str> = s.splitn(2, '*').collect();
    let prefix = parts[0];
    let suffix = parts[1];

    let dir = Path::new(prefix).parent().unwrap_or_else(|| Path::new("."));
    let prefix_file = Path::new(prefix)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let mut matches = vec![];
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.starts_with(&prefix_file) && file_name.ends_with(suffix) {
                // Only include valid font files
                if is_font_file(&entry.path()) {
                    matches.push(entry.path());
                }
            }
        }
    }

    matches.sort(); // Ensure consistent ordering
    matches
}

/// Check if a path is a font file based on extension
fn is_font_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            let ext_lower = ext.to_lowercase();
            FONT_EXTENSIONS.iter().any(|&font_ext| {
                font_ext.trim_start_matches('.').eq_ignore_ascii_case(&ext_lower)
            })
        })
        .unwrap_or(false)
}

/// Parse weight and italic from filename
/// `single_file_mode`: true if pattern points to a specific file (no wildcard)
fn parse_weight_and_italic(filename: &str, single_file_mode: bool) -> (u16, bool) {
    if single_file_mode {
        return (400, false); // Assume normal weight for direct file references
    }

    // Remove file extension for cleaner matching
    let stem = Path::new(filename)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase();

    // Match weights from most specific to least specific
    let mut weight = 400; // Default to Regular
    for (aliases, weight_value) in WEIGHT_MATCHES {
        if aliases.iter().any(|&alias| {
            let alias_lower = alias.to_lowercase();
            // Match whole words or hyphenated segments to avoid partial matches
            stem.split(&['-', '_', ' '][..])
                .any(|part| part == alias_lower || part.contains(&alias_lower))
        }) {
            weight = *weight_value;
            break;
        }
    }

    let italic = ITALIC_MATCHES
        .iter()
        .any(|&i| stem.contains(&i.to_lowercase()));

    (weight, italic)
}

/// Extract family name from pattern
fn extract_family_name(pattern: &Path) -> String {
    let s = pattern.to_string_lossy();

    // If no wildcard, use the file stem
    if !s.contains('*') {
        return pattern
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
    }

    // Extract the portion before the wildcard
    let before_star = s.split('*').next().unwrap_or("");

    // Get the last path component (filename part)
    Path::new(before_star)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

/// Gather fonts into families
pub fn gather_fonts(pattern: &Path) -> FontFamily {
    let single_file_mode = !pattern.to_string_lossy().contains('*');
    let paths = expand_single_star(pattern);

    let family_name = extract_family_name(pattern);

    let mut faces = vec![];

    for path in paths {
        if !path.is_file() {
            continue;
        }

        let filename = path.file_name().unwrap().to_string_lossy();
        let (weight, italic) = parse_weight_and_italic(&filename, single_file_mode);

        faces.push(FontFace {
            path,
            weight,
            italic,
        });
    }

    FontFamily {
        name: family_name,
        faces,
    }
}

pub(super) fn canonicalize(base: &Path, path: &PathBuf) -> Option<PathBuf> {
    let joined = if path.is_absolute() {
        path.clone()
    } else {
        base.join(path)
    };

    std::fs::canonicalize(&joined).ok()
}