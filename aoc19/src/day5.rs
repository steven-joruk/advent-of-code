use crate::intcode::Computer;
use std::error::Error;

static INPUT: &str = include_str!("../res/5");

pub fn solve() -> Result<(), Box<dyn Error>> {
    let program: Vec<i32> = INPUT
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut cpu = Computer::new(&program);

    let output = cpu.run(&[1])?;
    println!("Program output: {}", output);

    cpu.reset();

    let output = cpu.run(&[5])?;
    println!("Program output: {}", output);

    Ok(())
}
