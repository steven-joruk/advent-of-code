use std::collections::HashMap;

const INPUT: &str = include_str!("../res/2");

pub fn solve() {
    part1();
    part2();
}

fn part1() {
    let mut doubles = 0;
    let mut triples = 0;

    let values = INPUT.lines().collect::<Vec<_>>();

    // To speed things up I could create a 26 item Vec and convert the
    // character to an index.
    let mut char_counts = HashMap::<char, u32>::new();

    for value in values {
        char_counts.clear();

        for c in value.chars() {
            *char_counts.entry(c).or_insert(0) += 1;
        }

        doubles += if char_counts.values().any(|v| *v == 2) {
            1
        } else {
            0
        };

        triples += if char_counts.values().any(|v| *v == 3) {
            1
        } else {
            0
        };
    }

    println!(
        " 2 1 There are {} doubles and {} triples, the checksum is {}",
        doubles,
        triples,
        doubles * triples
    );
}

fn differences<'a>(left: &'a str, right: &'a str) -> usize {
    left.chars()
        .zip(right.chars())
        .filter(|(l, r)| l != r)
        .count()
}

fn commonalities(left: &str, right: &str) -> String {
    left.chars()
        .zip(right.chars())
        .filter(|(l, r)| l == r)
        .map(|(l, _)| l)
        .collect()
}

fn find_correct_boxes<'a>(ids: &'a [&str]) -> Option<(&'a str, &'a str)> {
    for left in ids {
        for right in ids {
            if differences(&left, &right) == 1 {
                return Some((&left, &right));
            }
        }
    }

    None
}

fn part2() {
    let ids = INPUT.lines().collect::<Vec<_>>();

    let correct_ids = find_correct_boxes(&ids).expect("Couldn't find the correct boxes");

    println!(
        " 2 2 The common letters between the correct box IDs are {}",
        commonalities(correct_ids.0, correct_ids.1),
    );
}
