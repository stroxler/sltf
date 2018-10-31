extern crate regex;
use regex::Regex;

use super::tok_ast::{Tok, Prim};
use super::tok_ast::Tok::*;

pub struct Lexer {
    token: regex::Regex,
    number: regex::Regex,
    symbol: regex::Regex,
    string: regex::Regex,
    delimiter: regex::Regex,
}

fn strict_regex(regex_str: &str) -> Regex {
    let mut modified = String::new();
    modified.push('^');
    modified.push_str(regex_str);
    modified.push('$');
    Regex::new(modified.as_ref())
        .expect("Failed to compile regex")
}

impl Lexer {

    pub fn new() -> Self {
        let NUMBER: &str = r"-?[0-9]+";
        let SYMBOL: &str = r#"[^"0-9\s][^"\s]*"#;
        let STRING: &str = r#""((?:\\.|[^\\"])*)""#;
        let STRING_NOCAP: &str = r#""(?:\\.|[^\\"])*""#;
        let DELIMITER: &str = r";|:";
        let number = strict_regex(NUMBER);
        let symbol = strict_regex(SYMBOL);
        let string = strict_regex(STRING);
        let delimiter = strict_regex(DELIMITER);
        let mut token_str = String::new();
        token_str.push_str(r"\s*(");
        token_str.push_str(NUMBER);
        token_str.push_str("|");
        token_str.push_str(SYMBOL);
        token_str.push_str("|");
        token_str.push_str(STRING_NOCAP);
        token_str.push_str("|");
        token_str.push_str(DELIMITER);
        token_str.push_str(r")");
        let token = Regex::new(token_str.as_ref()).expect("Failed to compile regex");
        Lexer {
            token, number, symbol, string, delimiter,
        }
    }

    pub fn tokenize(&self, input: &str) -> Vec<Tok> {
        self.token.captures_iter(input) 
            .map(|cap| cap[1].to_owned())
            .map(|raw| self.tokenize_one(raw))
            .collect()
    }

    fn tokenize_one(&self, raw_tok: String) -> Tok {
        if self.delimiter.is_match_at(raw_tok.as_ref(), 0) {
            self.tokenize_delim(raw_tok)
        } else if self.number.is_match_at(raw_tok.as_ref(), 0) {
            // NOTE: technically an overflow error probably causes a panic
            let num: i64 = raw_tok.parse().expect("Failed to parse number");
            LitTok(Prim::Int(num))
        } else if self.symbol.is_match_at(raw_tok.as_ref(), 0) {
            SymbolTok(raw_tok.to_owned())
        } else if self.string.is_match_at(raw_tok.as_ref(), 0) {
            self.tokenize_string(raw_tok)
        } else {
            unreachable!("Token didn't match any token type {:?}", raw_tok)
        }
    }

    fn tokenize_delim(&self, raw_tok: String) -> Tok {
        if raw_tok == ";" {
            SemiColon
        } else if raw_tok == ":" {
            Colon
        } else {
            unreachable!("Did not expect unknown delimiter, parsing {:?}", raw_tok)
        }
    }

    fn tokenize_string(&self, raw_tok: String) -> Tok {
        let cap = self.string.captures_iter(raw_tok.as_ref()).next()
                .expect("Did not expect non-capture with string");
        let contents = match cap.get(1) {
            Some(mtch) => {
                mtch.as_str().to_string()
                    .replace(r#"\n"#, "\n")
                    .replace(r#"\t"#, "\t")
                    .replace(r#"\r"#, "\r")
                    .replace(r#"\\"#, "\\")
                    .replace(r#"\""#, "\"")
            }
            None => String::new(),
        };
        LitTok(Prim::Str(contents))
    }

}

#[cfg(test)]
mod test_lexer {

    use super::*;

    fn tcase(input: &str, expected: Vec<Tok>) {
        let lexer: Lexer = Lexer::new();
        let actual = lexer.tokenize(input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_tokenize_delimiter() {
        tcase(";", vec![SemiColon]);
        tcase(":", vec![Colon]);
        tcase(": ;", vec![Colon, SemiColon]);
        tcase(" :", vec![Colon]);
        tcase(": ", vec![Colon]);
        tcase("   :  ", vec![Colon]);
        tcase("   :  ; ", vec![Colon, SemiColon]);
    }


    #[test]
    fn test_tokenize_number() {
        tcase("3", vec![LitTok(Prim::Int(3))]);
        tcase("-3", vec![LitTok(Prim::Int(-3))]);
        tcase(" -358     932 -5 ",
            vec![ LitTok(Prim::Int(-358))
                , LitTok(Prim::Int(932))
                , LitTok(Prim::Int(-5))
                ]);
    }

    #[test]
    fn test_tokenize_string() {

        fn tok_str(val: &str) -> Tok {
            LitTok(Prim::Str(val.to_string()))
        }

        tcase(
            r#" "" "#,
            vec![tok_str("")]
        );
        tcase(
            r#""hi""#,
            vec![tok_str("hi")]
        );
        tcase(
            r#""hello, there""#,
            vec![tok_str("hello, there")]
        );
        tcase(
            r#""\"Yo!\", he said""#,
            vec![tok_str("\"Yo!\", he said")]
        );
        tcase(
            r#""hey there" "x""#,
            vec![tok_str("hey there"), tok_str("x")]
        );
    }

    #[test]
    fn test_tokenize_symbol() {

        fn tok_sym(val: &str) -> Tok {
            SymbolTok(val.to_string())
        }

        tcase(
            r#"+"#,
            vec![tok_sym("+")],
        );
        tcase(
            r#"* /"#,
            vec![tok_sym("*"), tok_sym("/")],
        );
        tcase(
            r#"my-symbol %*+"#,
            vec![tok_sym("my-symbol"), tok_sym("%*+")],
        );
    }
}