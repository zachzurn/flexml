use crate::styles::builtin::text::{TEXT_COLOR, TEXT_SIZE};
use super::style::StyleValue::{NegativeNumber, Number, Percent, Empty, Invalid, Color};
use super::style::{AtomicStyle, PercentFloat, RawStyle, StyleId, StyleValueParser, RGBA};
use super::style_registry::{StyleRegistry};

// Helper to get a sorted vec of atomic style entries for comparison
// This helps make comparisons stable regardless of HashMap iteration order.
fn get_sorted_atomic_entries(registry: &StyleRegistry, alias_id: StyleId) -> Vec<AtomicStyle> {
    let mut entries = registry.get_definition(alias_id)
        .expect("Definition should exist for this ID")
        .clone();
    entries.sort_by_key(|e| e.id);
    entries
}

#[test]
fn test_style_value_color_parser(){
    let color_parser = StyleValueParser::ColorParser;

    let tests = vec![

        ("#00", Color(RGBA{ r: 0, g: 0, b: 0, a: 255 })),
        ("#FF", Color(RGBA{ r: 255, g: 255, b: 255, a: 255 })),
        ("#AB", Color(RGBA{ r: 171, g: 171, b: 171, a: 255 })),
        ("#cc", Color(RGBA{ r: 204, g: 204, b: 204, a: 255 })), // Case-insensitivity

        ("#F00", Color(RGBA{ r: 255, g: 0, b: 0, a: 255 })),
        ("#0F0", Color(RGBA{ r: 0, g: 255, b: 0, a: 255 })),
        ("#00F", Color(RGBA{ r: 0, g: 0, b: 255, a: 255 })),
        ("#ABC", Color(RGBA{ r: 170, g: 187, b: 204, a: 255 })),
        ("#fff", Color(RGBA{ r: 255, g: 255, b: 255, a: 255 })), // Case-insensitivity

        ("#F008", Color(RGBA{ r: 255, g: 0, b: 0, a: 136 })), // Red with ~50% alpha
        ("#0F04", Color(RGBA{ r: 0, g: 255, b: 0, a: 68 })),  // Green with ~25% alpha
        ("#1234", Color(RGBA{ r: 17, g: 34, b: 51, a: 68 })),
        ("#abcd", Color(RGBA{ r: 170, g: 187, b: 204, a: 221 })), // Case-insensitivity

        ("#FF0000", Color(RGBA{ r: 255, g: 0, b: 0, a: 255 })),
        ("#00FF00", Color(RGBA{ r: 0, g: 255, b: 0, a: 255 })),
        ("#0000FF", Color(RGBA{ r: 0, g: 0, b: 255, a: 255 })),
        ("#123456", Color(RGBA{ r: 18, g: 52, b: 86, a: 255 })),
        ("#FFFFFF", Color(RGBA{ r: 255, g: 255, b: 255, a: 255 })),
        ("#000000", Color(RGBA{ r: 0, g: 0, b: 0, a: 255 })),
        ("#abcdef", Color(RGBA{ r: 171, g: 205, b: 239, a: 255 })), // Case-insensitivity

        ("#FF000080", Color(RGBA{ r: 255, g: 0, b: 0, a: 128 })), // Red with 50% alpha
        ("#00000000", Color(RGBA{ r: 0, g: 0, b: 0, a: 0 })),     // Fully transparent black
        ("#000000FF", Color(RGBA{ r: 0, g: 0, b: 0, a: 255 })),   // Opaque black
        ("#12345678", Color(RGBA{ r: 18, g: 52, b: 86, a: 120 })),
        ("#ABCDEF01", Color(RGBA{ r: 171, g: 205, b: 239, a: 1 })),
        ("#abcdef99", Color(RGBA{ r: 171, g: 205, b: 239, a: 153 })), // Case-insensitivity
    ];

    for (input, value) in tests {
        assert_eq!(
            color_parser.parse(input),
            value,
            "Input {}", input
        )
    }

}

#[test]
fn test_style_value_color_parser_invalid(){
    let color_parser = StyleValueParser::ColorParser;

    let error_test = vec![
        "not_a_hex",
        "#F",
        "#F",
        "#F0000",
        "#ABCDE",
        "#123456789",
        "#GG",
        "#F0G",
        "#F00G",
        "#A_B"
    ];

    for name in error_test {
        let result = color_parser.parse(name);
        match result {
            Invalid(_,_) => {},
            _ => panic!("Expected Invalid got {:?} from input {}", result, name)
        }
    }

}

#[test]
fn test_style_value_number_parser(){
    let number_parser = StyleValueParser::NumberParser;

    let tests = vec![
        ("0", Number(0)),
        ("1", Number(1)),
        ("100", Number(100)),
        ("30000", Number(30000)),
        ("-1", NegativeNumber(1)),
        ("-1000", NegativeNumber(1000)),
        ("100%", Percent(PercentFloat::new(100.0f32))),
        ("30.33", Number(30)),
        ("30.33%", Percent(PercentFloat::new(30.33f32))),
        ("", Empty),
        ("-200", NegativeNumber(200)),
        ("-2000px", NegativeNumber(2000)),
        ("235em", Number(235)),
        ("0em", Number(0)),
        ("-90.45", NegativeNumber(90)),
    ];

    for (name, value) in tests {
        assert_eq!(
            number_parser.parse(name),
            value,
        )
    }

}

#[test]
fn test_style_value_number_parser_invalid(){
    let number_parser = StyleValueParser::NumberParser;

    let error_test = vec![
        "!23",
        "a200",
        "b",
        "bold",
        "_23",
        "  %",
        "  a 23"
    ];

    for name in error_test {
        let result = number_parser.parse(name);
        match result {
            Invalid(_,_) => {},
            _ => panic!("Expected Invalid StyleValue got {:?} from input {}", result, name)
        }
    }
}

#[test]
fn test_intern_and_resolve() {
    let mut registry = StyleRegistry::with_builtins();
    let id1 = registry.intern_name("test1");
    let id2 = registry.intern_name("test2");
    let id1_1 = registry.intern_name("Test1");
    let id1_2 = registry.intern_name("test1");

    assert_eq!(id1, id1_2, "Should be the same id for test1");
    assert_ne!(id1, id1_1, "Case is different, so it should be its own name");

    assert_eq!(registry.resolve_name(id1), Some("test1"), "Incorrect resolved name, should be 'test1'");
    assert_eq!(registry.resolve_name(id2), Some("test2"), "Incorrect resolved name, should be 'test2'");
    assert_eq!(registry.resolve_name(30000), None, "Non-existent ID should resolve to None");
}


#[test]
fn test_register_style_basic_alias_definition() {
    let mut registry = StyleRegistry::with_builtins();

    let alias_name = "smallRedText";
    let registered_style = registry.register_raw_style(alias_name, vec![
        RawStyle::new("fontSize", Some("12")),
        RawStyle::new("color", Some("#FF0000")),
    ]);

    let alias_id = registered_style.style_id;
    assert_eq!(registry.resolve_name(alias_id), Some(alias_name));
    assert!(!registered_style.atomic, "Should not be atomic");
    assert!(!registered_style.overwrote, "Should not have overwritten anything new");
}

#[test]
fn test_register_style_overwriting_existing_alias() {
    let mut registry = StyleRegistry::with_builtins();

    let alias_name = "myCustomStyle";
    let alias_id = registry.intern_name(alias_name);

    let reg1 = registry.register_raw_style(alias_name, vec![
        RawStyle::new("fontSize", Some("10")),
        RawStyle::new("color", Some("#FF00FF")),
    ]);

    assert_eq!(reg1.style_id, alias_id);
    assert!(!reg1.overwrote);
    assert!(!reg1.atomic, "myCustomStyle should not be atomic");

    // Second registration, overwriting
    let reg2 = registry.register_raw_style(alias_name, vec![
        RawStyle::new("fontSize", Some("14.0")),
        RawStyle::new("color", Some("blue")),
    ]);

    assert_eq!(reg2.style_id, alias_id);
    assert!(reg2.overwrote, "Should report as overwriting an existing alias");
}

#[test]
fn test_register_style_precedence_in_raw_definitions() {
    let mut registry = StyleRegistry::with_builtins();

    let atomic_color = registry.intern_name(TEXT_COLOR.name);
    let atomic_font_size = registry.intern_name(TEXT_SIZE.name);

    let registered_style = registry.register_raw_style("precedenceTest", vec![
        RawStyle::new(TEXT_COLOR.name, Some("#FF0000")),   // First declaration
        RawStyle::new(TEXT_SIZE.name, Some("10")),
        RawStyle::new(TEXT_COLOR.name, Some("#0000FF")),  // Later declaration for color should win
        RawStyle::new(TEXT_SIZE.name, Some("12")),   // Later declaration for size should win
        RawStyle::new(TEXT_COLOR.name, Some("#00FF00")), // Even later declaration for color should win
    ]);

    let alias_id = registered_style.style_id;
    let definition = get_sorted_atomic_entries(&registry, alias_id);
    assert_eq!(definition.len(), 2);
    assert_eq!(definition[0], AtomicStyle { id: atomic_color, value: Color(RGBA::green()) }); // 2 = green
    assert_eq!(definition[1], AtomicStyle { id: atomic_font_size, value: Number(12) });
}

#[test]
fn test_register_style_nested_alias_expansion() {
    let mut registry = StyleRegistry::with_builtins();

    let atomic_color = registry.intern_name(TEXT_COLOR.name);
    let atomic_font_size = registry.intern_name(TEXT_SIZE.name);

    // Register a base alias "redBox"
    let red_box_id = registry.register_raw_style("redBox", vec![
        RawStyle::new(TEXT_COLOR.name, Some("#FF0000")),
        RawStyle::new(TEXT_SIZE.name, Some("23")),
    ]).style_id;

    let red_box_definition = registry.get_definition(red_box_id).unwrap();

    println!("{:?}", red_box_definition);

    // should be [ color, font size ]
    assert_eq!(red_box_definition[0], AtomicStyle { id: atomic_color, value: Color(RGBA::red()) });
    assert_eq!(red_box_definition[1], AtomicStyle { id: atomic_font_size, value: Number(23) });

    // Register a new alias "fancyBox" that uses "redBox" and adds properties
    let fancy_box_id = registry.register_raw_style("fancyBox", vec![
        RawStyle::new("redBox", None),
        RawStyle::new(TEXT_COLOR.name, Some("#0000FF")),
    ]).style_id;

    // order should be preserved [ border, color ]
    let fancy_box_definition = registry.get_definition(fancy_box_id).unwrap();

    println!("{:?}", fancy_box_definition);

    assert_eq!(fancy_box_definition[0], AtomicStyle { id: atomic_font_size, value: Number(23) });
    assert_eq!(fancy_box_definition[1], AtomicStyle { id: atomic_color, value: Color(RGBA::blue()) });
}