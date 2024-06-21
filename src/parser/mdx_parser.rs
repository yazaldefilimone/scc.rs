use crate::ast;
use ast::Node;

pub struct MdxParser<'a> {
  pub input: &'a str,
  pub cursor: usize,
}

impl<'a> MdxParser<'a> {
  fn new(input: &'a str) -> MdxParser<'a> {
    MdxParser { input, cursor: 0 }
  }

  pub fn parse(input: &str) -> ast::Root {
    let mut parser = MdxParser::new(input);
    parser.parse_root()
  }
  // parser methods
  fn parse_root(&mut self) -> ast::Root {
    let mut document = ast::Root::default();
    while !self.is_end() {
      document.children.push(self.parse_node());
    }
    document
  }

  fn parse_node(&mut self) -> ast::Node {
    self.skip_trivial();
    let node = match self.peek_one() {
      '#' => self.parse_heading(),
      '`' => self.parse_code(),
      '*' => self.parse_list_item(),
      '>' => self.parse_blockquote(),
      '!' => self.parse_image(),
      '[' => self.parse_link(),
      'a'..='z' | 'A'..='Z' => self.parse_paragraph(),
      '1'..='9' => self.parse_list_item(),
      _ => panic!("Unknown character {}", self.peek_one()),
    };
    node
  }

  fn parse_paragraph(&mut self) -> ast::Node {
    let mut children = Vec::new();
    while !self.is_end() && self.peek_one() != '\n' {
      children.push(self.parse_node());
    }
    Node::Paragraph(ast::Paragraph { children: Box::new(children) })
  }

  fn parse_list_item(&mut self) -> ast::Node {
    if self.peek_one() == '*' {
      self.consume_expect("*");
      let mut children = Vec::new();
      while !self.is_end() && self.peek_one() != '\n' {
        self.skip_trivial();
        self.consume_expect("*");
        children.push(self.parse_node());
      }
      return Node::List(ast::List { ordered: false, start: None, children: Box::new(children) });
    }
    let mut children = Vec::new();
    self.skip_whitespace();
    let number = self.parse_number();
    self.skip_whitespace();
    self.consume_expect(".");
    children.push(self.parse_node());
    while !self.is_end() && self.peek_one() != '\n' {
      self.consume_while(|c| c == '.');
      children.push(self.parse_node());
    }
    Node::List(ast::List { ordered: true, start: Some(number), children: Box::new(children) })
  }

  fn parse_number(&mut self) -> usize {
    let mut number = String::new();
    while self.peek_one().is_digit(10) {
      number.push(self.consume());
    }
    number.parse::<usize>().unwrap()
  }
  fn parse_blockquote(&mut self) -> ast::Node {
    self.consume_expect(">");
    let mut children = Vec::new();
    while !self.is_end() && self.peek_one() != '\n' {
      children.push(self.parse_node());
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
    let text = self.consume_while(|c| c != '\n').to_owned();
    Node::Heading(ast::Heading::new(level, text))
  }

  fn parse_heading_level(&mut self) -> usize {
    let mut level = 1;
    while self.peek_one() == '#' {
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
    while !self.is_end() && self.starts_with("```") {
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

  fn parse_text(&mut self) -> ast::Node {
    let text = self.consume_while(|c| c != '\n');
    return Node::Text(ast::Text { text: text.to_owned() });
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

  fn skip_trivial(&mut self) {
    self.skip_whitespace();
  }

  fn peek_one(&self) -> char {
    self.input[self.cursor..].chars().next().unwrap()
  }
}
