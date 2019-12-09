use std::collections::VecDeque;
use std::convert::{TryFrom, TryInto};
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum StepResult {
    OutputAvailable(i64),
    NeedInput,
    Finished,
}

/// Modes are in respect to their operand order.
#[derive(Debug)]
#[repr(u8)]
enum Opcode {
    Add(Mode, Mode, Mode),
    Multiply(Mode, Mode, Mode),
    Input(Mode),
    Output(Mode),
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    IsLess(Mode, Mode, Mode),
    IsEqual(Mode, Mode, Mode),
    ModifyBase(Mode),
    End,
}

/// The `enum_primitive` crate would remove the need for this.
/// Also interesting discussions here:
/// https://internals.rust-lang.org/t/pre-rfc-adding-conversion-to-from-integer-on-enums-with-repr-i-u/8758
impl From<i64> for Opcode {
    fn from(v: i64) -> Self {
        let opstr = format!("{:5}", v);

        let mode_2: Option<Mode> = opstr[0..1].try_into().ok();
        let mode_1: Option<Mode> = opstr[1..2].try_into().ok();
        let mode_0: Option<Mode> = opstr[2..3].try_into().ok();

        let kind = if &opstr[3..4] == " " {
            opstr[4..].parse::<u8>().unwrap()
        } else {
            opstr[3..].parse::<u8>().unwrap()
        };

        match kind {
            1 => Opcode::Add(
                mode_0.unwrap_or(Mode::Indirect),
                mode_1.unwrap_or(Mode::Indirect),
                mode_2.unwrap_or(Mode::Immediate),
            ),
            2 => Opcode::Multiply(
                mode_0.unwrap_or(Mode::Indirect),
                mode_1.unwrap_or(Mode::Indirect),
                mode_2.unwrap_or(Mode::Immediate),
            ),
            3 => Opcode::Input(mode_0.unwrap_or(Mode::Immediate)),
            4 => Opcode::Output(mode_0.unwrap_or(Mode::Indirect)),
            5 => Opcode::JumpIfTrue(
                mode_0.unwrap_or(Mode::Indirect),
                mode_1.unwrap_or(Mode::Indirect),
            ),
            6 => Opcode::JumpIfFalse(
                mode_0.unwrap_or(Mode::Indirect),
                mode_1.unwrap_or(Mode::Indirect),
            ),
            7 => Opcode::IsLess(
                mode_0.unwrap_or(Mode::Indirect),
                mode_1.unwrap_or(Mode::Indirect),
                mode_2.unwrap_or(Mode::Immediate),
            ),
            8 => Opcode::IsEqual(
                mode_0.unwrap_or(Mode::Indirect),
                mode_1.unwrap_or(Mode::Indirect),
                mode_2.unwrap_or(Mode::Immediate),
            ),
            9 => Opcode::ModifyBase(mode_0.unwrap_or(Mode::Indirect)),
            99 => Opcode::End,
            _ => panic!("Unknown opcode: {}", kind),
        }
    }
}

impl Opcode {
    pub fn len(&self) -> usize {
        match self {
            Opcode::Add(_, _, _) => 4,
            Opcode::Multiply(_, _, _) => 4,
            Opcode::Input(_) => 2,
            Opcode::Output(_) => 2,
            Opcode::JumpIfTrue(_, _) => 3,
            Opcode::JumpIfFalse(_, _) => 3,
            Opcode::IsLess(_, _, _) => 4,
            Opcode::IsEqual(_, _, _) => 4,
            Opcode::ModifyBase(_) => 2,
            Opcode::End => 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Indirect,
    Immediate,
    Relative,
}

impl TryFrom<&str> for Mode {
    type Error = String;

    fn try_from(c: &str) -> Result<Self, Self::Error> {
        match c {
            "0" => Ok(Mode::Indirect),
            "1" => Ok(Mode::Immediate),
            "2" => Ok(Mode::Relative),
            _ => Err(format!("Couldn't convert {} to a mode", c).into()),
        }
    }
}

#[derive(Default)]
pub struct Computer<'a> {
    program: &'a [i64],
    inputs: VecDeque<i64>,
    mem: Vec<i64>,
    pc: usize,
    base: i64,
}

impl<'a> Computer<'a> {
    pub fn new(program: &'a [i64]) -> Self {
        let mut cpu = Computer::default();
        cpu.program = program;
        cpu.reset();
        cpu
    }

    pub fn reset(&mut self) {
        self.inputs.clear();
        self.mem = self.program.to_vec();
        self.pc = 0;
        self.base = 0;
    }

    pub fn add_input<'b>(&'b mut self, value: i64) -> &'b mut Self {
        self.inputs.push_back(value);
        self
    }

    #[inline]
    fn checked_load(&mut self, addr: usize) -> i64 {
        if addr >= self.mem.len() {
            self.mem.resize(addr, 0);
            0
        } else {
            self.mem[addr]
        }
    }

    #[inline]
    pub fn load_value(&mut self, mode: &Mode, offset: usize) -> i64 {
        match mode {
            Mode::Indirect => {
                let offset = self.checked_load(offset) as usize;
                self.checked_load(offset)
            }
            Mode::Immediate => self.checked_load(offset),
            Mode::Relative => {
                let offset = (self.base + self.checked_load(offset)) as usize;
                self.checked_load(offset)
            }
        }
    }

    #[inline]
    fn load_address(&mut self, mode: &Mode, offset: usize) -> usize {
        match mode {
            Mode::Indirect => self.load_value(mode, offset) as usize,
            Mode::Immediate => self.load_value(mode, offset) as usize,
            Mode::Relative => (self.base + self.checked_load(offset)) as usize,
        }
    }

    #[inline]
    pub fn store(&mut self, addr: usize, value: i64) {
        if addr as usize >= self.mem.len() {
            self.mem.resize(addr + 1, 0);
        }

        self.mem[addr as usize] = value;
    }

    /// Runs until I/O is required, or the program has ended.
    pub fn run(&mut self) -> Result<StepResult, Box<dyn Error>> {
        loop {
            let opcode: Opcode = self.mem[self.pc].into();

            match &opcode {
                Opcode::Add(left_mode, right_mode, dest_mode) => {
                    let left_value = self.load_value(&left_mode, self.pc + 1);
                    let right_value = self.load_value(&right_mode, self.pc + 2);
                    let dest = self.load_address(&dest_mode, self.pc + 3) as usize;
                    self.store(dest, left_value + right_value);
                }
                Opcode::Multiply(left_mode, right_mode, dest_mode) => {
                    let left_value = self.load_value(&left_mode, self.pc + 1);
                    let right_value = self.load_value(&right_mode, self.pc + 2);
                    let dest = self.load_address(&dest_mode, self.pc + 3) as usize;
                    self.store(dest, left_value * right_value);
                }
                Opcode::Input(mode) => {
                    if self.inputs.is_empty() {
                        return Ok(StepResult::NeedInput);
                    }
                    let dest = self.load_address(&mode, self.pc + 1);
                    let input = self.inputs.pop_front().unwrap();
                    self.store(dest, input);
                }
                Opcode::Output(mode) => {
                    let value = self.load_value(&mode, self.pc + 1);
                    self.pc += opcode.len();
                    return Ok(StepResult::OutputAvailable(value));
                }
                Opcode::JumpIfTrue(value_mode, dest_mode) => {
                    let value = self.load_value(&value_mode, self.pc + 1);
                    if value != 0 {
                        self.pc = self.load_value(&dest_mode, self.pc + 2) as usize;
                        continue;
                    }
                }
                Opcode::JumpIfFalse(value_mode, dest_mode) => {
                    let value = self.load_value(&value_mode, self.pc + 1);
                    if value == 0 {
                        self.pc = self.load_value(&dest_mode, self.pc + 2) as usize;
                        continue;
                    }
                }
                Opcode::IsLess(left_mode, right_mode, dest_mode) => {
                    let left_value = self.load_value(&left_mode, self.pc + 1);
                    let right_value = self.load_value(&right_mode, self.pc + 2);
                    let dest = self.load_address(&dest_mode, self.pc + 3) as usize;
                    let result = if left_value < right_value { 1 } else { 0 };
                    self.store(dest, result);
                }
                Opcode::IsEqual(left_mode, right_mode, dest_mode) => {
                    let left = self.load_value(&left_mode, self.pc + 1);
                    let right = self.load_value(&right_mode, self.pc + 2);
                    let dest = self.load_address(&dest_mode, self.pc + 3) as usize;
                    let result = if left == right { 1 } else { 0 };
                    self.store(dest, result);
                }
                Opcode::ModifyBase(mode) => {
                    self.base += self.load_value(&mode, self.pc + 1);
                }
                Opcode::End => {
                    return Ok(StepResult::Finished);
                }
            }

            self.pc += opcode.len();
        }
    }
}
