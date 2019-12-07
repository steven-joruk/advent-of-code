use std::convert::{TryFrom, TryInto};
use std::error::Error;

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
    mem: Vec<i32>,
}

impl<'a> Computer<'a> {
    pub fn new(program: &'a [i32]) -> Self {
        Computer {
            program,
            mem: program.to_vec(),
        }
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

    #[inline]
    pub fn reset(&mut self) {
        self.mem = self.program.to_vec();
    }

    /// Runs the loaded program. The memory isn't automatically reset between
    /// runs.
    pub fn run(&mut self, inputs: &[i32]) -> Result<i32, Box<dyn Error>> {
        let mut output: i32 = 0;
        let mut pc = 0;
        let mut remaining_inputs = inputs;

        while pc < self.mem.len() {
            let mut opstr = self.mem[pc].to_string();

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
                    let left_value = self.load(left_mode, pc + 1);
                    let right_value = self.load(right_mode, pc + 2);
                    let dest = self.load(Mode::Immediate, pc + 3) as usize;
                    self.store(dest, left_value + right_value);
                    pc += 4;
                }
                Opcode::Multiply => {
                    let left_value = self.load(left_mode, pc + 1);
                    let right_value = self.load(right_mode, pc + 2);
                    let dest = self.load(Mode::Immediate, pc + 3) as usize;
                    self.store(dest, left_value * right_value);
                    pc += 4;
                }
                Opcode::Input => {
                    let dest = self.load(Mode::Immediate, pc + 1) as usize;
                    let input = remaining_inputs[0];
                    remaining_inputs = &remaining_inputs[1..];
                    self.store(dest, input);
                    pc += 2;
                }
                Opcode::Output => {
                    output = self.load(Mode::Indirect, pc + 1);
                    pc += 2;
                }
                Opcode::JumpIfLess => {
                    let value = self.load(left_mode, pc + 1);
                    if value != 0 {
                        pc = self.load(right_mode, pc + 2) as usize;
                    } else {
                        pc += 3;
                    }
                }
                Opcode::JumpIfEqual => {
                    let value = self.load(left_mode, pc + 1);
                    if value == 0 {
                        pc = self.load(right_mode, pc + 2) as usize;
                    } else {
                        pc += 3;
                    }
                }
                Opcode::IsLess => {
                    let left_value = self.load(left_mode, pc + 1);
                    let right_value = self.load(right_mode, pc + 2);
                    let dest = self.load(Mode::Immediate, pc + 3) as usize;
                    let value = if left_value < right_value { 1 } else { 0 };
                    self.store(dest, value);
                    pc += 4;
                }
                Opcode::IsEqual => {
                    let left_value = self.load(left_mode, pc + 1);
                    let right_value = self.load(right_mode, pc + 2);
                    let dest = self.load(Mode::Immediate, pc + 3) as usize;
                    let value = if left_value == right_value { 1 } else { 0 };
                    self.store(dest, value);
                    pc += 4;
                }
                Opcode::End => {
                    break;
                }
            }
        }

        Ok(output)
    }
}
