use std::collections::VecDeque;
use std::fmt;
use std::fs;

const DEBUG_INSTRUCTIONS: bool = false;

#[derive(Debug)]
pub enum State {
  Booting,
  Ready,
  Running,
  Interrupted,
  Halted,
}

impl Copy for State {}

impl Clone for State {
  fn clone(&self) -> State {
    *self
  }
}

pub struct Computer {
  state: State,
  memory: Box<[i64; 65536]>,
  input_buffer: VecDeque<i64>,
  output_buffer: VecDeque<i64>,
  pc: u64,
  fp: i64,
}

pub enum Interruption {
  Halt,
  Output,
  Input,
}

impl fmt::Debug for Computer {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Computer {{ state: {:?}, pc: {}, fp: {}, input: {:?}, output: {:?} }}",
      self.state, self.pc, self.fp, self.input_buffer, self.output_buffer
    )
  }
}

impl Clone for Computer {
  fn clone(&self) -> Computer {
    Computer {
      state: self.state,
      memory: self.memory.clone(),
      input_buffer: self.input_buffer.clone(),
      output_buffer: self.output_buffer.clone(),
      pc: self.pc,
      fp: self.fp,
    }
  }
}

#[allow(dead_code)]
impl Computer {
  pub fn new() -> Computer {
    Computer {
      state: State::Booting,
      memory: Box::new([0; 65536]),
      input_buffer: VecDeque::new(),
      output_buffer: VecDeque::new(),
      pc: 0,
      fp: 0,
    }
  }

  pub fn flash(&mut self, filename: &str) {
    let raw = fs::read_to_string(filename).expect("Failed to read input file");

    for (i, val) in raw.split(',').enumerate() {
      let n: i64 = val.parse().expect("Failed to parse value from input");
      self.memory[i] = n;
    }

    self.state = State::Ready;
  }

  pub fn dump(&self) {
    for b in self.memory[..128].iter() {
      print!("{} ", b);
    }
    println!();
  }

  pub fn get_state(&self) -> State {
    self.state
  }

  pub fn input(&mut self, value: i64) {
    self.input_buffer.push_back(value);
  }

  pub fn output(&mut self) -> Option<i64> {
    self.output_buffer.pop_front()
  }

  pub fn read(&self, index: u64) -> i64 {
    self.memory[index as usize]
  }

  pub fn write(&mut self, index: u64, value: i64) {
    self.memory[index as usize] = value;
  }

  fn get_read_addr(&self, index: u64) -> u64 {
    let param_mode = (self.read(self.pc) / i64::pow(10, (index + 2) as u32)) % 10;
    let param_addr = self.pc + index + 1;
    match param_mode {
      0 => self.read(param_addr) as u64,
      1 => param_addr as u64,
      2 => (self.fp + self.read(param_addr)) as u64,
      _ => 0,
    }
  }

  fn get_write_addr(&self, index: u64) -> u64 {
    let param_mode = (self.read(self.pc) / i64::pow(10, (index + 2) as u32)) % 10;
    let param_addr = self.pc + index + 1;
    match param_mode {
      0 => self.read(param_addr) as u64,
      1 => self.read(param_addr) as u64,
      2 => (self.fp + self.read(param_addr)) as u64,
      _ => 0,
    }
  }

  pub fn run(&mut self) -> Interruption {
    self.state = State::Running;

    loop {
      let opcode = self.read(self.pc);

      match opcode % 100 {
        1 => {
          let p1 = self.read(self.get_read_addr(0));
          let p2 = self.read(self.get_read_addr(1));
          let addr = self.get_write_addr(2);

          if DEBUG_INSTRUCTIONS {
            println!(
              " [{} {} {} {}] {} = {} + {}",
              opcode,
              self.read(self.pc + 1),
              self.read(self.pc + 2),
              self.read(self.pc + 3),
              addr,
              p1,
              p2
            );
          }

          self.write(addr, p1 + p2);
          self.pc += 4;
        }
        2 => {
          let p1 = self.read(self.get_read_addr(0));
          let p2 = self.read(self.get_read_addr(1));
          let addr = self.get_write_addr(2);

          if DEBUG_INSTRUCTIONS {
            println!(
              " [{} {} {} {}] {} = {} * {}",
              opcode,
              self.read(self.pc + 1),
              self.read(self.pc + 2),
              self.read(self.pc + 3),
              addr,
              p1,
              p2
            );
          }

          self.write(addr, p1 * p2);
          self.pc += 4;
        }
        3 => {
          // Opcode 3 takes a single integer as input and saves it
          // to the position given by its only parameter. For example,
          // the instruction 3,50 would take an input value and
          // store it at address 50.

          // input buffer exausted, interrupt to wait for input
          if self.input_buffer.len() == 0 {
            self.state = State::Interrupted;
            return Interruption::Input;
          }

          let addr = self.get_write_addr(0);

          let value = self
            .input_buffer
            .pop_front()
            .expect("Error reading input buffer");

          if DEBUG_INSTRUCTIONS {
            println!(
              " [{} {}] {} = input({})",
              opcode,
              self.read(self.pc + 1),
              addr,
              value
            );
          }

          self.write(addr, value);
          self.pc += 2;
        }
        4 => {
          // Opcode 4 outputs the value of its only parameter. For
          // example, the instruction 4,50 would output the value at
          // address 50.

          let value = self.read(self.get_read_addr(0));

          if DEBUG_INSTRUCTIONS {
            println!(" [{} {}] output({})", opcode, self.read(self.pc + 1), value);
          }

          self.output_buffer.push_back(value);
          self.pc += 2;
          self.state = State::Interrupted;
          return Interruption::Output;
        }
        5 => {
          // Opcode 5 is jump-if-true: if the first parameter is
          // non-zero, it sets the instruction pointer to the value
          // from the second parameter. Otherwise, it does nothing.

          let value = self.read(self.get_read_addr(0));
          let addr = self.read(self.get_read_addr(1));

          if DEBUG_INSTRUCTIONS {
            println!(
              " [{} {} {}] jump-if-true({}, addr={})",
              opcode,
              self.read(self.pc + 1),
              self.read(self.pc + 2),
              value,
              addr
            );
          }

          if value != 0 {
            self.pc = addr as u64;
          } else {
            self.pc += 3;
          }
        }
        6 => {
          // Opcode 6 is jump-if-false: if the first parameter is
          // zero, it sets the instruction pointer to the value
          // from the second parameter. Otherwise, it does nothing.

          let value = self.read(self.get_read_addr(0));
          let addr = self.read(self.get_read_addr(1));

          if DEBUG_INSTRUCTIONS {
            println!(
              " [{} {} {}] jump-if-false({}, addr={})",
              opcode,
              self.read(self.pc + 1),
              self.read(self.pc + 2),
              value,
              addr
            );
          }

          if value == 0 {
            self.pc = addr as u64;
          } else {
            self.pc += 3;
          }
        }
        7 => {
          // Opcode 7 is less than: if the first parameter is less
          // than the second parameter, it stores 1 in the position
          // given by the third parameter. Otherwise, it stores 0.

          let p1 = self.read(self.get_read_addr(0));
          let p2 = self.read(self.get_read_addr(1));
          let addr = self.get_write_addr(2);

          if DEBUG_INSTRUCTIONS {
            println!(
              " [{} {} {} {}] {} = {} < {}",
              opcode,
              self.read(self.pc + 1),
              self.read(self.pc + 2),
              self.read(self.pc + 3),
              addr,
              p1,
              p2
            );
          }

          self.write(addr, if p1 < p2 { 1 } else { 0 });
          self.pc += 4;
        }
        8 => {
          // Opcode 8 is equals: if the first parameter is equal to
          // the second parameter, it stores 1 in the position given
          // by the third parameter. Otherwise, it stores 0.

          let p1 = self.read(self.get_read_addr(0));
          let p2 = self.read(self.get_read_addr(1));
          let addr = self.get_write_addr(2);

          if DEBUG_INSTRUCTIONS {
            println!(
              " [{} {} {} {}] {} = {} == {}",
              opcode,
              self.read(self.pc + 1),
              self.read(self.pc + 2),
              self.read(self.pc + 3),
              addr,
              p1,
              p2
            );
          }

          self.write(addr, if p1 == p2 { 1 } else { 0 });
          self.pc += 4;
        }
        9 => {
          // Opcode 9 adjusts the relative base by the value of its only
          // parameter. The relative base increases (or decreases, if the
          // value is negative) by the value of the parameter.

          let value = self.read(self.get_read_addr(0));

          if DEBUG_INSTRUCTIONS {
            println!(
              " [{} {}] fp += {} ({})",
              opcode,
              self.read(self.pc + 1),
              value,
              self.fp + value
            );
          }

          self.fp += value;
          self.pc += 2;
        }
        99 => {
          if DEBUG_INSTRUCTIONS {
            println!(" [{}] halt", opcode);
          }

          self.state = State::Halted;
          return Interruption::Halt;
        }
        _ => {
          self.dump();
          panic!("Unknown opcode {} at address {}", opcode, self.pc);
        }
      }
    }
  }
}
