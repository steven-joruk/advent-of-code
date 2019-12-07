use crate::intcode::Computer;
use std::error::Error;

static INPUT: &str = include_str!("../res/7");

pub fn solve() -> Result<(), Box<dyn Error>> {
    let result = solve_with_input(INPUT)?;
    println!("Maximum thrusters signal: {:?}", result);
    Ok(())
}

fn solve_with_input(input: &str) -> Result<i32, Box<dyn Error>> {
    let program: Vec<i32> = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut maximum_output: Option<i32> = None;

    for setting_a in 0..5 {
        for setting_b in 0..5 {
            if setting_a == setting_b {
                continue;
            }

            for setting_c in 0..5 {
                if [setting_a, setting_b].contains(&setting_c) {
                    continue;
                }

                for setting_d in 0..5 {
                    if [setting_a, setting_b, setting_c].contains(&setting_d) {
                        continue;
                    }

                    for setting_e in 0..5 {
                        if [setting_a, setting_b, setting_c, setting_d].contains(&setting_e) {
                            continue;
                        }

                        let output_a = Computer::new(&program).run(&[setting_a, 0])?;
                        let output_b = Computer::new(&program).run(&[setting_b, output_a])?;
                        let output_c = Computer::new(&program).run(&[setting_c, output_b])?;
                        let output_d = Computer::new(&program).run(&[setting_d, output_c])?;
                        let output = Computer::new(&program).run(&[setting_e, output_d])?;

                        if let Some(highest) = maximum_output {
                            if output > highest {
                                maximum_output = Some(output);
                            }
                        } else {
                            maximum_output = Some(output);
                        }
                    }
                }
            }
        }
    }

    Ok(maximum_output.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = solve_with_input("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0").unwrap();
        let expected = 43210;
        assert_eq!(result, expected);
    }
}
