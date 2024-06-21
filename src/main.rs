mod ast;
mod core;
mod format;
mod parser;
mod transformer;
use parser::MdxParser;

fn main() {
  let mdx = r#"
# Hello World

Hello, My name is **Yazalde Filimone**. I'm a **Software Engineer** and I love **Rust**.
Do you like **Rust**?


"#;

  let ast = MdxParser::parse(mdx);
  println!("{:#?}", ast);
}
