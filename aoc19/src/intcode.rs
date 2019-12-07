use std::collections::VecDeque;
use std::convert::{TryFrom, TryInto};
use std::error::Error;

pub enum StepResult {
    OutputAvailable(i32),
    NeedInput,
    Finished,
}

#[repr(u8)]
enum Opcode {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfLess = 5,
    JumpIfEqual = 6,
    IsLess = 7,
    IsEqual = 8,
    End = 99,
}

impl TryFrom<u8> for Opcode {
    type Error = String;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Opcode::Add as u8 => Ok(Opcode::Add),
            x if x == Opcode::Multiply as u8 => Ok(Opcode::Multiply),
            x if x == Opcode::Input as u8 => Ok(Opcode::Input),
            x if x == Opcode::Output as u8 => Ok(Opcode::Output),
            x if x == Opcode::JumpIfLess as u8 => Ok(Opcode::JumpIfLess),
            x if x == Opcode::JumpIfEqual as u8 => Ok(Opcode::JumpIfEqual),
            x if x == Opcode::IsLess as u8 => Ok(Opcode::IsLess),
            x if x == Opcode::IsEqual as u8 => Ok(Opcode::IsEqual),
            x if x == Opcode::End as u8 => Ok(Opcode::End),
            _ => Err(format!("Unknown opcode: {}", v).into()),
        }
    }
}

#[derive(PartialEq)]
pub enum Mode {
    Indirect,
    Immediate,
}

impl From<char> for Mode {
    fn from(c: char) -> Self {
        if c == '1' {
            Mode::Immediate
        } else {
            Mode::Indirect
        }
    }
}

pub struct Computer<'a> {
    program: &'a [i32],
    inputs: VecDeque<i32>,
    mem: Vec<i32>,
    pc: usize,
}

impl<'a> Computer<'a> {
    pub fn new(program: &'a [i32]) -> Self {
        Computer {
            program,
            inputs: VecDeque::new(),
            mem: program.to_vec(),
            pc: 0,
        }
    }

    pub fn reset(&mut self) {
        self.inputs.clear();
        self.mem = self.program.to_vec();
        self.pc = 0;
    }

    pub fn add_input<'b>(&'b mut self, value: i32) -> &'b mut Self {
        self.inputs.push_back(value);
        self
    }

    #[inline]
    pub fn load(&mut self, mode: Mode, addr: usize) -> i32 {
        if mode == Mode::Immediate {
            self.mem[addr as usize]
        } else {
            self.mem[self.mem[addr as usize] as usize]
        }
    }

    #[inline]
    pub fn store(&mut self, addr: usize, value: i32) {
        self.mem[addr as usize] = value;
    }

    /// Runs until I/O is required, or the program has ended.
    pub fn run(&mut self) -> Result<StepResult, Box<dyn Error>> {
        loop {
            let opstr = self.mem[self.pc].to_string();

            let opcode = if opstr.len() == 1 {
                opstr[opstr.len() - 1..].parse::<u8>()?
            } else {
                opstr[opstr.len() - 2..].parse::<u8>()?
            };

            // It'd be nicer if Rust had negative indexing already.
            let right_mode: Mode = opstr.chars().rev().nth(3).unwrap_or('0').into();
            let left_mode: Mode = opstr.chars().rev().nth(2).unwrap_or('0').into();

            match opcode.try_into()? {
                Opcode::Add => {
                    let left_value = self.load(left_mode, self.pc + 1);
                    let right_value = self.load(right_mode, self.pc + 2);
                    let dest = self.load(Mode::Immediate, self.pc + 3) as usize;
                    self.store(dest, left_value + right_value);
                    self.pc += 4;
                }
                Opcode::Multiply => {
                    let left_value = self.load(left_mode, self.pc + 1);
                    let right_value = self.load(right_mode, self.pc + 2);
                    let dest = self.load(Mode::Immediate, self.pc + 3) as usize;
                    self.store(dest, left_value * right_value);
                    self.pc += 4;
                }
                Opcode::Input => {
                    if self.inputs.is_empty() {
                        return Ok(StepResult::NeedInput);
                    }

                    let dest = self.load(Mode::Immediate, self.pc + 1) as usize;
                    let input = self.inputs.pop_front().unwrap();
                    self.store(dest, input);
                    self.pc += 2;
                }
                Opcode::Output => {
                    let output = self.load(Mode::Indirect, self.pc + 1);
                    self.pc += 2;
                    return Ok(StepResult::OutputAvailable(output));
                }
                Opcode::JumpIfLess => {
                    let value = self.load(left_mode, self.pc + 1);
                    if value != 0 {
                        self.pc = self.load(right_mode, self.pc + 2) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Opcode::JumpIfEqual => {
                    let value = self.load(left_mode, self.pc + 1);
                    if value == 0 {
                        self.pc = self.load(right_mode, self.pc + 2) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Opcode::IsLess => {
                    let left_value = self.load(left_mode, self.pc + 1);
                    let right_value = self.load(right_mode, self.pc + 2);
                    let dest = self.load(Mode::Immediate, self.pc + 3) as usize;
                    let result = if left_value < right_value { 1 } else { 0 };
                    self.store(dest, result);
                    self.pc += 4;
                }
                Opcode::IsEqual => {
                    let left_value = self.load(left_mode, self.pc + 1);
                    let right_value = self.load(right_mode, self.pc + 2);
                    let dest = self.load(Mode::Immediate, self.pc + 3) as usize;
                    let result = if left_value == right_value { 1 } else { 0 };
                    self.store(dest, result);
                    self.pc += 4;
                }
                Opcode::End => {
                    self.pc = self.mem.len();
                    return Ok(StepResult::Finished);
                }
            }
        }
    }
}
