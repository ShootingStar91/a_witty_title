#[allow(warnings)]

mod lexer;
mod token;

fn main() {
    
    test_scan();

}

fn test_scan() {
    let sources = Vec::from([
        "print 123\nprint 456".to_string(),
        "pr 43 \n   1sf fk# #".to_string()
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