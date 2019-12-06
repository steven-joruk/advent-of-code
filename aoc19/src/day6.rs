use std::collections::HashMap;

static INPUT: &str = include_str!("../res/6");

pub fn solve() {
    let mut orbits = HashMap::new();

    for line in INPUT.lines() {
        let mut names = line.split(")");
        let parent = names.next().unwrap();
        let child = names.next().unwrap();

        orbits.insert(child, parent);
    }

    let mut total = 0;

    for (object, parent) in orbits.iter() {
        if *object == "COM" {
            continue;
        }

        total += 1;

        let mut cur = parent;

        while *cur != "COM" {
            cur = orbits.get(cur).unwrap();
            total += 1;
        }
    }

    println!("Total orbits: {}", total);
}
