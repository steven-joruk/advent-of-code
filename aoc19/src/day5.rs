use crate::intcode::{Computer, StepResult};
use std::error::Error;

static INPUT: &str = include_str!("../res/5");

pub fn solve() -> Result<(), Box<dyn Error>> {
    let program: Vec<i64> = INPUT
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut last_output: i64 = 0;

    match Computer::new(&program).add_input(1).run()? {
        StepResult::OutputAvailable(output) => last_output = output,
        StepResult::Finished => println!("5.2 {:?}", last_output),
        StepResult::NeedInput => panic!("Program needs input"),
    }

    match Computer::new(&program).add_input(5).run()? {
        StepResult::OutputAvailable(output) => println!("5.2 {:?}", output),
        StepResult::Finished => panic!("Program finished before returning output"),
        StepResult::NeedInput => panic!("Program needs input"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_5_1() {
        let program: Vec<i64> = "3,9,8,9,10,9,4,9,99,-1,8"
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        assert_eq!(
            Computer::new(&program).add_input(7).run().unwrap(),
            StepResult::OutputAvailable(0)
        );

        assert_eq!(
            Computer::new(&program).add_input(8).run().unwrap(),
            StepResult::OutputAvailable(1)
        );

        assert_eq!(
            Computer::new(&program).add_input(9).run().unwrap(),
            StepResult::OutputAvailable(0)
        );
    }
}
