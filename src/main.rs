#![allow(dead_code)]

use std::{fs, io::Read};

pub struct Processor {
    /// Location in program
    pc: usize,
    /// Address pointer to tape
    ap: usize,
    tape: Vec<u8>,
    program: String,
}

impl Processor {
    pub fn new(program: &str, tape_len: usize) -> Self {
        Self {
            pc: 0,
            ap: 0,
            tape: vec![0; tape_len],
            program: program.to_owned(),
        }
    }

    fn run(&mut self) {
        while let Some(op) = self.get_opcode(self.pc) {
            self.step(op);
        }
    }

    fn step(&mut self, op: char) {
        self.pc += 1;
        match op {
            '>' => self.ap += 1,
            '<' => self.ap -= 1,
            '+' => self.tape[self.ap] = self.tape[self.ap].wrapping_add(1),
            '-' => self.tape[self.ap] = self.tape[self.ap].wrapping_sub(1),
            '.' => print!("{}", self.tape[self.ap] as char),
            ',' => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin().read_exact(&mut input).unwrap();
                self.tape[self.ap] = input[0];
            }
            '[' => {
                if self.tape[self.ap] == 0 {
                    let mut stack = vec![];
                    let mut i = self.pc;
                    loop {
                        match self.get_opcode(i).unwrap() {
                            '[' => stack.push('['),
                            ']' => {
                                stack.pop();
                                if stack.is_empty() {
                                    self.pc = i;
                                    break;
                                }
                            }
                            _ => {}
                        }
                        i += 1;
                    }
                }
            }
            ']' => {
                if self.tape[self.ap] != 0 {
                    let mut stack = vec![];
                    let mut i = self.pc;
                    loop {
                        match self.get_opcode(i).unwrap() {
                            ']' => stack.push(']'),
                            '[' => {
                                stack.pop();
                                if stack.is_empty() {
                                    self.pc = i;
                                    break;
                                }
                            }
                            _ => {}
                        }

                        i -= 1;
                    }
                }
            }
            _ => { /* Spec is to ignore other characters */ }
        }
    }

    fn get_opcode(&self, pc: usize) -> Option<char> {
        self.program.chars().nth(pc)
    }
}

fn main() {
    let prog = fs::read_to_string("hello_world.bfk").unwrap();
    let mut proc = Processor::new(&prog, 10);
    proc.run();
}
