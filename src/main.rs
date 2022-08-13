use std::env;
use std::process;

const VERSION: &str = "0.1.0";
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() <= 1 {
        eprintln!("No command provided, add the help argumen");
        process::exit(1);
    }

    match args[1].to_lowercase().as_str() {
        "version" | "v" => println!("{}", VERSION),
        "build" | "b" => println!("b"), //generate the static site
        _ => println!("h"),
    }
}