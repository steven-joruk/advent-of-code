use crate::intcode::{Computer, StepResult};
use std::error::Error;

static INPUT: &str = include_str!("../res/9");

pub fn solve() -> Result<(), Box<dyn Error>> {
    let program: Vec<i64> = INPUT
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut cpu = Computer::new(&program);
    cpu.add_input(1);

    loop {
        match cpu.run()? {
            StepResult::OutputAvailable(v) => println!("9.1 {}", v),
            StepResult::Finished => break,
            output => panic!("Unexpected output: {:?}", output),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let program: Vec<i64> = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let mut cpu = Computer::new(&program);
        let mut output = Vec::new();

        loop {
            if let StepResult::OutputAvailable(o) = cpu.run()? {
                output.push(o);
            } else {
                break;
            }
        }

        assert_eq!(
            output,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );

        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let program: Vec<i64> = "1102,34915192,34915192,7,4,7,99,0"
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let mut cpu = Computer::new(&program);

        assert_eq!(
            StepResult::OutputAvailable(1_219_070_632_396_864),
            cpu.run()?
        );

        Ok(())
    }

    #[test]
    fn example_3() -> Result<(), Box<dyn Error>> {
        let program: Vec<i64> = "104,1125899906842624,99"
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let mut cpu = Computer::new(&program);

        assert_eq!(
            StepResult::OutputAvailable(1_125_899_906_842_624),
            cpu.run()?
        );

        Ok(())
    }
}
