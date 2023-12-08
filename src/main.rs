use std::{
    collections::VecDeque,
    env,
    io::{stdin, stdout, Write},
    path, process,
};

fn tokenize(input: String) -> Result<Vec<String>, String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut op_stack: VecDeque<char> = VecDeque::new();
    let mut current_token: String = String::new();
    for c in input.chars() {
        match c {
            ' ' | '\t' => {
                if op_stack.back() == Some(&'"') {
                    current_token.push(c);
                } else {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            '(' | '{' => {
                if op_stack.back() == Some(&'"') {
                    current_token.push(c);
                } else {
                    op_stack.push_back(c);
                }
            }
            ')' | '}' => {
                if op_stack.back() == Some(&'"') {
                    current_token.push(c);
                } else if (c == ')' && op_stack.back() == Some(&'('))
                    || (c == '}' && op_stack.back() == Some(&'{'))
                {
                    op_stack.pop_back();
                }
            }
            '"' => {
                if op_stack.back() == Some(&'"') {
                    op_stack.pop_back();
                    current_token.push(c);
                    tokens.push(current_token.clone());
                    current_token.clear();
                } else {
                    op_stack.push_back(c);
                    tokens.push(current_token.clone());
                    current_token.clear();
                    current_token.push(c);
                }
            }
            '|' => {
                if op_stack.back() == Some(&'"') {
                    continue;
                } else {
                    tokens.push(current_token.clone());
                    tokens.push(String::from('|'));
                    current_token.clear()
                }
            }
            _ => current_token.push(c),
        }
    }
    tokens.push(current_token);
    Ok(tokens)
}

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

        println!("{:#?}", tokenize(input.clone()).unwrap());

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
