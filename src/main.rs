mod ast;
mod format;
mod parser;
use parser::MdxParser;

fn main() {
  let mdx = r#"
# Hello World

This is a test.

## Hello World

This is a test.

### Hello World

This is a test.

#### Hello World

This is a test.

##### Hello World

This is a test.

###### Hello World

This is a test."#;
  let ast = MdxParser::parse(mdx);
  println!("{:#?}", ast);
}
