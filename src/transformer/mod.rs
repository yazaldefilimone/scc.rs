use crate::ast;

mod notation_diff;

pub trait Transformer {
  fn transform(&self, ast: &mut ast::Ast) -> ast::Ast;
}
