use std::ops::RangeInclusive;
use super::nodes::Node;
use super::parser::{Parser, ParserError};

fn parse_all(input: &str) -> (Vec<Node>, Vec<ParserError>) {
    let mut parser = Parser::new(input);
    let mut nodes = Vec::new();
    while let Some(node) = parser.parse_next() {
        nodes.push(node);
    }
    (nodes, parser.errors)
}

fn check_inputs<'a>(
    inputs: &[&'a str],
    expect_nodes: RangeInclusive<usize>,
    expect_errors: RangeInclusive<usize>,
    expected_patterns: &[fn(&Node<'a>) -> bool],
) {
    for input in inputs {
        let (nodes, errors) = parse_all(input);

        let node_count = nodes.len();
        let error_count = errors.len();

        assert!(
            expect_nodes.contains(&node_count),
            "Input `{}`: Expected number of nodes in range {:?}, found {}",
            input,
            expect_nodes,
            node_count
        );

        assert!(
            expect_errors.contains(&error_count),
            "Input `{}`: Expected number of errors in range {:?}, found {} {:?}",
            input,
            expect_errors,
            error_count,
            errors
        );

        for (i, pattern) in expected_patterns.iter().enumerate() {
            assert!(
                i < node_count,
                "Input `{}`: Expected at least {} nodes to check pattern, found {}",
                input,
                i + 1,
                node_count
            );
            assert!(
                pattern(&nodes[i]),
                "Input `{}`: Node {} did not match expected pattern got {:?}",
                input,
                i,
                &nodes[i]
            );
        }
    }
}


#[test]
// parse naked text
// We are also checking that certain
// tokens don't error when on their own
// Plain pipe, close bracket, close raw, escaped raw open
// Style tags that should be parsed as text
fn parse_naked_text() {
    let inputs = &[
        "Hello ] = | =| \r\n World {myStyle bold+italic}} < \\|=",
        "Hello ] = | =| \n\r World {myStyle bold+italic}} < \\|=",
        "Hello ] = | =| \n World {myStyle bold+italic}} < \\|=",
        "Hello ] = | =| \r World {myStyle bold+italic}} < \\|=",
    ];

    check_inputs(
        inputs,
        3..=3,
        0..=0,
        &[
            |n| matches!(n, Node::Text(t) if *t == "Hello "),
            |n| matches!(n, Node::Text(t) if *t == "] = | =| "),
            |n| matches!(n, Node::Text(t) if *t == " World {myStyle bold+italic}} < \\|="),
        ],
    );
}

#[test]
fn parse_simple_box() {
    let inputs = &[
        "[bold+italic Hello World ]",
        "[bold+ italic Hello World ]",
        "[bold + italic Hello World ]",
        "[bold\r\n \n \r + \r\n italic Hello World ]",
        "[bold\r\n \r\n + \r\n \r\n italic         Hello World      ]",
        "[bold + italic:2 Hello World ]",
        "[bold + italic : 2 Hello World ]",
        "[bold + italic: 2 Hello World ]",
        "[bold + italic :2   Hello World ]",
        "[bold+ italic :2   Hello World ]",
        "[bold +italic :2   Hello World]"
    ];

    check_inputs(
        inputs,
        1..=1,
        0..=0,
        &[
            |n| {
                if let Node::BoxContainer { styles, children } = n {
                    assert_eq!(styles.len(), 2);
                    assert_eq!(styles[0].name, "bold");
                    assert_eq!(styles[1].name, "italic");

                    assert_eq!(children.len(), 1);
                    if let Node::Text(text) = &children[0] {
                        assert_eq!(*text, "Hello World");
                    } else {
                        panic!("Expected text inside box");
                    }
                    true
                } else {
                    false
                }
            },
        ],
    );
}

#[test]
fn parse_tag_and_text() {
    let inputs = &[
        "Hello <tagName> world!",
        "This is a tag <-> here"
    ];

    check_inputs(
        inputs,
        3..=3,
        0..=0,
        &[
            |n| matches!(n, Node::Text(_)),
            |n| matches!(n, Node::Tag { .. }),
            |n| matches!(n, Node::Text(_)),
        ],
    );
}

#[test]
fn parse_raw_block() {
    let inputs = &["|= This is raw =|"];

    check_inputs(
        inputs,
        1..=1,
        0..=0,
        &[
            |n| matches!(n, Node::Text(text) if text.contains("This is raw")),
        ],
    );
}

#[test]
fn parse_unclosed_raw_warns() {
    let inputs = &[
        "|= This is unterminated raw",
        "|= Another raw without close",
        "|= Nested [stuff ] but no end",
        "|= \\|=  raw without close",
    ];

    check_inputs(
        inputs,
        1..=1, // Each input should produce exactly 1 node
        1..=1, // Each input should produce exactly 1 warning
        &[
            |n| matches!(n, Node::Text(_)), // Raw fallback always returns Text node
        ],
    );
}

#[test]
// Styles should parse without any errors
fn parse_styles_first() {
    let inputs = vec![
        "{myStyle = bold+italic+size:3} [myStyle This is styled ]",
        "   {myStyle = bold+italic+size:3} [myStyle This is styled ]",
        "\r\n{myStyle = bold+italic+ size:3} [myStyle This is styled ]",
        "{myStyle bold+italic+size:3} [myStyle This is styled ]",
        "{ myStyle = +bold+italic+size:3} [myStyle This is styled ]"
    ];

    check_inputs(
        &inputs,
        1..=2,
        0..=0,
        &[
            |n| matches!(n, Node::StyleDefinition { .. }),
            |n| matches!(n, Node::BoxContainer { .. }),
        ]
    );
}

#[test]
// Styles should parse but with errors
fn parse_incomplete_styles() {
    let inputs = vec![
        "{myStyle = bold+italic+size {myStyle=bold+italic+size:3}",
        "{myStyle = bold+italic+size:3 {myStyle=bold+italic+size:3} ",
        "{myStyle} {myStyle=bold+italic+size:3}",
        "{myStyle ",
        "{myStyle   +++}",
    ];

    check_inputs(
        &inputs,
        1..=2,
        1..=2,
        &[
            |n| matches!(n, Node::StyleDefinition { .. }),
        ]
    );

}
