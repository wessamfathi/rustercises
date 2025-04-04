use std::collections::HashMap;
use std::io;

pub fn enter_data() {
    let mut db: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print_usage();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Faied to read line");

        let mut split = input.trim()[..].split_whitespace().collect::<Vec<&str>>();

        match split[0] {
            "add" => {
                if split.len() == 4 {
                    db.entry(split[3].to_string()).or_default().push(split[1].to_string());
                    println!("Added employee to company");
                } else {
                    println!("Invalid command");
                }
            },
            "list" => {
                if split.len() == 4 {
                    println!("List employee to company");
                    println!("{:?}", db.get(split[3]));
                } else {
                    println!("Invalid command");
                }
            },
            _ => {
                println!("Quitting");
                break;
            },
        }
    }
}

fn print_usage() {
    println!("Type commands like:");
    println!("    |");
    println!("    |");
    println!("    -> Add Amir to Sales");
    println!("    |");
    println!("    |");
    println!("    -> Add Sally to Engineering");
    println!("    |");
    println!("    |");
    println!("    -> List all in Marketing");
    println!("    |");
    println!("    |");
    println!("    -> Quit");
}
