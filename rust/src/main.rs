#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;

    fn load_input(file_name: &str) -> impl BufRead {
        const RES_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../inputs");

        let path = PathBuf::from(&RES_PATH).join(file_name);
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => panic!("Failed to open {}: {}", path.display(), e),
        };

        BufReader::new(file)
    }

    #[test]
    fn test_day_1_part_1() {
        let result = load_input("2018-1")
            .lines()
            .map(|r| r.unwrap().parse::<i32>().unwrap())
            .fold(0, |acc, x| acc + x);

        assert_eq!(result, 599);
    }

    #[test]
    fn test_day_1_part_2() {
        let values = load_input("2018-1")
            .lines()
            .map(|l| l.unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut seen = HashSet::new();
        seen.insert(0);

        let mut frequency = 0;
        let mut result: Option<i32> = None;

        while result == None {
            for value in &values {
                frequency += value;

                if seen.contains(&frequency) {
                    result = Some(frequency);
                    break;
                }

                seen.insert(frequency);
            }
        }

        assert_eq!(result, Some(81204));
    }
}

fn main() {
    println!("Use `cargo test`");
}
