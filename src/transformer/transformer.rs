pub trait Transformer {
  fn transform(&self, ast: &mut ast::Ast) -> &mut ast::Ast;
}
