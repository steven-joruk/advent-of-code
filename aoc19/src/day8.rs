use crate::intcode::{Computer, StepResult};
use std::error::Error;

static INPUT: &str = include_str!("../res/8");
static WIDTH: usize = 25;
static HEIGHT: usize = 6;

pub fn solve() -> Result<(), Box<dyn Error>> {
    let layers = INPUT.as_bytes().chunks(WIDTH * HEIGHT);

    // Doing it this way was just for fun, it's not particularly nice.
    // Convert each layer in to a tuple of (0_pixels, 1_pixels, 2_pixels) so
    // you can have all totals as the result.
    let pixel_counts = layers
        .map(|l| {
            l.iter()
                .map(|p| match p {
                    b'0' => (1, 0, 0),
                    b'1' => (0, 1, 0),
                    b'2' => (0, 0, 1),
                    _ => panic!("Unknown pixel colour"),
                })
                .fold((0, 0, 0), |acc, v| (acc.0 + v.0, acc.1 + v.1, acc.2 + v.2))
        })
        .min_by(|l, r| (*l).0.cmp(&r.0))
        .unwrap();

    let ones = pixel_counts.1;
    let twos = pixel_counts.2;

    println!("8.1 {:?}", ones * twos);
    Ok(())
}
