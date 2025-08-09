fn no_arg_cat() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    loop {
        print!("{}", input);
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
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
