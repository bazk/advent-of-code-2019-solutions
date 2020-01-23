use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Tile {
  Empty,
  Wall,
  Block,
  Paddle,
  Ball,
}

impl fmt::Display for Tile {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Tile::Empty => write!(f, "  "),
      Tile::Wall => write!(f, "\u{2588}\u{2588}"),
      Tile::Block => write!(f, "\u{2591}\u{2591}"),
      Tile::Paddle => write!(f, "\u{2591}\u{2591}"),
      Tile::Ball => write!(f, "\u{2591}\u{2591}"),
    }
  }
}

#[allow(dead_code)]
impl Tile {
  pub fn code(&self) -> u32 {
    match self {
      Tile::Empty => 0,
      Tile::Wall => 1,
      Tile::Block => 2,
      Tile::Paddle => 3,
      Tile::Ball => 4,
    }
  }

  pub fn from_code(code: u32) -> Tile {
    match code {
      4 => Tile::Ball,
      3 => Tile::Paddle,
      2 => Tile::Block,
      1 => Tile::Wall,
      _ => Tile::Empty,
    }
  }
}
