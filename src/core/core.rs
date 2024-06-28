use crate::{ast, parser::MdxParser, transformer::Transformer};

pub struct Core<'a> {
  ast: Option<ast::Ast>,
  transformers: Vec<Box<dyn Transformer>>,
  raw: &'a str,
}

impl<'a> Default for Core<'a> {
  fn default() -> Self {
    Self::new()
  }
}

impl<'a> Core<'a> {
  pub fn new() -> Self {
    Self { transformers: vec![], raw: "", ast: None }
  }

  pub fn use_transformer(mut self, transformer: Box<dyn Transformer>) -> Self {
    self.transformers.push(transformer);
    self
  }

  fn transform(&mut self) {
    if let Some(ast) = &mut self.ast {
      for transformer in &mut self.transformers {
        let mut ast = ast.clone();
        self.ast = Some(transformer.transform(&mut ast).clone());
      }
    }
  }

  pub fn compile(&mut self, raw: &str) {
    self.ast = Some(self.parse(raw));
    self.transform();
  }

  pub fn parse(&self, raw: &str) -> ast::Ast {
    return MdxParser::parse(raw);
  }
}
