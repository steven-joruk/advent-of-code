use std::cell::Cell;

static INPUT: &str = include_str!("../res/2");

const INSTRUCTION_SIZE: usize = 4;
const ADD: u32 = 1;
const MULTIPLY: u32 = 2;
const END: u32 = 99;

pub fn solve() {
    part1();
    part2();
}

struct Operands {
    left: usize,
    right: usize,
    destination: usize,
}

enum Instruction {
    Add(Operands),
    Multiply(Operands),
    End,
}

impl From<&[Cell<u32>]> for Instruction {
    fn from(input: &[Cell<u32>]) -> Self {
        match input[0].get() {
            ADD => Instruction::Add(Operands {
                left: input[1].get() as usize,
                right: input[2].get() as usize,
                destination: input[3].get() as usize,
            }),
            MULTIPLY => Instruction::Multiply(Operands {
                left: input[1].get() as usize,
                right: input[2].get() as usize,
                destination: input[3].get() as usize,
            }),
            END => Instruction::End,
            _ => unimplemented!(),
        }
    }
}

fn run(program: &mut [Cell<u32>]) {
    for instruction in program.chunks(INSTRUCTION_SIZE).map(Instruction::from) {
        match instruction {
            Instruction::Add(op) => {
                program[op.destination].set(program[op.left].get() + program[op.right].get());
            }
            Instruction::Multiply(op) => {
                program[op.destination].set(program[op.left].get() * program[op.right].get());
            }
            Instruction::End => {
                break;
            }
        }
    }
}

fn part1() {
    let mut program: Vec<Cell<u32>> = INPUT
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .map(Cell::new)
        .collect();

    program[1].set(12);
    program[2].set(2);

    run(&mut program);

    println!("Address 0 holds {}", program[0].get());
}

fn part2() {
    let program: Vec<Cell<u32>> = INPUT
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .map(Cell::new)
        .collect();

    let desired_result = 19_690_720;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut instance = program.clone();
            instance[1].set(noun);
            instance[2].set(verb);
            run(&mut instance);
            if instance[0].get() == desired_result {
                println!(
                    "Noun = {}, verb = {}. 100 * noun + verb = {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
                return;
            }
        }
    }

    println!("No noun and verb combination yields {}", desired_result);
}
