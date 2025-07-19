use super::style::StyleValue::{NegativeNumber, Number, Percent, Empty, Invalid, Color};
use super::style::{AtomicStyle, PercentFloat, RawStyle, StyleId, StyleValue, StyleValueParser, RGBA};
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

        ("  #FF0000 ", Color(RGBA{ r: 255, g: 0, b: 0, a: 255 })),
        ("\t#F00\n", Color(RGBA{ r: 255, g: 0, b: 0, a: 255 })),
        ("  #00000000  ", Color(RGBA{ r: 0, g: 0, b: 0, a: 0 })),
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


    let result = color_parser.parse("");
    assert_eq!(result,Empty);

    let result = color_parser.parse("  ");
    assert_eq!(result,Empty);

    let result = color_parser.parse("  \r\n");
    assert_eq!(result,Empty);

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
        ("   23", Number(23)),
        (" \r\n 10% ", Percent(PercentFloat::new(10.0f32))),
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


    let result = number_parser.parse("");
    assert_eq!(result,Empty);

    let result = number_parser.parse("  ");
    assert_eq!(result,Empty);

    let result = number_parser.parse("  \r\n");
    assert_eq!(result,Empty);

}

#[test]
fn test_intern_name_basic() {
    let mut registry = StyleRegistry::new();
    let id1 = registry.intern_name("test1");
    let id2 = registry.intern_name("test2");
    let id1_1 = registry.intern_name("Test1");
    let id1_2 = registry.intern_name("test1");

    assert_eq!(id1, 0, "First interned name should get ID 0");
    assert_eq!(id2, 1, "Second interned name should get ID 1");
    assert_eq!(id1_1, 2, "Case is different, so it should be its own name");
    assert_eq!(id1_2, 0, "Re-interning existing name should return its original ID");

    assert_eq!(registry.resolve_name(id1), Some("test1"), "ID 0 should resolve to 'test1'");
    assert_eq!(registry.resolve_name(id2), Some("test2"), "ID 1 should resolve to 'test2'");
    assert_eq!(registry.resolve_name(999), None, "Non-existent ID should resolve to None");
}

#[test]
fn test_register_atomic_style_success() {
    let mut registry = StyleRegistry::new();
    let font_size_id = registry.register_atomic_style("fontSize", StyleValueParser::NumberParser).expect("Should register fontSize");
    let color_id = registry.register_atomic_style("color", StyleValueParser::MatchParser(&["red", "blue"])).expect("Should register color");

    assert_eq!(font_size_id, 0, "fontSize should get ID 0");
    assert_eq!(color_id, 1, "color should get ID 1");

    assert_eq!(registry.resolve_name(font_size_id), Some("fontSize"));
    assert_eq!(registry.resolve_name(color_id), Some("color"));
}

#[test]
fn test_register_atomic_style_duplicate_name() {
    let mut registry = StyleRegistry::new();
    registry.register_atomic_style("fontSize", StyleValueParser::NumberParser).expect("First fontSize register should succeed");
    let result = registry.register_atomic_style("fontSize", StyleValueParser::NumberParser); // Try to register again

    assert!(result.is_err(), "Registering duplicate atomic style should fail");
    assert_eq!(result.unwrap_err(), "Atomic style 'fontSize' is already defined.".to_string());
}

#[test]
fn test_register_atomic_style_after_lock() {
    let mut registry = StyleRegistry::new();
    registry.register_atomic_style("fontSize", StyleValueParser::NumberParser).expect("First atomic style");
    registry.register_style("myStyle", vec![]);

    let result = registry.register_atomic_style("color", StyleValueParser::MatchParser(&["red"]));


    assert!(result.is_err(), "Should not be able to register atomic style after lock");
    assert_eq!(result.unwrap_err(), "Cannot register atomic style 'color': Regular styles have already been registered.".to_string());
}

#[test]
fn test_register_style_locks_atomic_styles() {
    let mut registry = StyleRegistry::new();
    let _ = registry.register_atomic_style("fontSize", StyleValueParser::NumberParser).unwrap();

    registry.register_style("myStyle", vec![RawStyle::new("fontSize", Some("10.0"))]);

    // Try to register another atomic style
    let result = registry.register_atomic_style("color", StyleValueParser::MatchParser(&["red"]));
    assert!(result.is_err(), "Should fail to register atomic style after lock");
}

#[test]
fn test_register_style_basic_alias_definition() {
    let mut registry = StyleRegistry::new();
    let _ = registry.register_atomic_style("fontSize", StyleValueParser::NumberParser).unwrap();
    let _ = registry.register_atomic_style("color", StyleValueParser::MatchParser(&["red", "blue"])).unwrap();

    let alias_name = "smallRedText";
    let registered_style = registry.register_style(alias_name, vec![
        RawStyle::new("fontSize", Some("12.0")),
        RawStyle::new("color", Some("red")),
    ]);

    let alias_id = registered_style.style_id;
    assert_eq!(registry.resolve_name(alias_id), Some(alias_name));
    assert!(!registered_style.atomic, "Should not be atomic");
    assert!(!registered_style.overwrote, "Should not have overwritten anything new");
}

#[test]
fn test_register_style_overwriting_existing_alias() {
    let mut registry = StyleRegistry::new();
    let _ = registry.register_atomic_style("fontSize", StyleValueParser::NumberParser).unwrap();
    let _ = registry.register_atomic_style("color", StyleValueParser::MatchParser(&["red", "blue", "green"])).unwrap();

    let alias_name = "myCustomStyle";
    let alias_id = registry.intern_name(alias_name);

    let reg1 = registry.register_style(alias_name, vec![
        RawStyle::new("fontSize", Some("10.0")),
        RawStyle::new("color", Some("red")),
    ]);

    assert_eq!(reg1.style_id, alias_id);
    assert!(!reg1.overwrote);
    assert!(!reg1.atomic, "myCustomStyle should not be atomic");

    // Second registration, overwriting
    let reg2 = registry.register_style(alias_name, vec![
        RawStyle::new("fontSize", Some("14.0")),
        RawStyle::new("color", Some("blue")),
    ]);

    assert_eq!(reg2.style_id, alias_id);
    assert!(reg2.overwrote, "Should report as overwriting an existing alias");
}

#[test]
fn test_register_style_atomic_property_clash() {
    let mut registry = StyleRegistry::new();
    let font_size_id = registry.register_atomic_style("fontSize", StyleValueParser::NumberParser).unwrap();

    // Try to register an alias with the same name as an atomic style
    let registered_style = registry.register_style("fontSize", vec![
        RawStyle::new("fontSize", Some("20.0")),
    ]);

    assert!(registered_style.atomic, "Should correctly identify as an atomic style");
    assert!(!registered_style.overwrote, "Should not overwrite an atomic style");
    assert_eq!(registered_style.style_id, font_size_id, "Should return the existing atomic ID");
}

#[test]
fn test_register_style_precedence_in_raw_definitions() {
    let mut registry = StyleRegistry::new();
    let color_id = registry.register_atomic_style("color", StyleValueParser::MatchParser(&["red", "blue", "green"])).unwrap();
    let size_id = registry.register_atomic_style("size", StyleValueParser::NumberParser).unwrap();

    let registered_style = registry.register_style("precedenceTest", vec![
        RawStyle::new("color", Some("red")),   // First declaration
        RawStyle::new("size", Some("10")),
        RawStyle::new("color", Some("blue")),  // Later declaration for color should win
        RawStyle::new("size", Some("12")),   // Later declaration for size should win
        RawStyle::new("color", Some("green")), // Even later declaration for color should win
    ]);

    let alias_id = registered_style.style_id;
    let definition = get_sorted_atomic_entries(&registry, alias_id);
    assert_eq!(definition.len(), 2);
    assert_eq!(definition[0], AtomicStyle { id: color_id, value: StyleValue::Match(2) }); // 2 = green
    assert_eq!(definition[1], AtomicStyle { id: size_id, value: StyleValue::Number(12) });
}

#[test]
fn test_register_style_nested_alias_expansion() {
    let mut registry = StyleRegistry::new();
    let color_id = registry.register_atomic_style("color", StyleValueParser::MatchParser(&["red", "blue", "green"])).unwrap();
    let border_id = registry.register_atomic_style("border", StyleValueParser::NumberParser).unwrap();

    // Register a base alias "redBox"
    let red_box_id = registry.register_style("redBox", vec![
        RawStyle::new("color", Some("red")),
        RawStyle::new("border", Some("23")),
    ]).style_id;


    let red_box_definition = registry.get_definition(red_box_id).unwrap();

    println!("{:?}", red_box_definition);

    // should be [ color, border ]
    assert_eq!(red_box_definition[0], AtomicStyle { id: color_id, value: StyleValue::Match(0) });
    assert_eq!(red_box_definition[1], AtomicStyle { id: border_id, value: StyleValue::Number(23) });

    // Register a new alias "fancyBox" that uses "redBox" and adds properties
    let fancy_box_id = registry.register_style("fancyBox", vec![
        RawStyle::new("redBox", None),
        RawStyle::new("color", Some("blue")),
    ]).style_id;

    // order should be preserved [ border, color ]
    let fancy_box_definition = registry.get_definition(fancy_box_id).unwrap();

    println!("{:?}", fancy_box_definition);

    assert_eq!(fancy_box_definition[0], AtomicStyle { id: border_id, value: StyleValue::Number(23) });
    assert_eq!(fancy_box_definition[1], AtomicStyle { id: color_id, value: StyleValue::Match(1) });
}

#[test]
fn test_get_definition_for_non_existent_id() {
    let registry = StyleRegistry::new();
    assert_eq!(registry.get_definition(999), None);
}

#[test]
fn test_resolve_name_for_non_existent_id() {
    let registry = StyleRegistry::new();
    assert_eq!(registry.resolve_name(999), None);
}