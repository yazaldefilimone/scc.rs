use clap::{Arg, ArgAction, Command};

pub fn command_line() -> clap::ArgMatches {
  let matches = Command::new("scc.rs")
    .about("Rust-based Structured Content Compiler.")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .author("yazaldefi <yazaldefilimon@gmail.com>")
    .subcommand(
      Command::new("run")
        .about("compile a mdx or md file to html.")
        .arg(Arg::new("file").help("the mdx or md file to compile.").required(true)),
    )
    .subcommand(
      Command::new("run-jsx")
        .about("compile a mdx or md file to jsx.")
        .arg(Arg::new("file").help("the mdx or md file to compile.").required(true)),
    )
    .subcommand(
      Command::new("run-vue")
        .about("compile a mdx or md file to vue.")
        .arg(Arg::new("file").help("the mdx or md file to compile.").required(true)),
    )
    .get_matches();

  return matches;
}
