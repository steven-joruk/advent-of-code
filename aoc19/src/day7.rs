use crate::intcode::{Computer, StepResult};
use std::error::Error;

static INPUT: &str = include_str!("../res/7");

pub fn solve() -> Result<(), Box<dyn Error>> {
    let result = solve_with_input(INPUT, false)?;
    println!("7.1 {:?}", result);

    let result = solve_with_input(INPUT, true)?;
    println!("7.2 {:?}", result);

    Ok(())
}

fn run_circuit(amps: &mut [&mut Computer]) -> i32 {
    let mut last_output = None;

    loop {
        for amp in amps.iter_mut() {
            if let Some(input) = last_output {
                amp.add_input(input);
            }

            match amp.run().expect("Failed to run the amp") {
                StepResult::OutputAvailable(output) => {
                    last_output = Some(output);
                    continue;
                }
                StepResult::NeedInput => break,
                StepResult::Finished => return last_output.expect("The last output is None"),
            }
        }
    }
}

fn solve_with_input(input: &str, feedback_mode: bool) -> Result<i32, Box<dyn Error>> {
    let program: Vec<i32> = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut maximum_output: Option<i32> = None;

    let (lower, upper) = if feedback_mode { (5, 10) } else { (0, 5) };

    // TODO: Support any number of amps
    let mut amp_a = Computer::new(&program);
    let mut amp_b = Computer::new(&program);
    let mut amp_c = Computer::new(&program);
    let mut amp_d = Computer::new(&program);
    let mut amp_e = Computer::new(&program);

    for setting_a in lower..upper {
        for setting_b in lower..upper {
            if setting_a == setting_b {
                continue;
            }

            for setting_c in lower..upper {
                if [setting_a, setting_b].contains(&setting_c) {
                    continue;
                }

                for setting_d in lower..upper {
                    if [setting_a, setting_b, setting_c].contains(&setting_d) {
                        continue;
                    }

                    for setting_e in lower..upper {
                        if [setting_a, setting_b, setting_c, setting_d].contains(&setting_e) {
                            continue;
                        }

                        amp_a.add_input(setting_a).add_input(0);
                        amp_b.add_input(setting_b);
                        amp_c.add_input(setting_c);
                        amp_d.add_input(setting_d);
                        amp_e.add_input(setting_e);
                        let output = run_circuit(&mut [
                            &mut amp_a, &mut amp_b, &mut amp_c, &mut amp_d, &mut amp_e,
                        ]);

                        if let Some(highest) = maximum_output {
                            if output > highest {
                                maximum_output = Some(output);
                            }
                        } else {
                            maximum_output = Some(output);
                        }

                        amp_a.reset();
                        amp_b.reset();
                        amp_c.reset();
                        amp_d.reset();
                        amp_e.reset();
                    }
                }
            }
        }
    }

    Ok(maximum_output.expect("No maximum output"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular() {
        let result =
            solve_with_input("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", false).unwrap();
        let expected = 43_210;
        assert_eq!(result, expected);
    }

    #[test]
    fn feedback() {
        let result = solve_with_input(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
            true,
        )
        .unwrap();
        let expected = 139_629_729;
        assert_eq!(result, expected);
    }
}
