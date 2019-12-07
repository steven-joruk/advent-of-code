use crate::intcode::{Computer, StepResult};
use std::error::Error;

static INPUT: &str = include_str!("../res/5");

pub fn solve() -> Result<(), Box<dyn Error>> {
    let program: Vec<i32> = INPUT
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    match Computer::new(&program).add_input(1).run()? {
        StepResult::OutputAvailable(output) => println!("5.1 {:?}", output),
        StepResult::Finished => panic!("Program finished before returning output"),
        StepResult::NeedInput => panic!("Program needs input"),
    }

    match Computer::new(&program).add_input(5).run()? {
        StepResult::OutputAvailable(output) => println!("5.2 {:?}", output),
        StepResult::Finished => panic!("Program finished before returning output"),
        StepResult::NeedInput => panic!("Program needs input"),
    }

    Ok(())
}
