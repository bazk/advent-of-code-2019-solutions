use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Tile {
  Wall,
  Empty,
  Oxygen,
  Unexplored,
  Start,
}

impl fmt::Display for Tile {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Tile::Wall => write!(f, "\u{2588}\u{2588}"),
      Tile::Empty => write!(f, "  "),
      Tile::Oxygen => write!(f, "oO"),
      Tile::Unexplored => write!(f, "\u{2591}\u{2591}"),
      Tile::Start => write!(f, "##"),
    }
  }
}

#[allow(dead_code)]
impl Tile {
  pub fn code(&self) -> u32 {
    match self {
      Tile::Wall => 0,
      Tile::Empty => 1,
      Tile::Oxygen => 2,
      Tile::Unexplored => 3,
      Tile::Start => 4,
    }
  }

  pub fn from_code(code: u32) -> Tile {
    match code {
      0 => Tile::Wall,
      1 => Tile::Empty,
      2 => Tile::Oxygen,
      3 => Tile::Unexplored,
      4 => Tile::Start,
      _ => panic!("unknown code"),
    }
  }
}
