use crate::point::Point;
use crate::tile::Tile;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Grid {
  tiles: HashMap<Point, Tile>,
}

impl fmt::Display for Grid {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let (mut begin, mut end) = self.bounding_box();

    if begin.x < -32 {
      begin.x = -32;
    }
    if begin.y < -32 {
      begin.y = -32;
    }
    if end.x > 32 {
      end.x = 32;
    }
    if end.y > 32 {
      end.y = 32;
    }

    for y in (begin.y..end.y).rev() {
      write!(f, " {:>4} ", y).unwrap();
      for x in begin.x..end.x {
        let pos = Point {
          x: x as i32,
          y: y as i32,
        };
        let tile = if x == 0 && y == 0 {
          Tile::Start
        } else {
          *self.tiles.get(&pos).unwrap_or(&Tile::Unexplored)
        };
        write!(f, "{}", tile).unwrap();
      }
      write!(f, "\n").unwrap();
    }

    write!(f, "      ").unwrap();
    for x in begin.x..end.x {
      write!(f, "{:>2}", x % 10).unwrap();
    }
    write!(f, "\n").unwrap();

    write!(f, "")
  }
}

impl Grid {
  pub fn new() -> Grid {
    Grid {
      tiles: HashMap::new(),
    }
  }

  pub fn set(&mut self, point: Point, tile: Tile) {
    self.tiles.insert(point, tile);
  }

  fn bounding_box(&self) -> (Point, Point) {
    let mut min = Point { x: 0, y: 0 };
    let mut max = Point { x: 0, y: 0 };

    for &point in self.tiles.keys() {
      if point.x <= min.x {
        min.x = point.x;
      }
      if point.y <= min.y {
        min.y = point.y;
      }
      if point.x >= max.x {
        max.x = point.x;
      }
      if point.y >= max.y {
        max.y = point.y;
      }
    }

    let margin = Point { x: 3, y: 3 };

    (min - margin, max + margin)
  }
}
