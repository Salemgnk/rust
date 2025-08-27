use std::io::Write;

fn weather_report(x: &str)
{
    match x {
        "soleil" => println!("Mets de la crème solaire !"),
        "pluie" => println!("N'oublie pas ton parapluie."),
        "neige" => println!("Habille toi chaudement"),
        "vent" => println!("Attention aux bourrasques"),
        _ => println!("Je ne connais pas ce temps là.")
    }
}

fn main()
{
    print!("Entrez le temps qu'il fait : ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.chars().rev().collect();
    weather_report(input.trim());
}