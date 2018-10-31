#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Tok {
    LitTok(Prim),
    SemiColon,
    Colon,
    SymbolTok(String)
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Atom {
    Lit(Prim),
    Symbol(String),
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Ast {
    Simple(Atom),
    WordDef(String, Vec<Atom>),
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Prim {
    Str(String), // a fully-owned string
    Int(i64),
}