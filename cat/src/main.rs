fn no_arg_cat() {
    let mut input: String = String::new();
    loop {
        let bytes_read = std::io::stdin().read_line(&mut input).unwrap();
        if bytes_read == 0 {
            break;
        }
        print!("{}", input);
        input.clear();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        no_arg_cat()
    } else {
        println!("Not defined yet")
    }
}
