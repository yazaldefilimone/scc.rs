use crate::ast;
use crate::transformer::Transformer;

pub struct HtmlCompiler {
  output: String,
}

impl HtmlCompiler {
  pub fn new() -> Self {
    Self { output: String::new() }
  }
  pub fn compile(&mut self, ast: &mut ast::Ast) -> String {
    match ast {
      ast::Ast::Root(root) => self.compile_root(root),
    }
    return self.output.trim().to_string();
  }

  fn compile_root(&mut self, root: &mut ast::Root) {
    for node in &mut *root.children {
      self.compile_node(node);
    }
  }

  pub fn compile_node(&mut self, node: &mut ast::Node) {
    match node {
      ast::Node::Heading(heading) => self.compile_heading(heading),
      ast::Node::Text(text) => self.compile_text(text),
      ast::Node::InlineCode(inline_code) => self.compile_inline_code(inline_code),
      ast::Node::CodeBlock(code_block) => self.compile_code_block(code_block),
      ast::Node::List(list) => self.compile_list(list),
      ast::Node::Blockquote(blockquote) => self.compile_blockquote(blockquote),
      ast::Node::Link(link) => self.compile_link(link),
      ast::Node::Image(image) => self.compile_image(image),
      ast::Node::Paragraph(paragraph) => self.compile_paragraph(paragraph),
      _ => println!("Unknown node"),
    }
  }

  fn compile_line_node(&mut self, node: &mut ast::Node) {
    match node {
      ast::Node::Text(text) => self.compile_text(text),
      ast::Node::InlineCode(inline_code) => self.compile_inline_code(inline_code),
      ast::Node::CodeBlock(code_block) => self.compile_code_block(code_block),
      ast::Node::Bold(bold) => self.compile_bold(bold),
      ast::Node::Italic(italic) => self.compile_italic(italic),
      ast::Node::Link(link) => self.compile_link(link),
      ast::Node::Image(image) => self.compile_image(image),
      _ => println!("Unknown node"),
    }
  }

  fn compile_bold(&mut self, bold: &mut ast::Bold) {
    self.push("<b>");
    for node in &mut *bold.children {
      self.compile_line_node(node);
    }
    self.push("</b>");
  }

  fn compile_italic(&mut self, italic: &mut ast::Italic) {
    self.push("<i>");
    for node in &mut *italic.children {
      self.compile_line_node(node);
    }
    self.push("</i>");
  }
  fn compile_heading(&mut self, heading: &mut ast::Heading) {
    self.push(&format!("<h{}>{}</h{}>", heading.level, heading.text, heading.level));
    self.double_new_line();
  }

  fn compile_text(&mut self, text: &mut ast::Text) {
    self.output.push_str(&text.text);
  }

  fn compile_inline_code(&mut self, inline_code: &mut ast::InlineCode) {
    self.output.push_str(&format!("<code>{}</code>", inline_code.code));
  }

  fn compile_code_block(&mut self, code_block: &mut ast::CodeBlock) {
    // TODO: add language
    let code = format!(
      r#"
      <pre>
        <code>{}</code>
      </pre>"#,
      code_block.code,
    );
    self.push(code.as_str())
  }

  fn compile_list(&mut self, list: &mut ast::List) {
    let mut list_items = String::new();
    for item in &*list.children {
      list_items.push_str(&format!("<li>{}</li>", item));
    }
    let tag = if list.ordered { "ol" } else { "ul" };
    let text = format!(
      r#"
    <{}>
      {}
    </{}>
    "#,
      tag, list_items, tag
    );
    self.new_line();
    self.push(&text.trim());
    self.double_new_line();
  }

  fn compile_blockquote(&mut self, blockquote: &mut ast::Blockquote) {
    let mut blockquote_items = String::new();
    for item in &*blockquote.children {
      blockquote_items.push_str(&format!("<p>{}</p>", item));
    }
    self.push(&format!("<blockquote>{}</blockquote>", blockquote_items));
  }

  fn compile_link(&mut self, link: &mut ast::Link) {
    // TODO: add title
    self.push(&format!("<a href=\"{}\">{}</a>", link.url, link.alt));
  }

  fn compile_image(&mut self, image: &mut ast::Link) {
    // TODO: add title
    self.push(&format!("<img src=\"{}\" alt=\"{}\" />", image.url, image.alt));
    self.double_new_line();
  }

  fn compile_paragraph(&mut self, paragraph: &mut ast::Paragraph) {
    self.push("<p>");
    for node in &mut *paragraph.children {
      self.compile_line_node(node);
    }
    self.push("</p>");
    self.double_new_line();
  }

  pub fn new_line(&mut self) {
    self.output.push_str("\n");
  }

  pub fn double_new_line(&mut self) {
    self.output.push_str("\n\n");
  }
  pub fn white_space(&mut self) {
    self.output.push_str(" ");
  }
  pub fn push(&mut self, text: &str) {
    self.output.push_str(text);
  }
}
