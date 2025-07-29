use std::ops::RangeInclusive;
use super::nodes::Node;
use super::parser::{FlexmlDocument};

pub(super)  fn check_inputs<'a>(
    inputs: &[&'a str],
    expect_nodes: RangeInclusive<usize>,
    expect_errors: RangeInclusive<usize>,
    expected_patterns: &[fn(&Node<'a>) -> bool],
) {
    for input in inputs {
        let document = FlexmlDocument::new(input).parse();

        let node_count = document.nodes.len();
        let error_count = document.warnings.len();

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
            document.warnings
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
                pattern(&document.nodes[i]),
                "Input `{}`: Node {} did not match expected pattern got {:?}",
                input,
                i,
                &document.nodes[i]
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
        2..=2,
        0..=0,
        &[
            |n| if let Node::Text(text) = n { text == &"Hello " } else { false },
            |n| if let Node::Text(text) = n {
                text.starts_with(&"] = | =| ") && text.ends_with(" World {myStyle bold+italic}} < \\|=")
            } else {
                false
            },
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

fn count_box_depth(node: &Node) -> usize {
    match node {
        Node::BoxContainer { children, .. } => {
            let child_depths = children.iter().map(count_box_depth).max().unwrap_or(0);
            1 + child_depths
        }
        _ => 0,
    }
}

#[test]
fn max_node_depth() {
    //6 deep
    let input = "[1 [2 [3 [4 [5 [6 [7 [8] 7] 6] 5] 4] 3] 2] 1] []";

    let document = FlexmlDocument::new(input)
        .with_max_depth(5)
        .parse();

    let warnings= document.get_warnings();

    let actual_depth = count_box_depth(&document.nodes[0]);

    assert_eq!(actual_depth, 5);
    assert_eq!(warnings.len(), 1);

}


fn count_nodes(nodes: &[Node]) -> usize {
    nodes.iter().map(|node| match node {
        Node::BoxContainer { children, .. } => {
            1 + count_nodes(children)
        }
        _ => 1, // Leaf nodes
    }).sum()
}


#[test]
fn max_nodes() {
    let inputs = [
        "[1 [2 [3 [4 [5 [6 [7 [8] 7] 6] 5] 4] 3] 2] 1] []",
        "[] [] [] [] [] [] [] []",
        "{myStyle = bold} {myOtherStyle = fontSize:3} [ My box [ with a child ] ] [And another box]"
    ];

    inputs.iter().for_each(|input| {
        let document = FlexmlDocument::new(input)
            .with_max_nodes(5)
            .parse();

        println!("{:?}",document.nodes);

        let warnings= document.get_warnings();

        let actual_count = count_nodes(&document.nodes);

        assert_eq!(actual_count, 5);
        assert_eq!(warnings.len(), 1);
    });

}

#[test]
fn newline_handling() {
    let input = "\r\n\r\n\r\n";

    let document = FlexmlDocument::new(input).parse();

    assert_eq!(document.nodes.len(), 0);
}
