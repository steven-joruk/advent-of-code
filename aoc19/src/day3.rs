static INPUT: &str = include_str!("../res/3");

pub fn solve() {
    part1();
    part2();
}

// TODO: See if I can combine Move/Line in one pass during parsing

#[derive(Debug)]
enum Move {
    Up(i16),
    Down(i16),
    Left(i16),
    Right(i16),
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        let distance = input[1..].parse::<i16>().unwrap();
        match input.chars().next() {
            Some('U') => Move::Up(distance),
            Some('D') => Move::Down(distance),
            Some('L') => Move::Left(distance),
            Some('R') => Move::Right(distance),
            ch => panic!("Unsupported direction: {:?}", ch),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn manhattan_distance(&self) -> i16 {
        self.x.abs() + self.y.abs()
    }
}

struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn intersection(&self, other: &Line) -> Option<Point> {
        if !self.is_perpendicular_to(other) {
            return None;
        }

        let vline: &Line;
        let hline: &Line;

        if self.is_vertical() {
            vline = self;
            hline = other;
        } else {
            vline = other;
            hline = self;
        }

        if vline.from.x > hline.from.x && vline.from.x > hline.to.x {
            return None;
        }

        if vline.from.x < hline.from.x && vline.from.x < hline.to.x {
            return None;
        }

        if hline.from.y > vline.from.y && hline.from.y > vline.to.y {
            return None;
        }

        if hline.from.y < vline.from.y && hline.from.y < vline.to.y {
            return None;
        }

        let point = Point {
            x: vline.from.x,
            y: hline.from.y,
        };

        Some(point)
    }

    #[inline]
    fn is_perpendicular_to(&self, other: &Line) -> bool {
        self.is_vertical() != other.is_vertical()
    }

    #[inline]
    fn is_vertical(&self) -> bool {
        self.from.y != self.to.y
    }
}

fn create_lines_from_moves(moves: &[Move]) -> Vec<Line> {
    let mut lines = Vec::new();
    lines.reserve(moves.len());

    let mut position: Point;
    let mut last_position = Point::default();

    for mv in moves {
        position = last_position;

        match *mv {
            Move::Up(d) => position.y -= d,
            Move::Down(d) => position.y += d,
            Move::Left(d) => position.x -= d,
            Move::Right(d) => position.x += d,
        }

        lines.push(Line {
            from: last_position,
            to: position,
        });

        last_position = position;
    }

    lines
}

fn find_path_intersections(wire1: &[Line], wire2: &[Line]) -> Vec<Point> {
    let mut intersections = Vec::new();
    for line1 in wire1 {
        for line2 in wire2 {
            if let Some(point) = line1.intersection(line2) {
                intersections.push(point);
            }
        }
    }

    intersections
}

fn part1() {
    let wires_moves: Vec<Vec<Move>> = INPUT
        .lines()
        .map(|l| l.split(',').map(|m| Move::from(m)).collect())
        .collect();

    let wire1_path = create_lines_from_moves(&wires_moves[0]);
    let wire2_path = create_lines_from_moves(&wires_moves[1]);

    let min = find_path_intersections(&wire1_path, &wire2_path)
        .iter()
        .map(|p| p.manhattan_distance())
        .min();

    println!("The manhattan distance is {:?}", min);
}

fn part2() {}
