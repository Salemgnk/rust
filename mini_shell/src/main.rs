//
// EPITECH PROJECT, 2025
// mini_shell
// File description:
// main
//

use std::{io::Write, process::{exit, Command}};
use std::collections::HashMap;

fn display_prompt() -> () {
    print!("{}@{} $> ", users::get_current_username().unwrap().to_string_lossy(),
        gethostname::gethostname().to_string_lossy());
    std::io::stdout().flush().unwrap();

}

fn cd_handler(args: &[&str], env_cpy: &HashMap<String, String>) {
    if let Some(dir) = args.get(0) {
        match std::env::set_current_dir(dir) {
            Ok (()) => (),
            Err(e) => eprintln!("cd: {}", e),
        }
    } else {
        std::env::set_current_dir(&env_cpy["HOME"]).unwrap();
    }
}
fn builtin_handler(cmd: &str, args: &[&str], env_cpy: &HashMap<String, String>)-> bool {
    match cmd {
        "cd" => {
            cd_handler(args, env_cpy);
            true
        },
        "exit" => {
            std::process::exit(0);
        }
        _ => false
    }
}
fn main_loop(env_cpy: HashMap<String, String>) {
    let mut input: String = String::new();

    loop {
        display_prompt();
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
        if builtin_handler(cmd, &args, &env_cpy) {
            input.clear();
            continue;
        }
        match Command::new(cmd).args(&args).output() {
            Ok(output) => {
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
    let env_cpy: HashMap<String, String> = std::env::vars().collect();
    if args.len() != 1 {
        exit(84);
    } else {
        main_loop(env_cpy);
    }
}
