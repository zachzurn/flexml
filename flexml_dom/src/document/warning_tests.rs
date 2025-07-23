use super::document::{FlexmlDocument};

#[test]
fn empty_warning() {
    let input = "";

    let document = FlexmlDocument::new(input)
        .with_name("Empty Warning Test")
        .parse();

    document.print_warnings();

    assert_eq!(document.warnings.len(), 1);
    assert_eq!(document.nodes.len(), 0);
}

#[test]
fn missing_style_definition_warning() {
    let input = "{myStyle}";

    let document = FlexmlDocument::new(input)
        .with_name("Missing Style Definitions")
        .parse();

    document.print_warnings();

    assert_eq!(document.warnings.len(), 1);

}

#[test]
fn unclosed_style_warning() {
    let input = "{myStyle bold + italic";

    let document = FlexmlDocument::new(input)
        .with_name("Unclosed Style Warning")
        .parse();

    document.print_warnings();

    assert_eq!(document.warnings.len(), 1);

}

#[test]
fn node_depth_warning() {
    let input = "[1 [2 [3 [4 [5 [6 [7 [8] 7] 6] 5] 4] 3] 2] 1] []";

    let document = FlexmlDocument::new(input)
        .with_max_depth(5)
        .with_name("Node Depth Warning")
        .parse();

    document.print_warnings();

    assert_eq!(document.warnings.len(), 1);

}

#[test]
fn max_nodes_warning() {
    let input = "[1 [2 [3 [4 [5 [6 [7 [8] 7] 6] 5] 4] 3] 2] 1] []";

    let document = FlexmlDocument::new(input)
        .with_max_nodes(5)
        .with_name("Max Nodes Warning")
        .parse();

    document.print_warnings();

    assert_eq!(document.warnings.len(), 1);

}


#[test]
fn max_nodes_warning_large() {
    let input = "[1 [2 [3 [4 [5 [6 [7 [8] 7] 6] 5] 4] 3] 2] 1] \r\n [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] \r\n [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] []";

    let document = FlexmlDocument::new(input)
        .with_max_nodes(1)
        .with_name("Max Nodes Warning Large")
        .parse();

    document.print_warnings();

    assert_eq!(document.warnings.len(), 1);

}
