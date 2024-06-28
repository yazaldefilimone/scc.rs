/*
Copyright (c) 2024 Yazalde Filimone. All rights reserved.


*/
use crate::ast;
use ast::Node;

// ==================
// guia: https://www.markdownguide.org/basic-syntax/

pub struct MdxParser<'a> {
  pub input: &'a str,
  pub cursor: usize,
}

impl<'a> MdxParser<'a> {
  fn new(input: &'a str) -> MdxParser<'a> {
    MdxParser { input, cursor: 0 }
  }

  pub fn parse(input: &str) -> ast::Ast {
    let mut parser = MdxParser::new(input);
    ast::Ast::Root(parser.parse_root())
  }
  // parser methods
  fn parse_root(&mut self) -> ast::Root {
    let mut root = ast::Root::default();
    while let Some(node) = self.parse_node() {
      root.children.push(node);
    }
    root
  }
  // ==================
  // parse node
  //
  fn parse_node(&mut self) -> Option<ast::Node> {
    self.skip_trivial();
    if self.is_end() {
      return None;
    }
    let node = match self.peek_one() {
      '#' => self.parse_heading(),
      '`' => self.parse_code(),
      '>' => self.parse_blockquote(),
      '!' => self.parse_image(),
      '[' => self.parse_link(),
      'a'..='z' | 'A'..='Z' => self.parse_paragraphs(),
      '1'..='9' | '*' | '_' | '+' | '-' => self.parse_list(),
      _ => panic!("Unknown character {}", self.peek_one()),
    };
    Some(node)
  }

  // ==================
  // parse text, inline, strong, em, bold, italic, link
  //

  fn parse_line_node(&mut self) -> ast::Node {
    match self.peek_one() {
      '`' => self.parse_inline_code(),
      '*' => {
        if self.starts_with("**") {
          return self.parse_strong();
        }
        return self.parse_italic();
      }
      '_' => self.parse_italic(),
      '[' => self.parse_link(),
      _ => self.parse_text(),
    }
  }

  fn parse_strong(&mut self) -> ast::Node {
    self.consume_expect("**");
    let mut children = Vec::new();
    while !self.is_end() && self.peek_one() != '*' {
      children.push(self.parse_line_node());
    }
    self.consume_expect("**");
    Node::Bold(ast::Bold { children: Box::new(children) })
  }

  fn parse_italic(&mut self) -> ast::Node {
    if !self.starts_with("*") || !self.starts_with("_") {
      panic!("Expected '*' or '_' but got '{}'", self.peek_one());
    }
    let character = self.consume();
    let mut children = Vec::new();
    while !self.is_end() && self.peek_one() != '*' {
      children.push(self.parse_line_node());
    }
    self.consume_expect(&character.to_string());
    Node::Italic(ast::Italic { children: Box::new(children) })
  }

  fn parse_paragraphs(&mut self) -> ast::Node {
    let mut children = Vec::new();
    while !self.is_end() && !self.is_doble_newline() {
      children.push(self.parse_line_node());
    }
    Node::Paragraph(ast::Paragraph { children: Box::new(children) })
  }
  fn parse_list(&mut self) -> ast::Node {
    match self.peek_one() {
      '+' | '-' | '*' | '_' => self.parse_unordered_list(),
      '1'..='9' => self.parse_ordered_list(),
      _ => panic!("Unknown character {}", self.peek_one()),
    }
  }

  /*
  - Item
  - Item
  */
  fn parse_unordered_list(&mut self) -> ast::Node {
    let indicator = self.peek_one();
    let mut children = Vec::new();
    while !self.is_end() && self.is_doble_newline() {
      children.push(self.parse_unordered_list_item(&indicator.to_string()));
    }
    self.consume_expect_double_newline();
    Node::List(ast::List { ordered: false, start: None, children: Box::new(children) })
  }
  // - Item
  fn parse_unordered_list_item(&mut self, indicator: &str) -> ast::Node {
    self.consume_expect(indicator);
    self.consume_expect_whitespace();

    let line = self.parse_line_node();
    return line;
  }

  /*
  1. Item
  2. Item
  */

  fn parse_ordered_list(&mut self) -> ast::Node {
    if !self.is_digit() {
      panic!("Expected number but got '{}'", self.peek_one());
    }

    let text = self.consume_while(|c| c != '.');
    let start = Some(text.parse::<usize>().unwrap());
    let mut children = Vec::new();
    while !self.is_end() && self.is_doble_newline() {
      children.push(self.parse_ordered_list_item());
    }
    self.consume_expect_double_newline();
    Node::List(ast::List { ordered: true, start, children: Box::new(children) })
  }

  fn parse_ordered_list_item(&mut self) -> ast::Node {
    self.parse_number();
    self.consume_expect(".");
    self.consume_expect_whitespace();
    return self.parse_line_node();
  }
  fn parse_number(&mut self) -> usize {
    let mut number = String::new();
    while !self.is_end() && self.is_digit() {
      number.push(self.consume());
    }
    if number.is_empty() {
      panic!("Expected number but got '{}'", self.peek_one());
    }
    number.parse::<usize>().unwrap()
  }
  fn parse_blockquote(&mut self) -> ast::Node {
    self.consume_expect(">");
    let mut children = Vec::new();
    while !self.is_end() && self.is_newline() {
      children.push(self.parse_node().unwrap());
    }
    Node::Blockquote(ast::Blockquote { children: Box::new(children) })
  }

  fn parse_link(&mut self) -> ast::Node {
    // [alt](url "title")
    self.consume_expect("[");
    let alt = self.consume_while(|c| c != ']').to_string();
    self.consume_expect("]");
    self.consume_expect("(");
    let mut url = String::new();
    while !self.is_end() {
      self.skip_whitespace();
      if self.peek_one() == '"' || self.peek_one() == '\'' || self.peek_one() == ')' {
        break;
      }
      url.push(self.consume());
    }
    let maybe_title = self.consume_while(|c| c != ')').trim();
    let title = if maybe_title.is_empty() {
      None
    } else {
      Some(maybe_title.to_string())
    };
    self.consume_expect(")");
    Node::Link(ast::Link { url, alt, title })
  }

  fn parse_image(&mut self) -> ast::Node {
    // ![alt](url "title")
    self.consume_expect("![");
    let alt = self.consume_while(|c| c != ']').to_string();
    self.consume_expect("]");

    self.consume_expect("(");
    let mut url = String::new();
    while !self.is_end() {
      self.skip_whitespace();
      if self.peek_one() == '"' || self.peek_one() == '\'' || self.peek_one() == ')' {
        break;
      }
      url.push(self.consume());
    }
    let maybe_title = self.consume_while(|c| c != ')').trim();
    let title = if maybe_title.is_empty() {
      None
    } else {
      Some(maybe_title.to_string())
    };
    self.consume_expect(")");
    Node::Image(ast::Link { url, alt, title })
  }

  fn parse_heading(&mut self) -> Node {
    self.consume_expect("#");
    let level = self.parse_heading_level();
    self.consume_expect_whitespace();
    let text = self.consume_while(|c| c != '\n').to_string();
    self.consume_expect_newline();
    Node::Heading(ast::Heading { level, text })
  }

  fn parse_heading_level(&mut self) -> usize {
    let mut level = 1;
    while !self.is_end() && self.peek_one() == '#' {
      self.consume();
      level += 1;
    }
    level
  }

  fn parse_code(&mut self) -> ast::Node {
    if self.peek_many(3) == "```" {
      return self.parse_code_block();
    }
    return self.parse_inline_code();
  }

  fn parse_inline_code(&mut self) -> ast::Node {
    self.consume_expect("`");
    let code = self.consume_while(|c| c != '`').to_string();
    Node::InlineCode(ast::InlineCode { code })
  }
  fn parse_code_block(&mut self) -> ast::Node {
    let language = self.parse_code_block_language();
    let mut code = String::new();
    while !self.is_end() && !self.starts_with("```") {
      code.push_str(&self.consume_while(|character| character != '\n'));
    }
    Node::CodeBlock(ast::CodeBlock { language, code, meta: None })
  }

  fn parse_code_block_language(&mut self) -> String {
    self.consume_expect("```");
    let language = self.consume_while(|c| c != '\n');
    self.consume_expect("\n");
    language.to_string()
  }
  // tex e.g: This is a test, **bold** and *italic* and `code`.
  fn parse_text(&mut self) -> ast::Node {
    let mut text = String::new();
    while !self.is_end() && !self.is_doble_newline() {
      if self.contains(vec!["`", "*", "_"]) {
        break;
      }
      text.push(self.consume());
    }
    return Node::Text(ast::Text { text });
  }

  // lexer methods
  fn peek_many(&self, count: usize) -> String {
    self.input[self.cursor..].chars().take(count).collect()
  }

  fn starts_with(&self, s: &str) -> bool {
    self.input[self.cursor..].starts_with(s)
  }

  fn is_end(&self) -> bool {
    self.cursor >= self.input.len()
  }

  fn advance_one(&mut self) {
    self.cursor += 1;
  }

  fn advance_many(&mut self, count: usize) {
    self.cursor += count;
  }

  fn consume(&mut self) -> char {
    let mut iter = self.input[self.cursor..].char_indices();
    let (_, cur_char) = iter.next().unwrap();
    let (next_cursor, _) = iter.next().unwrap_or((1, ' '));
    self.cursor += next_cursor;
    cur_char
  }

  fn consume_expect(&mut self, text: &str) {
    if &self.peek_many(text.len()) == text {
      self.advance_many(text.len());
    } else {
      panic!("Expected '{}' but got '{}'", text, &self.peek_many(text.len()));
    }
  }

  fn consume_expect_whitespace(&mut self) {
    if !self.is_end() && self.peek_one().is_whitespace() {
      self.advance_one();
    } else {
      panic!("Expected whitespace but got '{}'", self.peek_one());
    }
  }

  // fn consume_expect_many(&mut self, expectds: Vec<&'a str>) -> &'a str {
  //   for expectd in &expectds {
  //     if &self.peek_many(expectd.len()) == expectd {
  //       self.advance_many(expectd.len());
  //       return expectd;
  //     }
  //   }
  //   // report error
  //   let report_text = expectds.join(" or ");
  //   let current_text = &self.peek_many(expectds[0].len());
  //   panic!("Expected '{}' but got '{}'", report_text, current_text);
  // }

  fn contains(&self, expectds: Vec<&str>) -> bool {
    for expectd in expectds {
      if &self.peek_many(expectd.len()) == expectd {
        return true;
      }
    }
    false
  }

  fn consume_expect_newline(&mut self) {
    if !self.is_end() && self.is_newline() {
      return self.advance_one();
    }
    panic!("Expected newline but got '{}'", self.peek_one());
  }
  fn consume_expect_double_newline(&mut self) {
    if !self.is_end() && self.is_doble_newline() {
      return self.advance_many(2);
    }
    panic!("Expected double newline but got '{}'", self.peek_one());
  }

  fn consume_while(&mut self, mut test: impl FnMut(char) -> bool) -> &'a str {
    let start_cursor = self.cursor;
    while !self.is_end() && test(self.peek_one()) {
      self.advance_one();
    }
    &self.input[start_cursor..self.cursor]
  }

  fn skip_whitespace(&mut self) {
    self.consume_while(char::is_whitespace);
  }

  fn is_whitespace(&self) -> bool {
    self.peek_one().is_whitespace()
  }

  fn is_newline(&self) -> bool {
    self.peek_one() == '\n'
  }

  fn is_doble_newline(&self) -> bool {
    self.peek_many(2) == "\n\n"
  }
  fn is_digit(&self) -> bool {
    match self.peek_one() {
      '0'..='9' => true,
      _ => false,
    }
  }
  fn skip_trivial(&mut self) {
    self.skip_whitespace();
  }

  fn peek_one(&self) -> char {
    self.input[self.cursor..].chars().next().unwrap()
  }
}
