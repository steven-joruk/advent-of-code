use std::collections::HashSet;
use std::convert::From;
use std::error::Error;

static INPUT: &str = include_str!("../res/10");

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn angle_to(&self, other: &Point) -> f64 {
        let m1 = self.y / self.x;
        let m2 = other.y / other.x;
        m1.atan2(m2)
    }
}

#[derive(Clone, Debug)]
struct Asteroid {
    pos: Point,
    angles_observed: Vec<f64>,
}

impl Asteroid {
    pub fn observe(&mut self, pos: &Point) {
        if self.pos == *pos {
            return;
        }

        let angle = self.pos.angle_to(&pos);
        if !self.angles_observed.contains(&angle) {
            self.angles_observed.push(angle);
        } else {
            //            println!("Already contains {}", angle)
        }
    }
}

pub fn solve() {
    println!("10.1: {}", part1(INPUT));
}

pub fn part1(input: &str) -> usize {
    let mut asteroids = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                let pos = Point {
                    x: x as f64,
                    y: y as f64,
                };

                asteroids.push(Asteroid {
                    pos,
                    angles_observed: Vec::new(),
                });
            }
        }
    }

    let other_asteroids = asteroids.clone();

    for mut origin in &mut asteroids {
        for other in &other_asteroids {
            origin.observe(&other.pos);
        }
    }

    let best = asteroids
        .iter()
        .max_by_key(|x| x.angles_observed.len())
        .unwrap();

    let hmm = asteroids
        .iter()
        .find(|a| a.pos.x == 11.0 && a.pos.y == 13.0)
        .unwrap();

    println!("best = {:?}", best.pos);
    println!("hmm = {}... {:?}", hmm.angles_observed.len(), hmm);

    best.angles_observed.len()
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angles() {
        assert_eq!(
            Point { x: 0.0, y: 10.0 }.angle_to(&Point { x: 10.0, y: 20.0 }),
            45.0
        );
    }

    #[test]
    fn example() {
        let input = "
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        assert_eq!(part1(input), 210);
    }
}
