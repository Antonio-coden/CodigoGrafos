use std:: collections::VecDeque;
use colored::*;


struct RedTrenes {
    estaciones: Vec<String>,
    adyacencia: Vec<Vec<usize>>,
}

impl RedTrenes {

    fn nueva() -> Self {
        RedTrenes {
            estaciones: Vec::new(),
            adyacencia: Vec::new(),
        }
    }

     fn agregar_estacion(&mut self, nombre: &str) -> usize {

        // El tamaño actual del vector será el índice
        // que ocupará la nueva estación.
        let i = self.estaciones.len();

        // Guarda el nombre.
        self.estaciones.push(nombre.to_string());

        // Crea una lista vacía para sus conexiones.
        self.adyacencia.push(Vec::new());

        // Devuelve el índice generado.
        i
    }

fn mostrar_red(&self) {
        let w = 58usize;
        println!();
        println!("{}", format!("╔{}╗", "═".repeat(w)).yellow());
        println!("{}", format!("║{:^width$}║", "  REDRAIL — RED DE TRENES", width = w).yellow());
        println!("{}", format!("║{:^width$}║", "  Lista de Adyacencia (Vértices y Aristas)", width = w).yellow());
        println!("{}", format!("╠{}╣", "═".repeat(w)).yellow());
        println!("{}", format!("║ {:<4} {:<14} {}", "IDX", "ESTACIÓN", "CONEXIONES DIRECTAS (vías)").yellow());
        println!("{}", format!("╟{}╢", "─".repeat(w)).yellow());
        for (i, est) in self.estaciones.iter().enumerate() {
            let vecinos: Vec<&str> = self.adyacencia[i].iter().map(|&j| self.estaciones[j].as_str()).collect();
            println!("{}", format!("║ [{:<2}] {:<14} → {}", i, est, vecinos.join(", ")).yellow());
            if i < self.estaciones.len() - 1 {
                println!("{}", format!("╟{}╢", "─".repeat(w)).yellow());
            }
        }
        println!("{}", format!("╚{}╝", "═".repeat(w)).yellow());
        let e = self.adyacencia.iter().map(|v| v.len()).sum::<usize>() / 2;
        println!();
        println!("  {} {}   {} {}",
            "Vértices totales V:".bright_cyan(), self.estaciones.len().to_string().bright_white().bold(),
            "Aristas totales E:".bright_cyan(),  e.to_string().bright_white().bold());
        println!();
    }

}



pub fn ejecutar() {

println!("=================================");
    println!("IMPLEMENTACION MANUAL");
    println!("=================================");

    let mut red = RedTrenes::nueva();

    let auroria  = red.agregar_estacion("Auroria");   // 0
    let velstrom = red.agregar_estacion("Velstrom");  // 1
    let nexara   = red.agregar_estacion("Nexara");    // 2
    let dralion  = red.agregar_estacion("Dralion");   // 3
    let korveth  = red.agregar_estacion("Korveth");   // 4
    let myrenth  = red.agregar_estacion("Myrenth");   // 5
    let caldrix  = red.agregar_estacion("Caldrix");   // 6
    let zentova  = red.agregar_estacion("Zentova");   // 7
    let pyloran  = red.agregar_estacion("Pyloran");   // 8
    let thornex  = red.agregar_estacion("Thornex");   // 9

    red.mostrar_red();

}


