use std::collections::HashSet;

const INPUT: &str = include_str!("../res/1");

pub fn solve() {
    part1();
    part2();
}

fn part1() {
    let freq: i32 = INPUT.lines().map(|r| r.parse::<i32>().unwrap()).sum();
    println!(" 1 1 The resuling frequency is {}", freq);
}

fn part2() {
    let values = INPUT
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
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

    if let Some(freq) = result {
        println!(" 1 2 The first frequency seen twice is {:?}", freq);
    } else {
        eprintln!(" 1 2 Couldn't find a duplicate frequency");
    }
}

mod tests {
    #[bench]
    fn bench_part1(b: &mut test::Bencher) {
        b.iter(|| super::part1());
    }

    #[bench]
    fn bench_part2(b: &mut test::Bencher) {
        b.iter(|| super::part2());
    }
}
