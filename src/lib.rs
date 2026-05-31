pub const DEFAULT_WIDTH: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
  Twitter,
  Mono,
}

impl Style {
  fn layout(self) -> &'static DuckLayout {
    match self {
      Style::Twitter => &TWITTER_LAYOUT,
      Style::Mono => &MONO_LAYOUT,
    }
  }
}

struct DuckLayout {
  header: &'static str,
  speech_prefix: &'static str,
  body: &'static str,
  continuation_gap: &'static str,
}

const TWITTER_LAYOUT: DuckLayout = DuckLayout {
  header: "<!--      _\n",
  speech_prefix: "       .__( . )< ",
  body: "        \\___)",
  continuation_gap: "     ",
};

const MONO_LAYOUT: DuckLayout = DuckLayout {
  header: "<!--       _\n",
  speech_prefix: "       .__(.)< ",
  body: "        \\___)",
  continuation_gap: "    ",
};

const FOOTER: &str = " ~~~~~~~~~~~~~~~~~~-->\n";

pub fn render(message: &str, width: usize) -> String {
  render_with_style(message, width, Style::Mono)
}

pub fn render_with_style(message: &str, width: usize, style: Style) -> String {
  let lines = wrap_message(message, width);

  render_duck(&lines, style.layout())
}

fn render_duck(lines: &[String], layout: &DuckLayout) -> String {
  let mut output = String::new();

  output.push_str(layout.header);

  match lines {
    [line] => write_single_line_message(&mut output, layout, line),
    [first, rest @ ..] => write_wrapped_message(&mut output, layout, first, rest),
    [] => write_single_line_message(&mut output, layout, ""),
  }

  output.push_str(FOOTER);
  output
}

fn write_single_line_message(output: &mut String, layout: &DuckLayout, line: &str) {
  output.push_str(layout.speech_prefix);
  output.push('(');
  output.push_str(line);
  output.push_str(")\n");
  output.push_str(layout.body);
  output.push('\n');
}

fn write_wrapped_message(output: &mut String, layout: &DuckLayout, first: &str, rest: &[String]) {
  output.push_str(layout.speech_prefix);
  output.push('(');
  output.push_str(first);
  output.push('\n');
  output.push_str(layout.body);
  output.push_str(layout.continuation_gap);
  write_continuation_lines(output, rest);
  output.push_str(")\n");
}

fn write_continuation_lines(output: &mut String, lines: &[String]) {
  for (index, line) in lines.iter().enumerate() {
    if index > 0 {
      output.push(' ');
    }

    output.push_str(line);
  }
}

fn wrap_message(message: &str, width: usize) -> Vec<String> {
  let width = width.max(1);
  let mut wrapped_lines = Vec::new();

  for source_line in message.lines() {
    wrap_source_line(source_line, width, &mut wrapped_lines);
  }

  if wrapped_lines.is_empty() {
    wrapped_lines.push(String::new());
  }

  wrapped_lines
}

fn wrap_source_line(source_line: &str, width: usize, wrapped_lines: &mut Vec<String>) {
  if source_line.is_empty() {
    wrapped_lines.push(String::new());
    return;
  }

  let mut line = String::new();

  for word in source_line.split_whitespace() {
    if text_width(word) > width {
      finish_line(&mut line, wrapped_lines);
      wrapped_lines.extend(split_long_word(word, width));
      continue;
    }

    if !word_fits_on_line(&line, word, width) {
      finish_line(&mut line, wrapped_lines);
    }

    append_word(&mut line, word);
  }

  finish_line(&mut line, wrapped_lines);
}

fn word_fits_on_line(line: &str, word: &str, width: usize) -> bool {
  line.is_empty() || text_width(line) + 1 + text_width(word) <= width
}

fn append_word(line: &mut String, word: &str) {
  if !line.is_empty() {
    line.push(' ');
  }

  line.push_str(word);
}

fn finish_line(line: &mut String, wrapped_lines: &mut Vec<String>) {
  if !line.is_empty() {
    wrapped_lines.push(std::mem::take(line));
  }
}

fn split_long_word(word: &str, width: usize) -> Vec<String> {
  let mut chunks = Vec::new();
  let mut current = String::new();

  for ch in word.chars() {
    current.push(ch);
    if text_width(&current) >= width {
      chunks.push(current);
      current = String::new();
    }
  }

  if !current.is_empty() {
    chunks.push(current);
  }

  chunks
}

fn text_width(value: &str) -> usize {
  value.chars().count()
}
