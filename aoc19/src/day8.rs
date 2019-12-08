static INPUT: &str = include_str!("../res/8");
static WIDTH: usize = 25;
static HEIGHT: usize = 6;
static AREA: usize = WIDTH * HEIGHT;

pub fn solve() {
    let layers = INPUT.as_bytes().chunks(AREA);

    // Doing it this way was just for fun, it's not particularly nice.
    // Convert each layer in to a tuple of (0_pixels, 1_pixels, 2_pixels) so
    // you can have all totals as the result.
    let pixel_counts = layers
        .clone()
        .map(|layer| {
            layer
                .iter()
                .map(|pixel| match pixel {
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

    let mut image = Vec::new();
    image.resize(AREA, b' ');

    for layer in layers.rev() {
        for i in 0..AREA {
            match layer[i] {
                b'0' => image[i] = b' ',
                b'1' => image[i] = b'#',
                _ => continue,
            }
        }
    }

    println!("8.2");

    image
        .chunks(WIDTH)
        .map(|line| std::str::from_utf8(line).unwrap())
        .for_each(|line| println!("{}", line));
}
