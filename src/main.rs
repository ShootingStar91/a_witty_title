#[allow(warnings, unused)]
mod lexer;
mod parser;
mod token;
mod checker;
mod executor;

pub struct RootNode {
    pub stmts: Vec<parser::Node>,
}

fn main() {
    let source = "print 123".to_string();

    let tokens = lexer::scan(&source);
    
    println!("List of tokens:");

    for token in &tokens {
        println!("{}", token);
    }


    let mut parser = parser::Parser {
        token_index: 0,
        tokens,
        program_root: parser::Node::Root(RootNode { stmts: Vec::new() }),
        current_token: None,
    };

    let root = parser.parse();

    println!("List of statements:");
    for stmt in &root.stmts {
        println!("Statement: {}", stmt);
    }

    let checker = checker::Checker { error_messages: Vec::new() };

    checker.check_ast(&root);

    if checker.error_messages.len() != 0 {
        println!("Program was not semantically correct!");
        return;
    }



    println!("Executing...");

    let mut executor = executor::Executor { symbolTable: 1 }; 
    executor.execute(root);

    //test_scan();
}

#[allow(unused)]
fn test_scan() {
    let sources = Vec::from([
        "print 123\nprint 456".to_string(),
        "pr 43 \n   1sf fk# #".to_string(),
    ]);

    for source in sources.iter() {
        println!("\nSource:");
        println!("{}", source);
        println!("\nTokens:");

        let tokens = lexer::scan(source);

        for i in 0..tokens.len() {
            println!("{}", tokens[i].to_string());
        }
    }
}
