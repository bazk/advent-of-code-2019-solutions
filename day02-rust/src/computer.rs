use std::fs;

pub struct Computer {
  memory: [i32; 1024]
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            memory: [0; 1024],
        }
    }

    pub fn flash(&mut self, filename: &str) {
        let raw = fs::read_to_string(filename).expect("Failed to read input file");

        for (i, val) in raw.split(',').enumerate() {
            let n: i32 = val.parse().expect("Failed to parse value from input");
            self.memory[i] = n;
        }
    }

    pub fn read(&mut self, index: usize) -> i32 {
        self.memory[index]
    }

    pub fn write(&mut self, index: usize, value: i32) {
        self.memory[index] = value;
    }

    pub fn run(&mut self) {
        let mut pc: usize = 0;

        loop {
            match self.memory[pc] {
                1 => {
                    let t1_addr = self.memory[pc+1] as usize;
                    let t2_addr = self.memory[pc+2] as usize;
                    let res_addr = self.memory[pc+3] as usize;
                    self.memory[res_addr] = self.memory[t1_addr] + self.memory[t2_addr];
                    pc += 4;
                },
                2 => {
                    let t1_addr = self.memory[pc+1] as usize;
                    let t2_addr = self.memory[pc+2] as usize;
                    let res_addr = self.memory[pc+3] as usize;
                    self.memory[res_addr] = self.memory[t1_addr] * self.memory[t2_addr];
                    pc += 4;
                },
                99 => return,
                _ => panic!("Unknown opcode: {}", self.memory[pc])
            }
        }
    }
}
