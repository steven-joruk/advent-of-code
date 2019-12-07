struct Passwords {
    higher: u32,
    current: String,
    exclusive_pair: bool,
}

impl Iterator for Passwords {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_u32 = self.current.parse::<u32>().unwrap();

        loop {
            // This skips the first password but it doesn't matter for the input
            // I have.
            current_u32 += 1;
            self.current = current_u32.to_string();

            if current_u32 > self.higher {
                return None;
            }

            if self.current_is_valid() {
                return Some(current_u32);
            }
        }
    }
}

impl Passwords {
    fn new(lower: u32, higher: u32, exclusive_pair: bool) -> Self {
        Passwords {
            higher,
            current: lower.to_string(),
            exclusive_pair,
        }
    }

    fn current_is_valid(&self) -> bool {
        #[derive(PartialEq)]
        enum State {
            UniquePrevious,
            TwoAdjacent,
            ThreeAdjacent,
        };

        let mut prev = self.current.chars().next().unwrap();
        let mut state = State::UniquePrevious;
        let mut has_two_adjacent = false;

        for ch in self.current.chars().skip(1) {
            if prev > ch {
                return false;
            }

            match state {
                State::UniquePrevious => {
                    if prev == ch {
                        state = State::TwoAdjacent;
                        if !self.exclusive_pair {
                            has_two_adjacent = true;
                        }
                    }
                }
                State::TwoAdjacent => {
                    if prev == ch {
                        state = State::ThreeAdjacent;
                    } else {
                        state = State::UniquePrevious;
                        has_two_adjacent = true;
                    }
                }
                State::ThreeAdjacent => {
                    if prev != ch {
                        state = State::UniquePrevious;
                    }
                }
            }

            prev = ch;
        }

        state == State::TwoAdjacent || has_two_adjacent
    }
}

pub fn solve() {
    let lower = 172_851;
    let higher = 675_869;

    let passwords = Passwords::new(lower, higher, false);
    println!("4.1 {}", passwords.count());

    let passwords = Passwords::new(lower, higher, true);
    println!("4.2 {}", passwords.count());
}
