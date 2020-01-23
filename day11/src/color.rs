use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Color {
  Black,
  White,
  Grey
}

impl fmt::Debug for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Color::Black => write!(f, "  "),
      Color::White => write!(f, "\u{2588}\u{2588}"),
      Color::Grey => write!(f, "\u{2591}\u{2591}")
    }
  }
}

impl Color {
  pub fn code(&self) -> u32 {
    match self {
      Color::Black => 0,
      Color::White => 1,
      Color::Grey => 2
    }
  }

  pub fn from_code(code: u32) -> Color {
    match code {
      2 => Color::Grey,
      1 => Color::White,
      _ => Color::Black
    }
  }
}