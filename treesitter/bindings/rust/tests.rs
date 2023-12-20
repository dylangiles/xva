mod literals;

use tree_sitter::{Node, Parser};

#[test]
fn test_can_load_grammar() {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(super::language())
        .expect("Error loading xva language");
}

fn get_parser() -> Parser {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(super::language())
        .expect("Error loading Xva language");
    parser
}

/// Extracts nth sibling node at the mth level descendant from the root
///
/// `n = 0` is the level immediately descendant of the root.
fn extract_nth_node_at_mth_level(root: Node, n: usize, level: usize) -> Node {
    let mut node: Option<Node> = None;
    for i in 0..=level {
        if i == 0 {
            node = root.child(0)
        } else {
            if i == level {
                node = node.unwrap().child(n)
            } else {
                node = node.unwrap().child(0)
            }
        }
    }

    node.unwrap()
}
