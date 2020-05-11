use crate::computer::{Computer, Interruption};
use crate::path::Step;

pub struct Robot {
  computer: Computer,
}

impl Robot {
  pub fn new(program: &str) -> Self {
    let mut computer = Computer::new();
    computer.flash(program);
    computer.write(0, 2);
    Self { computer }
  }

  pub fn run(&mut self, routines: &Vec<Vec<Step>>) -> i64 {
    for routine in routines.iter() {
      for idx in 0..routine.len() {
        for &b in routine[idx].to_string().as_bytes() {
          self.computer.input(b as i64);
        }

        if idx < routine.len() - 1 {
          self.computer.input(44); // comma
        }
      }

      self.computer.input(10); // new line
    }

    // answer to coninuous video feed
    self.computer.input('n' as i64);
    self.computer.input(10); // new line

    let mut result: Vec<u8> = Vec::new();

    loop {
      match self.computer.run() {
        Interruption::Output => {
          let output = self
            .computer
            .output()
            .expect("Failed to retrieve status from computer");

          result.push(output as u8);
        }
        Interruption::Input => {
          panic!("computer is asking for input but none was given");
        }
        Interruption::Halt => {
          println!("result = {:?}", result);
          return String::from_utf8(result).unwrap().parse().unwrap();
        }
      }
    }
  }
}
