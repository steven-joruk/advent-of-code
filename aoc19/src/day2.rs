use crate::intcode::{Computer, Mode};

static INPUT: &str = include_str!("../res/2");

pub fn solve() {
    part1();
    part2();
}

fn part1() {
    let program: Vec<i32> = INPUT
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut cpu = Computer::new(&program);
    cpu.store(1, 12);
    cpu.store(2, 12);
    cpu.run().unwrap();
    let result = cpu.load(Mode::Immediate, 0);

    println!("Address 0 holds {}", result);
}

fn part2() {
    let program: Vec<i32> = INPUT
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let desired_result = 19_690_720;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut cpu = Computer::new(&program);
            cpu.store(1, noun);
            cpu.store(2, verb);
            cpu.run().unwrap();
            if cpu.load(Mode::Immediate, 0) == desired_result {
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
