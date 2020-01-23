use std::fs;
use std::collections::VecDeque;

pub enum State {
    Booting,
    Ready,
    Running,
    Interrupted,
    Halted
}

impl Copy for State { }

impl Clone for State {
    fn clone(&self) -> State {
        *self
    }
}

pub struct Computer {
    state: State,
    memory: [i32; 1024],
    input_buffer: VecDeque<i32>,
    output_buffer: VecDeque<i32>,
    pc: usize
}

pub enum Interruption {
    Halt,
    Input,
    Output
}

impl Clone for Computer {
    fn clone(&self) -> Computer {
        Computer {
            state: self.state,
            memory: self.memory,
            input_buffer: self.input_buffer.clone(),
            output_buffer: self.output_buffer.clone(),
            pc: self.pc
        }
    }
}

impl Computer {
    #[allow(dead_code)]
    pub fn new() -> Computer {
        Computer {
            state: State::Booting,
            memory: [0; 1024],
            input_buffer: VecDeque::new(),
            output_buffer: VecDeque::new(),
            pc: 0
        }
    }

    #[allow(dead_code)]
    pub fn flash(&mut self, filename: &str) {
        let raw = fs::read_to_string(filename).expect("Failed to read input file");

        for (i, val) in raw.split(',').enumerate() {
            let n: i32 = val.parse().expect("Failed to parse value from input");
            self.memory[i] = n;
        }

        self.state = State::Ready;
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        for b in self.memory.iter() {
            print!("{} ", b);
        }
        println!();
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    #[allow(dead_code)]
    pub fn read(&self, index: usize) -> i32 {
        self.memory[index]
    }

    #[allow(dead_code)]
    pub fn write(&mut self, index: usize, value: i32) {
        self.memory[index] = value;
    }

    #[allow(dead_code)]
    pub fn input(&mut self, value: i32) {
        self.input_buffer.push_back(value);
    }

    #[allow(dead_code)]
    pub fn output(&mut self) -> Option<i32> {
        self.output_buffer.pop_front()
    }

    #[allow(dead_code)]
    pub fn run(&mut self) -> Interruption {
        self.state = State::Running;

        loop {
            let opcode = self.memory[self.pc] % 100;

            let param_1_mode = (self.memory[self.pc] / 100) % 10;
            let param_2_mode = (self.memory[self.pc] / 1000) % 10;

            match opcode {
                1 => {
                    let t1 = if param_1_mode == 0 {
                        self.memory[self.memory[self.pc+1] as usize]
                    } else {
                        self.memory[self.pc+1]
                    };
                    let t2 = if param_2_mode == 0 {
                        self.memory[self.memory[self.pc+2] as usize]
                    } else {
                        self.memory[self.pc+2]
                    };

                    self.memory[self.memory[self.pc+3] as usize] = t1 + t2;
                    self.pc += 4;
                },
                2 => {
                    let t1 = if param_1_mode == 0 {
                        self.memory[self.memory[self.pc+1] as usize]
                    } else {
                        self.memory[self.pc+1]
                    };
                    let t2 = if param_2_mode == 0 {
                        self.memory[self.memory[self.pc+2] as usize]
                    } else {
                        self.memory[self.pc+2]
                    };

                    self.memory[self.memory[self.pc+3] as usize] = t1 * t2;
                    self.pc += 4;
                },
                3 => {
                    // Opcode 3 takes a single integer as input and saves it
                    // to the position given by its only parameter. For example,
                    // the instruction 3,50 would take an input value and
                    // store it at address 50.

                    let addr = self.memory[self.pc+1] as usize;

                    self.pc += 2;

                    if self.input_buffer.len() > 0 {
                        self.memory[addr] = self.input_buffer.pop_front()
                            .expect("Error reading input buffer");
                    } else {
                        self.state = State::Interrupted;
                        return Interruption::Input;
                    }
                },
                4 => {
                    // Opcode 4 outputs the value of its only parameter. For
                    // example, the instruction 4,50 would output the value at
                    // address 50.

                    let value = if param_1_mode == 0 {
                        self.memory[self.memory[self.pc+1] as usize]
                    } else {
                        self.memory[self.pc+1]
                    };

                    self.output_buffer.push_back(value);
                    self.pc += 2;

                    self.state = State::Interrupted;
                    return Interruption::Output;
                },
                5 => {
                    // Opcode 5 is jump-if-true: if the first parameter is
                    // non-zero, it sets the instruction pointer to the value
                    // from the second parameter. Otherwise, it does nothing.

                    let value = if param_1_mode == 0 {
                        self.memory[self.memory[self.pc+1] as usize]
                    } else {
                        self.memory[self.pc+1]
                    };

                    let addr = if param_2_mode == 0 {
                        self.memory[self.memory[self.pc+2] as usize]
                    } else {
                        self.memory[self.pc+2]
                    };

                    if value != 0 {
                        self.pc = addr as usize;
                    } else {
                        self.pc += 3;
                    }
                },
                6 => {
                    // Opcode 6 is jump-if-false: if the first parameter is
                    // zero, it sets the instruction pointer to the value
                    // from the second parameter. Otherwise, it does nothing.

                    let value = if param_1_mode == 0 {
                        self.memory[self.memory[self.pc+1] as usize]
                    } else {
                        self.memory[self.pc+1]
                    };

                    let addr = if param_2_mode == 0 {
                        self.memory[self.memory[self.pc+2] as usize]
                    } else {
                        self.memory[self.pc+2]
                    };

                    if value == 0 {
                        self.pc = addr as usize;
                    } else {
                        self.pc += 3;
                    }
                },
                7 => {
                    // Opcode 7 is less than: if the first parameter is less
                    // than the second parameter, it stores 1 in the position
                    // given by the third parameter. Otherwise, it stores 0.

                    let p1 = if param_1_mode == 0 {
                        self.memory[self.memory[self.pc+1] as usize]
                    } else {
                        self.memory[self.pc+1]
                    };
                    let p2 = if param_2_mode == 0 {
                        self.memory[self.memory[self.pc+2] as usize]
                    } else {
                        self.memory[self.pc+2]
                    };

                    self.memory[self.memory[self.pc+3] as usize] = if p1 < p2 { 1 } else { 0 };
                    self.pc += 4;
                },
                8 => {
                    // Opcode 8 is equals: if the first parameter is equal to
                    // the second parameter, it stores 1 in the position given
                    // by the third parameter. Otherwise, it stores 0.

                    let p1 = if param_1_mode == 0 {
                        self.memory[self.memory[self.pc+1] as usize]
                    } else {
                        self.memory[self.pc+1]
                    };
                    let p2 = if param_2_mode == 0 {
                        self.memory[self.memory[self.pc+2] as usize]
                    } else {
                        self.memory[self.pc+2]
                    };

                    self.memory[self.memory[self.pc+3] as usize] = if p1 == p2 { 1 } else { 0 };
                    self.pc += 4;
                },
                99 => {
                    self.state = State::Halted;
                    return Interruption::Halt;
                },
                _ => {
                    self.dump();
                    panic!("Unknown opcode {} at address {}", self.memory[self.pc], self.pc);
                }
            }
        }
    }
}
