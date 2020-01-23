use crate::color::Color;
use crate::point::Point;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Grid {
    painted_panels: HashMap<Point, Color>
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (mut begin, mut end) = self.bounding_box();

        if begin.x < -64 {
            begin.x = -64;
        }
        if begin.y < -64 {
            begin.y = -64;
        }
        if end.x > 64 {
            end.x = 64;
        }
        if end.y > 64 {
            end.y = 64;
        }

        for y in (begin.y..end.y).rev() {
            write!(f, " {:>2} ", y).unwrap();
            for x in begin.x..end.x {
                let pos = Point {
                    x: x as i32,
                    y: y as i32,
                };
                let color = self.painted_panels.get(&pos).unwrap_or(&Color::Black);
                write!(f, "{:?}", color).unwrap();
            }
            write!(f, "\n").unwrap();
        }

        write!(f, "    ").unwrap();
        for x in begin.x..end.x {
            write!(f, " {}", x % 10).unwrap();
        }
        write!(f, "\n").unwrap();

        write!(f, "")
    }
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            painted_panels: HashMap::new()
        }
    }

    pub fn get(&self, point: Point) -> Color {
        let color = self.painted_panels.get(&point).unwrap_or(&Color::Black);
        *color
    }

    pub fn paint(&mut self, point: Point, color: Color) {
        self.painted_panels.insert(point, color);
    }

    pub fn bounding_box(&self) -> (Point, Point) {
        let mut min = Point { x: 0, y: 0 };
        let mut max = Point { x: 0, y: 0 };

        for point in self.painted_panels.keys() {
            if *point <= min {
                min = *point;
            } else if *point >= max {
                max = *point;
            }
        }

        let margin = Point { x: 5, y: 5 };

        (min - margin, max + margin)
    }

    pub fn total_painted(&self) -> u32 {
        self.painted_panels.len() as u32
    }
}
