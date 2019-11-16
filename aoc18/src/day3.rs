use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &str = include_str!("../res/3");

#[derive(Eq, Hash, PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug)]
struct Rect {
    id: u16,
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

impl FromStr for Rect {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let subs: Vec<&str> = s.split_whitespace().collect();
        let points: Vec<&str> = subs[2].split(",").collect();
        let dimensions: Vec<&str> = subs[3].split("x").collect();

        let rect = Rect {
            id: subs[0][1..].parse().unwrap(),
            x: points[0].parse().unwrap(),
            // https://github.com/rust-lang/rfcs/issues/2249
            y: points[1][..points[1].len() - 1].parse().unwrap(),
            w: dimensions[0].parse().unwrap(),
            h: dimensions[1].parse().unwrap(),
        };

        Ok(rect)
    }
}

#[derive(Default)]
struct Fabric {
    pub claims: HashMap<Point, u16>,
}

impl Fabric {
    fn add_claim(&mut self, r: &Rect) {
        // Adding an rect.positions() iterator generator would be
        // nicer.
        for x in r.x..r.x + r.w {
            for y in r.y..r.y + r.h {
                let p = Point { x, y };
                *self.claims.entry(p).or_insert(0) += 1;
            }
        }
    }

    fn get_contested_inches_count(&self) -> usize {
        self.claims.values().filter(|v| **v > 1).count()
    }

    fn is_uncontested(&self, rect: &Rect) -> bool {
        for x in rect.x..rect.x + rect.w {
            for y in rect.y..rect.y + rect.h {
                if self.claims[&Point { x, y }] != 1 {
                    return false;
                }
            }
        }

        true
    }
}

pub fn solve() {
    let mut fabric = Fabric::default();

    let rects: Vec<Rect> = INPUT
        .lines()
        .map(|l| Rect::from_str(&l))
        .map(|r| r.unwrap())
        .collect();

    for rect in &rects {
        fabric.add_claim(rect);
    }

    println!(
        " 3 1 There are {} square inches of fabric with contested claims",
        fabric.get_contested_inches_count()
    );

    let id = rects
        .iter()
        .filter(|r| fabric.is_uncontested(r))
        .map(|r| r.id)
        .next()
        .expect("Couldn't find an uncontested claim id");

    println!(" 3 2 Claim {} is uncontested", id);
}
