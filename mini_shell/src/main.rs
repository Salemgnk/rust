use std::{io::Write, process::exit};

fn main_loop() {
    let mut input: String = String::new();

    loop {
        print!("$> ");
        std::io::stdout().flush().unwrap();
        let bytes_read = std::io::stdin().read_line(&mut input).unwrap();
        if bytes_read == 0 {
            break;
        }
        if input.trim() == "exit" {
            break;
        }
        let mut parts = input.trim().split_whitespace();
        let cmd =  match parts.next() {
            Some(c) => c,
            None => {
                input.clear();
                continue;
            }
        };
        input.clear();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 1 {
        exit(84);
    } else {
        main_loop();
    }
}
