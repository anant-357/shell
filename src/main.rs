use std::io::{stdin, stdout, Write};

fn main() {
    let mut history: Vec<String> = Vec::new();
    loop {
        print!("$ ");
        match stdout().flush() {
            Ok(_) => (),
            Err(_) => {
                eprintln!("IO Error");
                std::process::exit(1);
            }
        }

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                eprintln!("IO Error");
                std::process::exit(1);
            }
        }
        history.push(input.clone());

        let mut commands = input.trim().split_whitespace();
        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;
        }
    }
}
