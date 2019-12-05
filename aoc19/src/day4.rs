struct Passwords {
    higher: u32,
    current: String,
}

impl Iterator for Passwords {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_u32 = self.current.parse::<u32>().unwrap();

        loop {
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
    fn new(lower: u32, higher: u32) -> Self {
        Passwords {
            higher,
            current: lower.to_string(),
        }
    }

    fn current_is_valid(&self) -> bool {
        let mut has_adjacent_pair = false;
        let mut prev = self.current.chars().next().unwrap();

        for ch in self.current.chars().skip(1) {
            if prev > ch {
                return false;
            }

            if prev == ch {
                has_adjacent_pair = true;
            }

            prev = ch;
        }

        has_adjacent_pair
    }
}

pub fn solve() {
    let lower = 172_851;
    let higher = 675_869;

    let passwords = Passwords::new(lower, higher);
    println!("Number of passwords: {}", passwords.count());
}
