pub struct Processor {
  pub ast: Vec<String>,
}

impl CoreProcessor {
  pub fn new() -> Self {
    Self { ast: vec![] }
  }
}
