use std::{collections::HashMap, fs, io::Read};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    program: String,

    #[arg(short, long)]
    numeric_output: bool,
}

pub struct Processor {
    /// Location in program
    pc: usize,
    /// Address pointer to tape
    ap: usize,
    tape: Vec<u8>,
    parsed_program: Vec<Opcode>,
    numeric_output: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum Opcode {
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
    pub fn new(program: &str, numeric_output: bool, tape_len: usize) -> Self {
        Self {
            pc: 0,
            ap: 0,
            tape: vec![0; tape_len],
            parsed_program: Self::parse(program),
            numeric_output,
        }
    }

    fn parse(program: &str) -> Vec<Opcode> {
        let mut res = vec![];
        let mut stack = vec![];
        let mut open_to_close = HashMap::new();
        let mut close_to_open = HashMap::new();

        // First pass to compute bracket matches
        for (pc, c) in program.chars().enumerate() {
            match c {
                '[' => stack.push(pc),
                ']' => {
                    let matching_bracket = stack.pop().expect("malformed program");
                    open_to_close.insert(matching_bracket, pc);
                    close_to_open.insert(pc, matching_bracket);
                }
                _ => {}
            }
        }

        // Second pass to build the parsed program
        for (pc, c) in program.chars().enumerate() {
            match c {
                '>' => res.push(Opcode::Right),
                '<' => res.push(Opcode::Left),
                '+' => res.push(Opcode::Inc),
                '-' => res.push(Opcode::Dec),
                '.' => res.push(Opcode::Disp),
                ',' => res.push(Opcode::Read),
                '[' => res.push(Opcode::JpZero(
                    *open_to_close.get(&pc).expect("malformed program"),
                )),
                ']' => res.push(Opcode::JpNonZero(
                    *close_to_open.get(&pc).expect("malformed program"),
                )),
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

    fn parsed_step(&mut self, op: Opcode) {
        match op {
            Opcode::Right => self.ap += 1,
            Opcode::Left => self.ap -= 1,
            Opcode::Inc => self.tape[self.ap] = self.tape[self.ap].wrapping_add(1),
            Opcode::Dec => self.tape[self.ap] = self.tape[self.ap].wrapping_sub(1),
            Opcode::Disp => {
                if self.numeric_output {
                    println!("{:#04x}", self.tape[self.ap])
                } else {
                    print!("{}", self.tape[self.ap] as char)
                }
            }
            Opcode::Read => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin().read_exact(&mut input).unwrap();
                self.tape[self.ap] = input[0];
            }
            Opcode::JpZero(dst) => {
                if self.tape[self.ap] == 0 {
                    self.pc = dst;
                }
            }
            Opcode::JpNonZero(dst) => {
                if self.tape[self.ap] != 0 {
                    self.pc = dst;
                }
            }
        }
    }

    fn get_op(&mut self) -> Option<Opcode> {
        self.pc += 1;
        self.parsed_program.get(self.pc - 1).copied()
    }
}

fn main() {
    let args = Args::parse();
    let prog = fs::read_to_string(&args.program).unwrap();
    let mut proc = Processor::new(&prog, args.numeric_output, 10);
    proc.run();
}
