use super::parser::Node;
use super::parser::PrintNode;

pub struct PrintableError {
    message: String,
    start: i32,
    end: i32
}

pub struct Checker {
    pub error_messages: Vec<PrintableError>,
}

impl Checker {
    pub fn check_ast(&self, root: &super::RootNode) {
        for stmt in &root.stmts {
            match stmt {
                Node::Print(node) => self.check_print(&node),
                _ => println!("Error in typechecker: Unimplemented statement"),
            }
        }
    }
    fn check_print(&self, node: &PrintNode) {
        // check print expr conforms to printable types
    }
}
