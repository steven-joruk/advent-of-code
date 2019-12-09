use crate::intcode::{Computer, Mode, StepResult};

static INPUT: &str = include_str!("../res/2");

pub fn solve() {
    part1();
    part2();
}

fn part1() {
    let program: Vec<i64> = INPUT
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut cpu = Computer::new(&program);
    cpu.store(1, 12);
    cpu.store(2, 12);
    cpu.run().unwrap();
    let result = cpu.load_value(&Mode::Immediate, 0);

    println!("2.1 {}", result);
}

fn part2() {
    let program: Vec<i64> = INPUT
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let desired_result = 19_690_720;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut cpu = Computer::new(&program);
            cpu.store(1, noun);
            cpu.store(2, verb);
            match cpu.run().unwrap() {
                StepResult::Finished => {
                    if cpu.load_value(&Mode::Immediate, 0) == desired_result {
                        println!("2.2 {}", 100 * noun + verb);
                        return;
                    }
                }
                output => panic!("Unexpected output: {:?}", output),
            }
        }
    }

    println!("No noun and verb combination yields {}", desired_result);
}
