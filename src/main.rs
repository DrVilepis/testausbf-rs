use std::fs::read_to_string;

fn main() {
    let mut args = std::env::args();
    let mode = args.nth(1).expect("Missing agrument");
    let file = args.nth(0).expect("Missing agrument");
    println!("{} {}",mode, file);
    let input: String = read_to_string(file).unwrap();
    match mode.to_lowercase().as_str() {
        "gen" | "generate" => generate(input),
        "run" => run(input),
        _ => return
    }
}

fn generate(input: String) {
    unimplemented!();
}

fn run(input: String) {
    let valid = String::from("+-[]><.,");
    let input: Vec<char> = input.chars().filter(|c| valid.contains(c.to_owned()) ).collect();
    let mut cells = [0u8;30000];
    let mut data_pointer = 0usize;
    let mut pos = 0;
    let mut loop_stack: Vec<usize> = Vec::new();
    loop {
        if input.len() <= pos { break };
        match input[pos] {
            '+' => { cells[data_pointer] += 1 },
            '-' => { cells[data_pointer] -= 1 },
            '[' => { 
                if cells[data_pointer] != 0 {
                    loop_stack.push(pos);
                } else {
                    while input[pos] != ']' {
                        pos += 1;
                    }
                }
            },
            ']' => {
                if cells[data_pointer] != 0 {
                    pos = *loop_stack.last().unwrap();
                } else {
                    loop_stack.pop();
                }
            },
            '>' => { data_pointer += 1; },
            '<' => { data_pointer -= 1; },
            '.' => { print!("{}",cells[data_pointer] as char); },
            ',' => {},
            _ => unreachable!(),
        }
        pos += 1;
    }
    println!();
}
