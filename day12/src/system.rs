use std::fs;
use std::fmt;
use crate::vector3d::Vector3;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Axis {
  X,
  Y,
  Z
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct Moon {
  pub position: Vector3,
  pub velocity: Vector3
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct System {
  pub moons: Vec<Moon>
}

impl fmt::Debug for System {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for moon in self.moons.iter() {
      write!(f, "pos={:?}, vel={:?}\n", moon.position, moon.velocity).unwrap();
    }

    write!(f, "")
  }
}

impl System {
  pub fn load(filename: &str) -> System {
    let raw = fs::read_to_string(filename).expect(&format!("Failed to read input file: {}", filename));

    let mut moons = Vec::new();

    for (i, val) in raw.split('\n').enumerate() {
      let position: Vector3 = val.trim().parse()
        .expect(&format!("Failed to parse moon position in line {}", i+1));

      moons.push(Moon {
        position,
        velocity: Vector3 { x: 0, y: 0, z: 0 }
      });
    }

    System {
      moons
    }
  }

  pub fn total_energy(&self) -> i32 {
    let mut total = 0;

    for m in self.moons.iter() {
      let pot = m.position.x.abs() + m.position.y.abs() + m.position.z.abs();
      let kin = m.velocity.x.abs() + m.velocity.y.abs() + m.velocity.z.abs();
      total += pot * kin;
    }

    total
  }

  pub fn get_axis(&self, axis: &Axis) -> Vec<(i32, i32)> {
    let mut res = Vec::new();

    for moon in self.moons.iter() {
      let v = match axis {
        Axis::X => (moon.position.x, moon.velocity.x),
        Axis::Y => (moon.position.y, moon.velocity.y),
        Axis::Z => (moon.position.z, moon.velocity.z)
      };

      res.push(v);
    }

    res
  }

  pub fn step(&mut self) {
    for i in 0..self.moons.len() {
      for j in 0..self.moons.len() {
        if i == j {
          continue;
        }

        let g = Self::gravity(&self.moons[i], &self.moons[j]);
        self.moons[i].velocity += g;
      }
    }

    for m in self.moons.iter_mut() {
      m.position += m.velocity;
    }
  }

  fn gravity(m1: &Moon, m2: &Moon) -> Vector3 {
    fn cmp(a: i32, b: i32) -> i32 {
      if a < b {
        1
      } else if a > b {
        -1
      } else {
        0
      }
    }

    Vector3 {
      x: cmp(m1.position.x, m2.position.x),
      y: cmp(m1.position.y, m2.position.y),
      z: cmp(m1.position.z, m2.position.z)
    }
  }
}
