use std::io::{stdout, Write};
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

pub struct Writer {
  stdout: termion::input::MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>,
}

impl Writer {
  pub fn new() -> Self {
    Self {
      stdout: MouseTerminal::from(stdout().into_raw_mode().unwrap()),
    }
  }

  fn styled_input<C>(&mut self, input: &char, c: C)
  where
    C: color::Color,
  {
    write!(
      self.stdout,
      "{}{}{}{}",
      color::Fg(c),
      style::Underline,
      input,
      style::Reset
    );
  }

  pub fn input(&mut self, input: &char) {
    write!(self.stdout, "{}", input);
  }

  pub fn green_input(&mut self, input: &char) {
    self.styled_input(input, color::Green);
  }

  pub fn yellow_input(&mut self, input: &char) {
    self.styled_input(input, color::Yellow);
  }

  pub fn red_input(&mut self, input: &char) {
    self.styled_input(input, color::Red);
  }

  fn styled_guess<B, F>(&mut self, input: &char, bg: B, fg: F)
  where
    B: color::Color,
    F: color::Color,
  {
    write!(
      self.stdout,
      "{}{}{}{}",
      color::Bg(bg),
      color::Fg(fg),
      input,
      style::Reset,
    );
  }

  pub fn guess(&mut self, input: &char) {
    self.styled_guess(input, color::Reset, color::Reset);
  }

  pub fn green_guess(&mut self, input: &char) {
    self.styled_guess(input, color::Green, color::Black);
  }

  pub fn yellow_guess(&mut self, input: &char) {
    self.styled_guess(input, color::Yellow, color::Black);
  }

  pub fn line(&mut self, value: &str) {
    write!(self.stdout, "\r{}\n", value).unwrap();
  }

  pub fn newline(&mut self) {
    self.line("");
  }

  pub fn flush(&mut self) {
    self.stdout.flush().unwrap();
  }

  pub fn backspace(&mut self) {
    write!(self.stdout, "{}{}{}", cursor::Left(1), " ", cursor::Left(1));
  }

  pub fn clear(&mut self) {
    write!(self.stdout, "{}\r", clear::CurrentLine);
  }
}
