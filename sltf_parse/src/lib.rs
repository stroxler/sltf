#[cfg(test)] #[macro_use]
extern crate pretty_assertions;
extern crate regex;

pub mod lex;
pub mod tok_ast;
pub mod parse;

pub use tok_ast::Ast;
pub use tok_ast::Atom;
pub use tok_ast::Prim;


pub struct Parser {
    lexer: lex::Lexer,
}

impl Parser {

    pub fn new() -> Self {
        Parser { lexer: lex::Lexer::new() }
    }

    pub fn parse(&self, input: &str) -> Vec<Ast> {
        let tokens = self.lexer.tokenize(input);
        let ast = parse::tokens_to_ast(&tokens);
        ast
    }
}


#[cfg(test)]
mod test_parser {
    use super ::*;


}