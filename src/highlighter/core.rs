use std::{fs, path::Path};

pub struct Highlighter<'a> {
  path_name: &'a Path,
}

// suport vscode theme file and zed theme file
impl<'a> Highlighter<'a> {
  pub fn new(path_name: &Path) -> Self {
    Self { path_name }
  }

  pub fn highlight(&self, code: &str, language: &str) {
    todo!("to implement");
  }

  pub fn is_supported(&self) -> bool {
    return self.path_name.extension().unwrap() == "json";
  }

  pub fn json_parser(&mut self) {
    let text_json = fs::read_to_string(self.path_name).inspect_err(|err| {
      eprintln!("Failed to read file: {}", err);
    });
    let text_json = text_json.unwrap().as_str();
    // let json = serde_json::from_str(text_json).inspect_err(|err| {
    //   eprintln!("Failed to parse json: {}", err);
    // });

    println!("{:#?}", text_json);
  }
}
