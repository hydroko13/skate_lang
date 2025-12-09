#[derive(Debug)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenParen,
    CloseParen,
    OpenSquareBracket,
    CloseSquareBracket,
    Number(f64),
}

pub struct Lexer {
    pub text: String,
    pub idx: usize,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(text: &String) -> Lexer {
        Lexer {
            text: text.clone(),
            idx: 0,
            tokens: Vec::new(),
        }
    }

    pub fn get_current_char(&self) -> Option<char> {
        self.text.chars().nth(self.idx)
    }

    pub fn tokenize(&mut self) {
        loop {
            match self.get_current_char() {
                Some(c) => {
                    if c == ' ' || c == '\t' {
                        self.idx += 1;
                    } else if c == '+' {
                        self.tokens.push(Token::Plus);
                        self.idx += 1;
                    } else if c == '-' {
                        self.tokens.push(Token::Minus);
                        self.idx += 1;
                    } else if c == '*' {
                        self.tokens.push(Token::Multiply);
                        self.idx += 1;
                    } else if c == '/' {
                        self.tokens.push(Token::Divide);
                        self.idx += 1;
                    } else if c == '(' {
                        self.tokens.push(Token::OpenParen);
                        self.idx += 1;
                    } else if c == ')' {
                        self.tokens.push(Token::CloseParen);
                        self.idx += 1;
                    } else if c == '[' {
                        self.tokens.push(Token::OpenSquareBracket);
                        self.idx += 1;
                    } else if c == ']' {
                        self.tokens.push(Token::CloseSquareBracket);
                        self.idx += 1;
                    } else if c.is_ascii_digit() {
                        let mut num_str: String = String::new();
                        let mut dot_count: usize = 0;
                        loop {

                            match self.get_current_char() {
                                Some(c) => {
                                    if c.is_ascii_digit() {

                                        num_str.push(c);
                                        self.idx += 1;
                                    } 
                                    else if c == '.' {
                                        dot_count += 1;
                                        if dot_count >= 2 {
                                            num_str = "-1".to_string();
                                            break;
                                        }
                                        num_str.push(c);
                                        self.idx += 1;
                                    }
                                    else {
                                         
                                        break;
                                    }
                                    
                                },
                                None => break
                            }
                        }
                        self.tokens.push(Token::Number(num_str.parse().unwrap()));
                       

                        
                    } else {
                        self.idx += 1;
                    }
                }
                None => break,
            }
        }
    }
}

pub fn run() {
    let text: String = "1 + (2 - 3.1)".to_string();
    let mut lexer = Lexer::new(&text);

    lexer.tokenize();

    println!("{:?}", lexer.tokens);
}
