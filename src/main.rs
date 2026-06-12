mod VersionManual;
mod VersionPetgraph;

use std::env;

fn main(){

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Use para ejeuctar:");
        println!("cargo run manual");
        println!("cargo run petgraph");
        return;
    }

    match args[1].as_str() {
        "manual" => VersionManual::ejecutar(),
        "petgraph" => VersionPetgraph::ejecutar(),
        _ => println!("Opcion invalida"),
    }
}
