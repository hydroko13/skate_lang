mod skate_lexer;
mod skate_parser;

fn main() {

    let text: String = "1 + 2 + 3".to_string();

    let mut lexer = skate_lexer::Lexer::new(&text);

    lexer.tokenize();

    println!("Tokens: {:?}", lexer.tokens);




}
