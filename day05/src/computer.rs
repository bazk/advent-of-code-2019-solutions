use std::fs;
use std::io;

pub struct Computer {
  memory: [i32; 1024]
}

impl Computer {
    #[allow(dead_code)]
    pub fn new() -> Computer {
        Computer {
            memory: [0; 1024],
        }
    }

    #[allow(dead_code)]
    pub fn flash(&mut self, filename: &str) {
        let raw = fs::read_to_string(filename).expect("Failed to read input file");

        for (i, val) in raw.split(',').enumerate() {
            let n: i32 = val.parse().expect("Failed to parse value from input");
            self.memory[i] = n;
        }
    }

    #[allow(dead_code)]
    pub fn dump(&mut self) {
        for b in self.memory.iter() {
            print!("{} ", b);
        }
        println!();
    }

    #[allow(dead_code)]
    pub fn read(&mut self, index: usize) -> i32 {
        self.memory[index]
    }

    #[allow(dead_code)]
    pub fn write(&mut self, index: usize, value: i32) {
        self.memory[index] = value;
    }

    #[allow(dead_code)]
    pub fn run(&mut self) {
        let mut pc: usize = 0;

        loop {
            let opcode = self.memory[pc] % 100;

            let param_1_mode = (self.memory[pc] / 100) % 10;
            let param_2_mode = (self.memory[pc] / 1000) % 10;
            // let param_3_mode = (self.memory[pc] / 10000) % 10;

            match opcode {
                1 => {
                    let t1 = if param_1_mode == 0 {
                        self.memory[self.memory[pc+1] as usize]
                    } else {
                        self.memory[pc+1]
                    };
                    let t2 = if param_2_mode == 0 {
                        self.memory[self.memory[pc+2] as usize]
                    } else {
                        self.memory[pc+2]
                    };

                    self.memory[self.memory[pc+3] as usize] = t1 + t2;
                    pc += 4;
                },
                2 => {
                    let t1 = if param_1_mode == 0 {
                        self.memory[self.memory[pc+1] as usize]
                    } else {
                        self.memory[pc+1]
                    };
                    let t2 = if param_2_mode == 0 {
                        self.memory[self.memory[pc+2] as usize]
                    } else {
                        self.memory[pc+2]
                    };

                    self.memory[self.memory[pc+3] as usize] = t1 * t2;
                    pc += 4;
                },
                3 => {
                    // Opcode 3 takes a single integer as input and saves it
                    // to the position given by its only parameter. For example,
                    // the instruction 3,50 would take an input value and
                    // store it at address 50.

                    let mut buffer = String::new();
                    io::stdin().read_line(&mut buffer)
                        .expect("Failed to read input");

                    let value: i32 = buffer.trim().parse()
                        .expect("Invalid number from input");

                    let addr = self.memory[pc+1] as usize;

                    self.memory[addr] = value;
                    pc += 2;
                },
                4 => {
                    // Opcode 4 outputs the value of its only parameter. For
                    // example, the instruction 4,50 would output the value at
                    // address 50.

                    let value = if param_1_mode == 0 {
                        self.memory[self.memory[pc+1] as usize]
                    } else {
                        self.memory[pc+1]
                    };

                    println!("{}", value);
                    pc += 2;
                },
                5 => {
                    // Opcode 5 is jump-if-true: if the first parameter is
                    // non-zero, it sets the instruction pointer to the value
                    // from the second parameter. Otherwise, it does nothing.

                    let value = if param_1_mode == 0 {
                        self.memory[self.memory[pc+1] as usize]
                    } else {
                        self.memory[pc+1]
                    };

                    let addr = if param_2_mode == 0 {
                        self.memory[self.memory[pc+2] as usize]
                    } else {
                        self.memory[pc+2]
                    };

                    if value != 0 {
                        pc = addr as usize;
                    } else {
                        pc += 3;
                    }
                },
                6 => {
                    // Opcode 6 is jump-if-false: if the first parameter is
                    // zero, it sets the instruction pointer to the value
                    // from the second parameter. Otherwise, it does nothing.

                    let value = if param_1_mode == 0 {
                        self.memory[self.memory[pc+1] as usize]
                    } else {
                        self.memory[pc+1]
                    };

                    let addr = if param_2_mode == 0 {
                        self.memory[self.memory[pc+2] as usize]
                    } else {
                        self.memory[pc+2]
                    };

                    if value == 0 {
                        pc = addr as usize;
                    } else {
                        pc += 3;
                    }
                },
                7 => {
                    // Opcode 7 is less than: if the first parameter is less
                    // than the second parameter, it stores 1 in the position
                    // given by the third parameter. Otherwise, it stores 0.

                    let p1 = if param_1_mode == 0 {
                        self.memory[self.memory[pc+1] as usize]
                    } else {
                        self.memory[pc+1]
                    };
                    let p2 = if param_2_mode == 0 {
                        self.memory[self.memory[pc+2] as usize]
                    } else {
                        self.memory[pc+2]
                    };

                    self.memory[self.memory[pc+3] as usize] = if p1 < p2 { 1 } else { 0 };
                    pc += 4;
                },
                8 => {
                    // Opcode 8 is equals: if the first parameter is equal to
                    // the second parameter, it stores 1 in the position given
                    // by the third parameter. Otherwise, it stores 0.

                    let p1 = if param_1_mode == 0 {
                        self.memory[self.memory[pc+1] as usize]
                    } else {
                        self.memory[pc+1]
                    };
                    let p2 = if param_2_mode == 0 {
                        self.memory[self.memory[pc+2] as usize]
                    } else {
                        self.memory[pc+2]
                    };

                    self.memory[self.memory[pc+3] as usize] = if p1 == p2 { 1 } else { 0 };
                    pc += 4;
                },
                99 => return,
                _ => {
                    self.dump();
                    panic!("Unknown opcode {} at address {}", self.memory[pc], pc);
                }
            }
        }
    }
}
