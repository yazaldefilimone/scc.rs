#[derive(Debug)]
pub enum Ast {
  Root(Root),
}

#[derive(Debug)]
pub struct Root {
  pub children: Box<Vec<Node>>,
}

impl Default for Root {
  fn default() -> Self {
    Root { children: Box::new(Vec::new()) }
  }
}

#[derive(Debug)]
pub enum Node {
  Text(Text),
  CodeBlock(CodeBlock),           // ```ts code```
  Html(String),                   // <div>html</div>
  Heading(Heading),               // # heading
  Paragraph(Paragraph),           // paragraph
  List(List),                     // - list
  Blockquote(Blockquote),         // > blockquote
  Table(Table),                   // | table |
  ThematicBreak,                  // ---
  SoftBreak,                      // \
  HardBreak,                      // \
  Bold(Bold),                     // **strong**
  Italic(Italic),                 // ~~strikethrough~~ or *italic*
  Link(Link),                     // [link](url)
  Image(Link),                    // ![image](url)
  InlineCode(InlineCode),         // `inline code`
  ReactComponent(ReactComponent), // React components e.g. <Sidebar is_open={true}/>
  VueComponent(VueComponent),     // Vue components e.g. <Sidebar v-bind:is-open="true"/>
}

#[derive(Debug)]
pub struct InlineCode {
  pub code: String,
}

#[derive(Debug)]
pub struct CodeBlock {
  pub language: String,
  pub code: String,
  pub meta: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct Text {
  pub text: String,
}

#[derive(Debug)]
pub struct Link {
  pub url: String,
  pub alt: String,
  pub title: Option<String>,
}

#[derive(Debug)]
pub struct Bold {
  pub children: Box<Vec<Node>>,
}

#[derive(Debug)]
pub struct Italic {
  pub children: Box<Vec<Node>>,
}

#[derive(Debug)]
pub struct Heading {
  pub level: usize,
  pub text: String,
}

#[derive(Debug)]
pub struct Paragraph {
  pub children: Box<Vec<Node>>,
}

#[derive(Debug)]
pub struct List {
  pub ordered: bool,
  pub start: Option<usize>,
  pub children: Box<Vec<Node>>,
}

#[derive(Debug)]
pub struct Blockquote {
  pub children: Box<Vec<Node>>,
}

#[derive(Debug)]
pub struct ReactComponent {
  pub name: String,
  pub props: Vec<String>,
  pub children: Box<Vec<Node>>,
}

#[derive(Debug)]
pub struct VueComponent {
  pub name: String,
  pub props: Vec<String>,
  pub children: Box<Vec<Node>>,
}

#[derive(Debug)]
pub struct Table {
  pub header: Vec<String>,
  pub rows: Vec<Vec<String>>,
}
