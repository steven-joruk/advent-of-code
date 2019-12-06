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

    fn ancestors_between(child: &str, ancestor: &str, orbits: &HashMap<&str, &str>) -> Option<u32> {
        let mut count = 0;
        let mut cur = child;

        while cur != ancestor {
            cur = orbits.get(cur)?;
            count += 1;
        }

        Some(count)
    }

    let mut cur = orbits["YOU"];
    let mut my_hops = 0;

    let total = loop {
        if let Some(v) = ancestors_between("SAN", cur, &orbits) {
            break Some(my_hops + v - 1);
        }

        if cur == "COM" {
            break None;
        }

        cur = orbits[cur];
        my_hops += 1;
    };

    println!("Transfers between me and santa: {:?}", total);
}
