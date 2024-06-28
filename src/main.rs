mod ast;
mod cli;
mod compiler;
mod core;
mod format;
mod highlighter;
mod parser;
mod transformer;

use cli::command_line;
use parser::MdxParser;

// ========================
// Compilte to HTML
//
fn run_html(source: String) {
  let mut core = core::Core::default();
  core.compile(&source);
}

// ========================
// Compilte to JSX
//

fn run_jsx(source: String) {
  todo!("Not implemented yet");
}
// ========================
// Compilte to Vue
//
//

fn run_vue(source: String) {
  todo!("Not implemented yet");
}
fn main() {
  let mdx = r#"
# Hello World

Hello, My name is **Yazalde Filimone**. I'm a **Software Engineer** and I love **Rust**.
Do you like **Rust**?


"#;
  let mut ast = MdxParser::parse(mdx);
  let matches = command_line();
  match matches.subcommand() {
    Some(("run", matches)) => {
      let file = matches.get_one::<String>("file").unwrap();
      let debug = matches.get_flag("debug");
      let source = std::fs::read_to_string(file).expect("could not read file");
      run_html(source);
    }
    Some(("run-jsx", matches)) => {
      let file = matches.get_one::<String>("file").unwrap();
      let debug = matches.get_flag("debug");
      let source = std::fs::read_to_string(file).expect("could not read file");
      run_jsx(source);
    }
    Some(("run-vue", matches)) => {
      let file = matches.get_one::<String>("file").unwrap();
      let debug = matches.get_flag("debug");
      let source = std::fs::read_to_string(file).expect("could not read file");
      run_vue(source);
    }
    _ => {
      panic!("Unknown command");
    }
  }
}
