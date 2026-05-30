use std::num::NonZeroUsize;

use clap::Parser;
use ducksay::{DEFAULT_WIDTH, Style, render_with_style};

const DEFAULT_MESSAGE: &str = "I am Waddles";

#[derive(Debug, Parser)]
#[command(
  name = "ducksay",
  version,
  about = "Make Waddles say things",
  long_about = "Make Waddles say things.\n\nPass a message as arguments. Without a message, Waddles introduces himself."
)]
struct Cli {
  #[arg(
    short,
    long,
    default_value_t = NonZeroUsize::new(DEFAULT_WIDTH).expect("default width is non-zero"),
    help = "Wrap speech text at N columns"
  )]
  width: NonZeroUsize,

  #[arg(
    long,
    help = "Use plain monospace output without the Twitter workaround"
  )]
  mono: bool,

  #[arg(
    value_name = "MESSAGE",
    trailing_var_arg = true,
    help = "Text for Waddles to say"
  )]
  message: Vec<String>,
}

fn main() {
  let cli = Cli::parse();
  let message = cli.message.join(" ");

  let message = if message.trim().is_empty() {
    DEFAULT_MESSAGE
  } else {
    &message
  };

  let style = if cli.mono {
    Style::Mono
  } else {
    Style::Twitter
  };

  print!("{}", render_with_style(&message, cli.width.get(), style));
}
