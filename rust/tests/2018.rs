#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;
    use std::str::FromStr;

    fn load_input(file_name: &str) -> impl BufRead {
        const RES_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../inputs");

        let path = PathBuf::from(&RES_PATH).join(file_name);
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => panic!("Failed to open {}: {}", path.display(), e),
        };

        BufReader::new(file)
    }

    #[test]
    fn test_day_1_part_1() {
        let result = load_input("2018-1")
            .lines()
            .map(|r| r.unwrap().parse::<i32>().unwrap())
            .fold(0, |acc, x| acc + x);

        assert_eq!(result, 599);
    }

    #[test]
    fn test_day_1_part_2() {
        let values = load_input("2018-1")
            .lines()
            .map(|l| l.unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut seen = HashSet::new();
        seen.insert(0);

        let mut frequency = 0;
        let mut result: Option<i32> = None;

        while result == None {
            for value in &values {
                frequency += value;

                if seen.contains(&frequency) {
                    result = Some(frequency);
                    break;
                }

                seen.insert(frequency);
            }
        }

        assert_eq!(result, Some(81204));
    }

    #[test]
    fn test_day_2_part_1() {
        let mut doubles = 0;
        let mut triples = 0;

        let values = load_input("2018-2")
            .lines()
            .map(|l| l.unwrap())
            .collect::<Vec<_>>();

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

        assert_eq!(doubles * triples, 9139);
    }

    #[test]
    fn test_day_2_part_2() {
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

        fn find_correct_boxes<'a>(ids: &'a [String]) -> Option<(&'a str, &'a str)> {
            for left in ids {
                for right in ids {
                    if differences(&left, &right) == 1 {
                        return Some((&left, &right));
                    }
                }
            }

            None
        }

        let values = load_input("2018-2")
            .lines()
            .map(|l| l.unwrap())
            .collect::<Vec<_>>();

        let result = find_correct_boxes(&values).expect("Couldn't find the correct boxes");

        assert_eq!(
            commonalities(result.0, result.1),
            String::from("uqcidadzwtnhsljvxyobmkfyr")
        );
    }

    #[test]
    fn test_day_3() {
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

        let mut fabric = Fabric::default();

        let rects: Vec<Rect> = load_input("2018-3")
            .lines()
            .map(|l| l.unwrap())
            .map(|l| Rect::from_str(&l))
            .map(|r| r.unwrap())
            .collect();

        for rect in &rects {
            fabric.add_claim(rect);
        }

        assert_eq!(fabric.get_contested_inches_count(), 111_485);

        let uncontested: Vec<_> = rects
            .iter()
            .filter(|r| fabric.is_uncontested(r))
            .map(|r| r.id)
            .collect();

        assert_eq!(uncontested.len(), 1);
        assert_eq!(uncontested[0], 113);
    }

    #[test]
    fn test_day_4() {
        #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
        enum EventType {
            Begin(u16),
            Sleep,
            Wake,
        }

        #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
        struct Event {
            year: u16,
            month: u8,
            day: u8,
            hour: u8,
            minute: u8,
            event_type: EventType,
        }

        impl FromStr for Event {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let event_type: EventType;

                if s[19..].find("falls asleep").is_some() {
                    event_type = EventType::Sleep;
                } else if s[19..].find("wakes up").is_some() {
                    event_type = EventType::Wake;
                } else {
                    let sub = &s[s.find('#').expect("find #") + 1..];
                    let id: u16 = sub[..sub.find(" ").expect("find id space")].parse().expect("parse id");
                    event_type = EventType::Begin(id);
                }

                Ok(Event {
                    year: s[1..5].parse().expect("year"),
                    month: s[6..8].parse().expect("month"),
                    day: s[9..11].parse().expect("day"),
                    hour: s[12..14].parse().expect("hour"),
                    minute: s[15..17].parse().expect("minute"),
                    event_type,
                })
            }
        }

        let mut events: Vec<_> = load_input("2018-4")
            .lines()
            .map(|l| l.expect("unwrap line"))
            .map(|l| Event::from_str(&l))
            .map(|e| e.expect("unwrap event"))
            .collect();

        events.sort();

        let mut totals = HashMap::new();

        for event in events {
            let mut current_id;
            match event.event_type {
                EventType::Begin(id) => current_id = id,
                EventType::Begin => total.entry(current_id).or_insert(0) += 
            }
        }

        assert_eq!(1, 2);
    }
}
