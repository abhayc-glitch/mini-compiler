
//  Exports variants - instead of using Token::Function, consumers can use Function directly.
pub use self::Token::{
    Function,
    Extern,
    Delimiter,
    OpeningParenthesis,
    ClosingParenthesis,
    Comma,
    Ident,
    Number,
    Operator
};

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Function,
    Extern,
    If,
    Then,
    Else,
    Def,
    Delimiter, //';' character
    OpeningParenthesis,
    ClosingParenthesis,
    Comma,
    Ident(String),
    Number(f64),
    Operator(String)
}

// Takes in Vector of string from the parser
pub fn tokenize(input: &str) -> Vec<Token> {
    let comment_re = regex!(r"(?m)//.*\n");
    //By replacing the comments with newline characters, the preprocessed string is 
    //created, which represents the input string with all the comments removed. 
    let preproccesed = comment_re.replace_all(input, "\n");

    let mut result = Vec::new();

    // Assigns names based on 
    let token_re = regex!(concat!(
        r"(?P<ident>\p{Alphabetic}\w*)|",
        r"(?P<number>\d+\.?\d*)|",
        r"(?P<delimiter>;)|",
        r"(?P<oppar>\()|",
        r"(?P<clpar>\))|",
        r"(?P<comma>,)|",
        r"(?P<operator>\S)"));
    
     for cap in token_re.captures_iter(preprocessed.as_str()) {
        let token = if cap.name("ident").is_some() {
            match cap.name("ident").unwrap() {
                "function" => Function,
                "extern" => Extern,
                ident => Ident(ident.to_string())
            }
        } else if cap.name("number").is_some() {
            match cap.name("number").unwrap().parse() {
                Ok(number) => Number(number),
                Err(_) => panic!("Lexer failed trying to parse number")
            }
        } else if cap.name("delimiter").is_some() {
            Delimiter
        } else if cap.name("oppar").is_some() {
            OpeningParenthesis
        } else if cap.name("clpar").is_some() {
            ClosingParenthesis
        } else if cap.name("comma").is_some() {
            Comma
        } else {
            Operator(cap.name("operator").unwrap().to_string())
        };

        result.push(token)
    }

    return result;

}

