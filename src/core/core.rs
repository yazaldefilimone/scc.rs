use crate::transformer::Transformer;

pub struct Core {
  pub ast: Vec<String>,
  pub transformers: Vec<Box<dyn Transformer>>,
}

impl Core {
  pub fn new() -> Self {
    Self { ast: vec![], transformers: vec![] }
  }
  pub fn run(&self) {
    self.process_transformers();
    println!("{:#?}", self.ast);
  }

  pub fn use_transformer(&mut self, transformer: Box<dyn Transformer>) {
    self.transformers.push(transformer);
  }

  pub fn process_transformers(&mut self) {
    for transformer in &self.transformers {
      transformer.transform(&mut self.ast);
    }
  }
}
