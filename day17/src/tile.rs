use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Tile {
  Empty,
  Scaffold,
  RobotUp,
  RobotRight,
  RobotDown,
  RobotLeft,
}

impl fmt::Display for Tile {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Tile::Empty => write!(f, "  "),
      Tile::Scaffold => write!(f, "\u{2588}\u{2588}"),
      Tile::RobotUp => write!(f, "\u{25E2}\u{25E3}"),
      Tile::RobotRight => write!(f, "\u{25BA} "),
      Tile::RobotDown => write!(f, "\u{25E5}\u{25E4}"),
      Tile::RobotLeft => write!(f, " \u{25C0}"),
    }
  }
}

#[allow(dead_code)]
impl Tile {
  pub fn code(&self) -> u32 {
    match self {
      Tile::Empty => 46,
      Tile::Scaffold => 35,
      Tile::RobotUp => 94,
      Tile::RobotRight => 62,
      Tile::RobotDown => 118,
      Tile::RobotLeft => 60,
    }
  }

  pub fn from_code(code: u32) -> Tile {
    match code {
      46 => Tile::Empty,
      35 => Tile::Scaffold,
      94 => Tile::RobotUp,
      62 => Tile::RobotRight,
      118 => Tile::RobotDown,
      60 => Tile::RobotLeft,
      _ => panic!("unknown code"),
    }
  }
}
