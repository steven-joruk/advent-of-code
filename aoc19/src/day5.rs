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

pub fn run(mem: &mut [i32], input: i32) -> i32 {
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
                store(mem, i + 3, left_value + right_value);
                i += 4;
            }
            2 => {
                let left_value = load(&mem, left_mode, i + 1);
                let right_value = load(&mem, right_mode, i + 2);
                store(mem, i + 3, left_value * right_value);
                i += 4;
            }
            3 => {
                store(mem, i + 1, input);
                i += 2;
            }
            4 => {
                output = load(&mem, Mode::Indirect, i + 1);
                i += 2;
            }
            5 => {
                let value = load(mem, left_mode, i + 1);
                if value != 0 {
                    i = load(mem, right_mode, i + 2) as usize;
                } else {
                    i += 3;
                }
            }
            6 => {
                let value = load(mem, left_mode, i + 1);
                if value == 0 {
                    i = load(mem, right_mode, i + 2) as usize;
                } else {
                    i += 3;
                }
            }
            7 => {
                let left_value = load(mem, left_mode, i + 1);
                let right_value = load(mem, right_mode, i + 2);
                let value = if left_value < right_value { 1 } else { 0 };
                store(mem, i + 3, value);
                i += 4;
            }
            8 => {
                let left_value = load(mem, left_mode, i + 1);
                let right_value = load(mem, right_mode, i + 2);
                let value = if left_value == right_value { 1 } else { 0 };
                store(mem, i + 3, value);
                i += 4;
            }
            99 => {
                break;
            }
            opcode => panic!("Unknown opcode: {}", opcode),
        }
    }

    output
}

pub fn solve() {
    let mem: Vec<i32> = INPUT
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let output = run(&mut mem.clone(), 1);
    println!("Program output: {}", output);

    let output = run(&mut mem.clone(), 5);
    println!("Program output: {}", output);
}
