use crate::tile::Tile;
use std::fmt;
use std::slice::Iter;

#[derive(Debug)]
pub struct Grid {
  tiles: Vec<Vec<Tile>>,
}

impl fmt::Display for Grid {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for row in self.tiles.iter() {
      for tile in row.iter() {
        write!(f, "{}", tile).unwrap();
      }
      write!(f, "\n").unwrap();
    }

    write!(f, "")
  }
}

#[allow(dead_code)]
impl Grid {
  pub fn new(width: usize, height: usize) -> Grid {
    Grid {
      tiles: vec![vec![Tile::Empty; width]; height],
    }
  }

  pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
    self.tiles[y][x] = tile;
  }

  pub fn get(&self, x: usize, y: usize) -> Tile {
    self.tiles[y][x]
  }

  pub fn count(&self, match_tile: Tile) -> usize {
    let mut count = 0;

    for row in self.tiles.iter() {
      for tile in row.iter() {
        if *tile == match_tile {
          count += 1;
        }
      }
    }

    count
  }

  pub fn find(&self, match_tile: Tile) -> Option<(usize, usize)> {
    for (y, row) in self.tiles.iter().enumerate() {
      for (x, tile) in row.iter().enumerate() {
        if *tile == match_tile {
          return Some((x, y));
        }
      }
    }

    None
  }

  pub fn iter(&self) -> Iter<Vec<Tile>> {
    self.tiles.iter()
  }
}
