#![allow(dead_code)]

use std::{fs, io::Read};

pub struct Processor {
    /// Location in program
    pc: usize,
    /// Address pointer to tape
    ap: usize,
    tape: Vec<u8>,
    parsed_program: Vec<Opcodes>,
}

#[derive(Clone, Copy, Debug)]
pub enum Opcodes {
    Right,
    Left,
    Inc,
    Dec,
    Disp,
    Read,
    JpZero(usize),
    JpNonZero(usize),
}

impl Processor {
    pub fn new(program: &str, tape_len: usize) -> Self {
        Self {
            pc: 0,
            ap: 0,
            tape: vec![0; tape_len],
            parsed_program: Self::parse(program),
        }
    }

    fn parse(program: &str) -> Vec<Opcodes> {
        let mut res = vec![];
        for (pc, c) in program.chars().enumerate() {
            match c {
                '>' => res.push(Opcodes::Right),
                '<' => res.push(Opcodes::Left),
                '+' => res.push(Opcodes::Inc),
                '-' => res.push(Opcodes::Dec),
                '.' => res.push(Opcodes::Disp),
                ',' => res.push(Opcodes::Read),
                '[' => {
                    let mut stack = vec![];
                    let mut i = pc;
                    while i <= program.len() {
                        match program.chars().nth(i).unwrap() {
                            '[' => stack.push('['),
                            ']' => {
                                stack.pop();
                                if stack.is_empty() {
                                    res.push(Opcodes::JpZero(i));
                                    break;
                                }
                            }
                            _ => {}
                        }
                        i += 1;
                    }
                }
                ']' => {
                    let mut stack = vec![];
                    let mut i: i64 = pc as u64 as i64;
                    while i >= 0 {
                        match program.chars().nth(i as u64 as usize).unwrap() {
                            ']' => stack.push(']'),
                            '[' => {
                                stack.pop();
                                if stack.is_empty() {
                                    res.push(Opcodes::JpNonZero(i as u64 as usize));
                                    break;
                                }
                            }
                            _ => {}
                        }
                        i -= 1;
                    }
                }
                _ => { /* Spec is to ignore other characters */ }
            }
        }
        res
    }

    fn run(&mut self) {
        while let Some(op) = self.get_op() {
            self.parsed_step(op);
        }
    }

    fn parsed_step(&mut self, op: Opcodes) {
        match op {
            Opcodes::Right => self.ap += 1,
            Opcodes::Left => self.ap -= 1,
            Opcodes::Inc => self.tape[self.ap] = self.tape[self.ap].wrapping_add(1),
            Opcodes::Dec => self.tape[self.ap] = self.tape[self.ap].wrapping_sub(1),
            Opcodes::Disp => print!("{}", self.tape[self.ap] as char),
            Opcodes::Read => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin().read_exact(&mut input).unwrap();
                self.tape[self.ap] = input[0];
            }
            Opcodes::JpZero(dst) => {
                if self.tape[self.ap] == 0 {
                    self.pc = dst;
                }
            }
            Opcodes::JpNonZero(dst) => {
                if self.tape[self.ap] != 0 {
                    self.pc = dst;
                }
            }
        }
    }

    fn get_op(&mut self) -> Option<Opcodes> {
        let op = self.parsed_program.get(self.pc);
        let res = match op {
            Some(op) => Some(*op),
            None => None,
        };
        self.pc += 1;
        res
    }
}

fn main() {
    let prog = fs::read_to_string("hello_world.bfk").unwrap();
    let mut proc = Processor::new(&prog, 10);
    proc.run();
}
