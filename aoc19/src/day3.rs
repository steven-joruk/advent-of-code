static INPUT: &str = include_str!("../res/3");

// TODO: See if I can combine Move/Line in one pass during parsing

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Move {
    dir: Dir,
    length: i16,
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        let length = input[1..].parse::<i16>().unwrap();
        let dir = match input.chars().next() {
            Some('U') => Dir::Up,
            Some('D') => Dir::Down,
            Some('L') => Dir::Left,
            Some('R') => Dir::Right,
            ch => panic!("Unsupported direction: {:?}", ch),
        };

        Move { dir, length }
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
    fn point_intersecting(&self, other: &Line) -> Option<Point> {
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

    fn steps_to_intersect(&self, point: &Point) -> Option<i16> {
        if self.is_vertical() {
            if self.from.x != point.x {
                return None;
            }

            if self.from.y < point.y && self.to.y < point.y {
                return None;
            }

            if self.from.y > point.y && self.to.y > point.y {
                return None;
            }

            return Some((self.from.y - point.y).abs());
        }

        if self.from.y != point.y {
            return None;
        }

        if self.from.x < point.x && self.to.x < point.x {
            return None;
        }

        if self.from.x > point.x && self.to.x > point.x {
            return None;
        }

        Some((self.from.x - point.x).abs())
    }

    #[inline]
    fn is_perpendicular_to(&self, other: &Line) -> bool {
        self.is_vertical() != other.is_vertical()
    }

    #[inline]
    fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }

    #[inline]
    fn length(&self) -> i16 {
        if self.is_vertical() {
            self.from.y.max(self.to.y) - self.from.y.min(self.to.y)
        } else {
            self.from.x.max(self.to.x) - self.from.x.min(self.to.x)
        }
    }
}

fn create_lines_from_moves(moves: &[Move]) -> Vec<Line> {
    let mut lines = Vec::new();
    lines.reserve(moves.len());

    let mut position: Point;
    let mut last_position = Point::default();

    for mv in moves {
        position = last_position;

        match mv.dir {
            Dir::Up => position.y -= mv.length,
            Dir::Down => position.y += mv.length,
            Dir::Left => position.x -= mv.length,
            Dir::Right => position.x += mv.length,
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
            if let Some(point) = line1.point_intersecting(line2) {
                intersections.push(point);
            }
        }
    }

    intersections
}

fn count_steps_to_intersection(path: &[Line], point: &Point) -> i16 {
    let mut total: i16 = 0;

    for line in path {
        if let Some(steps) = line.steps_to_intersect(point) {
            return total + steps;
        }

        total += line.length();
    }

    unreachable!();
}

pub fn solve() {
    let wires_moves: Vec<Vec<Move>> = INPUT
        .lines()
        .map(|l| l.split(',').map(|m| Move::from(m)).collect())
        .collect();

    let wire1_path = create_lines_from_moves(&wires_moves[0]);
    let wire2_path = create_lines_from_moves(&wires_moves[1]);

    let intersections = find_path_intersections(&wire1_path, &wire2_path);
    let min = intersections.iter().map(|p| p.manhattan_distance()).min();
    println!("3.1 {:?}", min);

    let mut steps = Vec::new();

    for intersection in intersections {
        let wire1_steps = count_steps_to_intersection(&wire1_path, &intersection);
        let wire2_steps = count_steps_to_intersection(&wire2_path, &intersection);
        steps.push(wire1_steps + wire2_steps);
    }

    let min = steps.iter().min();
    println!("3.2 {:?}", min);
}
