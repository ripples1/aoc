use clap::Parser;
use std::{error::Error, path::PathBuf};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
  /// The input file path
  input: PathBuf,

  /// Challenge path to run. Example: "2023/1/1"
  #[arg(short, long)]
  challenge: PathBuf
}

fn main() -> Result<(), Box<dyn Error>> {
  let cli = Cli::parse();
  let input = cli.input.to_str().unwrap();
  let challenge = cli.challenge.to_str().unwrap();
  let contents = std::fs::read_to_string(input)?;

  println!("{}", challenges::run(&contents, challenge));
  Ok(())
}

