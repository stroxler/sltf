extern crate regex;
use regex::Regex;

use super::tok_ast::{Tok, Prim, Ast, Atom};


struct PartialDefinition {
    name: String,
    words: Vec<Atom>,
    finished: bool,
}

enum DefinitionStatus {
    TopLevel,
    WaitingForName,
    ProcessingBody(PartialDefinition),
}

pub fn tokens_to_ast(tokens: &Vec<Tok>) -> Vec<Ast> {

    use self::DefinitionStatus::*;

    let mut ast = Vec::new();
    let mut status = TopLevel;

    for tok in tokens {
        match status {
            TopLevel => {
                match tok {
                    Tok::LitTok(prim) =>
                        ast.push(Ast::Simple(Atom::Lit(prim.clone()))),
                    Tok::SymbolTok(sym) =>
                        ast.push(Ast::Simple(Atom::Symbol(sym.clone()))),
                    Tok::Colon =>
                        status = WaitingForName,
                    bad =>
                        panic!("Unexpected top-level token {:?}", bad),
                }
            },
            WaitingForName => {
                match tok {
                    Tok::SymbolTok(name) =>
                        status = ProcessingBody(
                            PartialDefinition {
                                name: name.clone(),
                                words: Vec::new(),
                                finished: false }
                        ),
                    bad =>
                        panic!("Expected name of word after :, got {:?}", bad)
                }
            },
            ProcessingBody(ref mut partial) => {
                match tok {
                    Tok::LitTok(prim) =>
                        partial.words.push(Atom::Lit(prim.clone())),
                    Tok::SymbolTok(sym) =>
                        partial.words.push(Atom::Symbol(sym.clone())),
                    Tok::SemiColon =>
                        partial.finished = true,
                    bad =>
                        panic!("Unexpected top-level token {:?}", bad),
                }
            }
        };

        // Make sure you understand why this needs to be in a separate block from
        // the ProcessingBody section above:
        //
        // There, we have a mutable reference `partial` to the PartialDefinition.
        // Why a mutable reference? Because:
        //  - we need to be able to modify the body (so an immutable reference is no good)
        //  - we don't want to invalidate `status` (which is what happens if we take
        //    ownership, unless we redefine `status` as part of our code... that would work
        //    as an alternative to our current pattern and would be the more functional
        //    approach, but isn't how we're doing it here)
        //
        // Moreover, we cannot do the destructuring below inside of that block, because
        // we're not allowed to take ownership of `status` when there's a mutable
        // reference `partial` still out on it. In theory, calling drop(partial) allow
        // it but I wasn't seeing how to make the borrow checker happy.
        match status {
            ProcessingBody( PartialDefinition { name, words, finished: true} ) => {
                status = DefinitionStatus::TopLevel;
                ast.push(Ast::WordDef(name, words))
            },
            _ => (),
        };
    };

    ast
}

#[test]
fn test_parse_simple() {
    let tokens = vec![
        Tok::LitTok(Prim::Int(2)),
        Tok::SymbolTok("DUP".to_string()),
    ];
    let expected = vec![
        Ast::Simple(Atom::Lit(Prim::Int(2))),
        Ast::Simple(Atom::Symbol("DUP".to_string()))
    ];
    let actual = tokens_to_ast(&tokens);
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_complex() {
    let tokens = vec![
        Tok::Colon,
        Tok::SymbolTok("SQUARE".to_string()),
        Tok::SymbolTok("DUP".to_string()),
        Tok::SymbolTok("*".to_string()),
        Tok::SemiColon,
        Tok::LitTok(Prim::Int(2)),
        Tok::SymbolTok("SQUARE".to_string()),
    ];
    let expected = vec![
        Ast::WordDef(
            "SQUARE".to_string(),
            vec![
                Atom::Symbol("DUP".to_string()),
                Atom::Symbol("*".to_string()),
            ]
        ),
        Ast::Simple(Atom::Lit(Prim::Int(2))),
        Ast::Simple(Atom::Symbol("SQUARE".to_string()))
    ];
    let actual = tokens_to_ast(&tokens);
    assert_eq!(expected, actual);
}