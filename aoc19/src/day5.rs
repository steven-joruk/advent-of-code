static INPUT: &str = include_str!("../res/5");

#[derive(PartialEq)]
enum Mode {
    Indirect,
    Immediate,
}

impl From<char> for Mode {
    fn from(c: char) -> Self {
        if c == '1' {
            Mode::Immediate
        } else {
            Mode::Indirect
        }
    }
}

fn load(mem: &[i32], mode: Mode, addr: usize) -> i32 {
    if mode == Mode::Immediate {
        mem[addr as usize]
    } else {
        mem[mem[addr as usize] as usize]
    }
}

fn store(mem: &mut [i32], addr: usize, value: i32) {
    let dest = load(mem, Mode::Immediate, addr) as usize;
    mem[dest] = value;
}

pub fn solve() {
    let mut mem: Vec<i32> = INPUT
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let input: i32 = 1;
    let mut output: i32 = 0;

    let mut i = 0;

    while i < mem.len() {
        let opstr = mem[i].to_string();

        let opcode = if opstr.len() == 1 {
            opstr[opstr.len() - 1..].parse::<u8>().unwrap()
        } else {
            opstr[opstr.len() - 2..].parse::<u8>().unwrap()
        };

        let right_mode = opstr
            .chars()
            .rev()
            .nth(3)
            .map(Mode::from)
            .unwrap_or(Mode::Indirect);

        let left_mode = opstr
            .chars()
            .rev()
            .nth(2)
            .map(Mode::from)
            .unwrap_or(Mode::Indirect);

        match opcode {
            1 => {
                let left_value = load(&mem, left_mode, i + 1);
                let right_value = load(&mem, right_mode, i + 2);
                store(&mut mem, i + 3, left_value + right_value);
                i += 4;
            }
            2 => {
                let left_value = load(&mem, left_mode, i + 1);
                let right_value = load(&mem, right_mode, i + 2);
                store(&mut mem, i + 3, left_value * right_value);
                i += 4;
            }
            3 => {
                store(&mut mem, i + 1, input);
                i += 2;
            }
            4 => {
                output = load(&mem, Mode::Indirect, i + 1);
                i += 2;
            }
            99 => {
                break;
            }
            opcode => panic!("Unknown opcode: {}", opcode),
        }
    }

    println!("Program output: {}", output);
}
