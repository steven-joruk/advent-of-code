static INPUT: &str = include_str!("../res/1");

pub fn solve() {
    part1();
    part2();
}

fn fuel_required_for_module(mass: i32) -> i32 {
    mass / 3 - 2
}

fn part1() {
    let module_fuel: i32 = INPUT
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .map(fuel_required_for_module)
        .sum();

    println!("Module fuel requirements: {}", module_fuel);
}

fn fuel_required_for_module_and_its_fuel(mass: i32) -> i32 {
    let mut total = fuel_required_for_module(mass);
    let mut current = total;

    loop {
        current = fuel_required_for_module(current);
        if current < 0 {
            return total;
        }

        total += current;
    }
}

fn part2() {
    // This could probably be done more succinctly with try_fold
    let total_fuel: i32 = INPUT
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .map(fuel_required_for_module_and_its_fuel)
        .sum();

    println!("Total fuel requirements: {}", total_fuel);
}
