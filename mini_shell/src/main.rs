use std::{io::Write, process::{exit, Command}};

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
        let args: Vec<&str> = parts.collect();
        let output = Command::new(cmd)
            .args(&args)
            .output();
        match output {
            Ok (output) => {
                print!("{}", String::from_utf8_lossy(&output.stdout));
                eprint!("{}", String::from_utf8_lossy(&output.stderr));
            }
            Err(e) => {
                eprintln!("{}", e);
            } 
        }
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
