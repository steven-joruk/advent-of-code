//use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &str = include_str!("../res/4");

pub fn solve() {
    part1();
}

fn part1() {
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
                let id: u16 = sub[..sub.find(" ").expect("find id space")]
                    .parse()
                    .expect("parse id");
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

    let mut events: Vec<_> = INPUT
        .lines()
        .map(|l| Event::from_str(&l))
        .map(|e| e.expect("unwrap event"))
        .collect();

    events.sort();

    //let mut totals = HashMap::new();
    /*
        for event in events {
            let mut current_id;
            match event.event_type {
                EventType::Begin(id) => current_id = id,
                EventType::Begin => total.entry(current_id).or_insert(0) +=
            }
        }
    */
    assert_eq!(1, 2);
}
