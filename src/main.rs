#[allow(warnings)]

mod token;
mod lexer;


fn main() {

    let source = "\"+-()123x5# \"".to_string();

    println!("\nSource:");
    println!("{}", source);
    println!("\nTokens:");

    let tokens = lexer::scan(source);

    for i in 0..tokens.len() {
        println!("{}", tokens[i].to_string());
    }

}

