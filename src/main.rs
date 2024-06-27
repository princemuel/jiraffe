use anyhow::{self, Context};
use clap::Parser;
use std::{self, fs, io, path};

#[derive(Parser)]
struct Cli {
  pattern: String,
  path: path::PathBuf,
}

fn main() -> anyhow::Result<()> {
  let args = Cli::parse();

  let result = fs::read_to_string(&args.path);
  let content = result.with_context(|| {
    format!("could not read file `{}`", args.path.display())
  })?;

  println!("file content: {}", content);

  find_matches(&content, &args.pattern, &mut io::stdout());

  Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl io::Write) {
  for line in content.lines() {
    if line.contains(pattern) {
      writeln!(writer, "{}", line);
    }
  }
}

#[test]
fn find_a_match() {
  let mut result = Vec::new();
  find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
  assert_eq!(result, b"lorem ipsum\n");
}
