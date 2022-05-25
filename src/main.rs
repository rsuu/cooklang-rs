use cooklang::cli::read;
use cooklang::syntax::{lexer, parser};

fn main() {
    let text = include_str!("../test.cook");
    let v: Vec<char> = text.chars().collect();
    let tokens = lexer::lexer(&v);
    // println!("{:#?}", tokens);
    let parse = parser::parser(tokens.as_slice(), &v);
    // println!("{:#?}", parse);
    read::parse_nodes(parse, &v);
}

#[cfg(test)]

mod test {
    use crate::Token;
    use crate::TokenType::*;
    use crate::TokenValue::*;
    use crate::*;

    #[test]
    fn test_lexer() {
        let text = "
@雪梨{121.375589%g}
";
        let tokens = lexer(text);
        let f = format!("{:?}", tokens);
        let ff = r#"[Token { t: NewLine, v: Sign }, Token { t: At, v: Sign }, Token { t: Text, v: Text("雪梨") }, Token { t: OpenBrace, v: Sign }, Token { t: Digit, v: Int(121) }, Token { t: Dot, v: Sign }, Token { t: Digit, v: Int(375589) }, Token { t: Percent, v: Sign }, Token { t: Text, v: Text("g") }, Token { t: CloseBrace, v: Sign }, Token { t: NewLine, v: Sign }]"#;

        //       assert_eq!(f, ff)
    }
}
