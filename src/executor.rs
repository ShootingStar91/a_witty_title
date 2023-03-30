use crate::parser::PrintNode;

use super::RootNode;
use super::parser::Node;

pub struct Executor {
    pub symbolTable: i32, // wrong type. temporarily until vars are added
}

impl Executor {
    pub fn execute(&mut self, root: RootNode) {
        for stmt in root.stmts {
            match stmt {
                Node::Print(printNode) => self.executePrint(printNode),
                _ => panic!("Encountered unimplemented statement while executing"),
            }
        }
    }
    fn executePrint(&mut self, node: PrintNode) {
        println!("{}", node.expr);
    }
}