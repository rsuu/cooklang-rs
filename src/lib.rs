//#![feature(type_alias_impl_trait)]
//use crate::syntax::ast::IsNode;
//pub type Nodes = Vec<impl IsNode>;

pub mod syntax {
    pub mod ast;
    pub mod lexer;
    pub mod parser;
    pub mod tokens;
}
pub mod cli {
    pub mod read;
}
