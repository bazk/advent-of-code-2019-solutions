use crate::grid::Grid;
use crate::point::Point;
use crate::tile::Tile;
use std::collections::HashSet;
use std::fmt;

const LEFT_ROTATION: f64 = std::f64::consts::FRAC_PI_2;
const RIGHT_ROTATION: f64 = -std::f64::consts::FRAC_PI_2;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
  Right,
  Left,
}

#[derive(Eq, PartialEq, Clone)]
pub enum Step {
  Forward(usize),
  TurnLeft,
  TurnRight,
  FunctionCall(String),
}

impl fmt::Debug for Step {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

impl Step {
  pub fn to_string(&self) -> String {
    match self {
      Step::Forward(n) => n.to_string(),
      Step::TurnLeft => String::from("L"),
      Step::TurnRight => String::from("R"),
      Step::FunctionCall(name) => name.clone(),
    }
  }
}

#[derive(Debug)]
pub struct PathFinder<'a> {
  grid: &'a Grid,
  visited: HashSet<Point>,
  position: Point,
  direction: Point,
}

impl<'a> PathFinder<'a> {
  pub fn new(grid: &'a Grid) -> Self {
    // find initial position and direction
    let position;
    let direction;
    if let Some(p) = grid.find(&Tile::RobotUp) {
      position = p;
      direction = Point::up();
    } else if let Some(p) = grid.find(&Tile::RobotRight) {
      position = p;
      direction = Point::right();
    } else if let Some(p) = grid.find(&Tile::RobotDown) {
      position = p;
      direction = Point::down();
    } else if let Some(p) = grid.find(&Tile::RobotLeft) {
      position = p;
      direction = Point::left();
    } else {
      panic!("robot not found on grid");
    }

    Self {
      grid: grid,
      visited: HashSet::new(),
      position,
      direction,
    }
  }

  pub fn find(&mut self) -> Vec<Step> {
    let mut steps = Vec::new();

    // explore the grid, adding new steps
    loop {
      if self.is_going_forward_possible() {
        // mark current position as visited
        self.visited.insert(self.position);

        // add a walk forward step
        steps.push(Step::Forward(1));
        self.position += self.direction;
      } else if self.explorable(&(self.position + self.direction.rotated(LEFT_ROTATION))) {
        // add a left turn
        steps.push(Step::TurnLeft);
        self.direction = self.direction.rotated(LEFT_ROTATION);
      } else if self.explorable(&(self.position + self.direction.rotated(RIGHT_ROTATION))) {
        // add a right turn
        steps.push(Step::TurnRight);
        self.direction = self.direction.rotated(RIGHT_ROTATION);
      } else {
        break;
      }
    }

    Self::simplify(&mut steps);

    steps
  }

  // Returns true if there is an explorable position that we
  // can reach by moving forward
  fn is_going_forward_possible(&self) -> bool {
    let one_step = self.position + self.direction;
    let two_steps = self.position + self.direction + self.direction;

    // returns true if taking one OR two steps forward leads to a valid and unexplored position
    self.explorable(&one_step) || (self.walkable(&one_step) && self.explorable(&two_steps))
  }

  // Returns true if it is possible to walk on "pos".
  // Does not care if we have already explored this "pos" or not.
  fn walkable(&self, pos: &Point) -> bool {
    (self.grid.get(pos) == Tile::Scaffold)
  }

  // Returns true if it is possible to walk on "pos" and we have
  // not visited this spot before.
  fn explorable(&self, pos: &Point) -> bool {
    (self.grid.get(pos) == Tile::Scaffold) && !self.visited.contains(&pos)
  }

  fn simplify(steps: &mut Vec<Step>) {
    let mut i = 1;
    while i < steps.len() {
      if let Step::Forward(prev) = steps[i - 1] {
        if let Step::Forward(cur) = steps[i] {
          steps[i - 1] = Step::Forward(prev + cur);
          steps.remove(i);
          continue;
        }
      }

      i += 1;
    }
  }
}
