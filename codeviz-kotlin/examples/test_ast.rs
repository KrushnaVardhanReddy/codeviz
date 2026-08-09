use tree_sitter::Parser;

fn main() {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_kotlin::language()).unwrap();

    let source_code = "data class DataFoo(val a: Int)";
    let tree = parser.parse(source_code, None).unwrap();
    let mut cursor = tree.walk();

    fn print_tree(cursor: &mut tree_sitter::TreeCursor, source: &str, depth: usize) {
        let node = cursor.node();
        let indent = "  ".repeat(depth);
        let text = node.utf8_text(source.as_bytes()).unwrap().replace('\n', " ");
        println!("{}{} [{} - {}]: {}", indent, node.kind(), node.start_position().row, node.end_position().row, text);

        if cursor.goto_first_child() {
            print_tree(cursor, source, depth + 1);
            while cursor.goto_next_sibling() {
                print_tree(cursor, source, depth + 1);
            }
            cursor.goto_parent();
        }
    }

    print_tree(&mut cursor, source_code, 0);
}
