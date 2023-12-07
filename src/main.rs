use std::{
    env,
    io::{stdin, stdout, Write},
    path, process,
};

fn main() {
    let mut history: Vec<String> = Vec::new();
    loop {
        print!("$ ");
        match stdout().flush() {
            Ok(_) => (),
            Err(_) => {
                eprintln!("IO Error");
                process::exit(1);
            }
        }

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                eprintln!("IO Error");
                process::exit(1);
            }
        }
        history.push(input.clone());

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "exit" => process::exit(1),
            "pwd" => println!("{:#?}", env::current_dir()),
            "cd" => match env::set_current_dir(path::Path::new(
                args.clone().peekable().peek().map_or("/", |x| *x),
            )) {
                Ok(_) => println!("{:#?}", args.peekable().peek()),
                Err(e) => eprintln!("{}", e),
            },
            _ => (),
        }
    }
}
