use super::parser::{Parser};

#[test]
fn empty_warning() {
    let input = "";

    let mut parser = Parser::new(input);
    let (nodes, warnings) = crate::parsing::tests::parse_all_with_parser(&mut parser);

    parser.print_warnings("empty_warning.flexml");

    assert_eq!(warnings.len(), 1);
    assert_eq!(nodes.len(), 0);
}

#[test]
fn missing_style_definition_warning() {
    let input = "{myStyle}";

    let mut parser = Parser::new(input);
    let (_, warnings) = crate::parsing::tests::parse_all_with_parser(&mut parser);

    parser.print_warnings("missing_style_definition_warning.flexml");

    assert_eq!(warnings.len(), 1);

}

#[test]
fn unclosed_style_warning() {
    let input = "{myStyle bold + italic";

    let mut parser = Parser::new(input);
    let (_, warnings) = crate::parsing::tests::parse_all_with_parser(&mut parser);

    parser.print_warnings("unclosed_style_warning.flexml");

    assert_eq!(warnings.len(), 1);

}

#[test]
fn node_depth_warning() {
    let input = "[1 [2 [3 [4 [5 [6 [7 [8] 7] 6] 5] 4] 3] 2] 1] []";

    let mut parser = Parser::new(input).with_max_depth(5);
    let (_, warnings) = crate::parsing::tests::parse_all_with_parser(&mut parser);

    parser.print_warnings("node_depth_warning.flexml");

    assert_eq!(warnings.len(), 1);

}

#[test]
fn max_nodes_warning() {
    let input = "[1 [2 [3 [4 [5 [6 [7 [8] 7] 6] 5] 4] 3] 2] 1] []";

    let mut parser = Parser::new(input).with_max_nodes(1);
    let (_, warnings) = crate::parsing::tests::parse_all_with_parser(&mut parser);

    parser.print_warnings("max_nodes_warning.flexml");

    assert_eq!(warnings.len(), 1);

}


#[test]
fn max_nodes_warning_large() {
    let input = "[1 [2 [3 [4 [5 [6 [7 [8] 7] 6] 5] 4] 3] 2] 1] \r\n [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] \r\n [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] []";

    let mut parser = Parser::new(input).with_max_nodes(1);
    let (_, warnings) = crate::parsing::tests::parse_all_with_parser(&mut parser);

    parser.print_warnings("max_nodes_warning_large.flexml");

    assert_eq!(warnings.len(), 1);

}
