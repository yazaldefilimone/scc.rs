use std::fmt::Display;

use crate::ast;

impl Display for ast::Node {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ast::Node::Heading(heading) => write!(f, "h{} {}", heading.level, heading.text),
      ast::Node::Text(text) => write!(f, "p {}", text.text),
      ast::Node::InlineCode(inline_code) => write!(f, "`{}`", inline_code.code),
      ast::Node::CodeBlock(code_block) => write!(f, "```{}\n{}\n```", code_block.language, code_block.code),
      ast::Node::List(list) => write!(f, "list {:?}", list.children),
      ast::Node::Blockquote(blockquote) => write!(f, "> {:?}", blockquote.children),
      ast::Node::Link(link) => write!(f, "[{}]({})", link.alt, link.url),
      ast::Node::Image(image) => write!(f, "![{}]({})", image.alt, image.url),
      ast::Node::Paragraph(paragraph) => write!(f, "p {:?}", paragraph.children),
      _ => write!(f, "Unknown node"),
    }
  }
}

impl Display for ast::Ast {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ast::Ast::Root(root) => write!(f, "{:?}", root.children),
    }
  }
}
