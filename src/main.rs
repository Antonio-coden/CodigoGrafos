mod VersionManual;
mod VersionPetgraph;

use std::env;

fn main(){

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("uso:");
        println!("manual");
        println!("petgraph");
        return;
    }

    match args[1].as_str() {
        "manual" => VersionManual::run(),
        "petgraph" => VersionPetgraph::run(),
        _ => println!("Unknown version: {}", args[1]),
    }
}
