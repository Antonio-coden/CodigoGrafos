use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::algo::astar;
//importar el BFS para poder darle un poco mas de dinamismo a la version de petgraph.
use petgraph::visit::Bfs;


//Esta funcion hace la tarea mostar las estaciones y sus vias o aristas 
// conectadas, es decir, muestra la lista de adyacencia del grafo.
fn mostrar_adyacencia(
    red: &UnGraph<&str, ()>
) {

    println!("\nLISTA DE ADYACENCIA\n");

    for nodo in red.node_indices() {

        print!("{} -> ", red[nodo]);

        let vecinos: Vec<_> =
            red.neighbors(nodo)
               .map(|v| red[v])
               .collect();

        println!("{}", vecinos.join(", "));
    }
}

//Muestra la lista de Estaciones en el orden que fueron visitadas por el 
// algoritmo BFS,
fn mostrar_bfs(
    red: &UnGraph<&str, ()>,
    origen: NodeIndex,
) {

    println!("\nRECORRIDO BFS\n");

    let mut bfs = Bfs::new(red, origen);

    let mut paso = 1;

    while let Some(nx) = bfs.next(red) {

        println!(
            "{}. {}",
            paso,
            red[nx]
        );

        paso += 1;
    }
}

//hace la tarea de buscar la ruta mas corta entre dos estaciones 
//usando el algoritmo A*,
fn buscar_ruta(
    red: &UnGraph<&str, ()>,
    origen: NodeIndex,
    destino: NodeIndex,
) {

    if let Some((_costo, camino)) = astar(
        red,
        origen,
        |n| n == destino,
        |_| 1,
        |_| 0,
    ) {

        println!();
        
    println!("\nRUTA MINIMA\n");

    println!("Origen  : {}", red[origen]);
    println!("Destino : {}", red[destino]);

    println!();

    for nodo in &camino {

    println!("{}", red[*nodo]);

    if *nodo != destino {
        println!("  ↓");
    }
    }

    println!();
    println!("Saltos: {}", camino.len() - 1);

        println!();
    }
}

pub fn ejecutar() {
    

    println!("=================================");
    println!("IMPLEMENTACION PETGRAPH");
    println!("=================================");

    //Crea un grafo no dirigido con nodos de tipo &str vacios y
    //aristas sin peso.
    let mut red = UnGraph::<&str, ()>::new_undirected();

    //Se agrega las esatciones llamando al metodo add_node del grafo,
    //el cual devuelve un NodeIndex que es un identificador unico para 
    //cada nodo.
    let auroria  = red.add_node("Auroria");
    let velstrom = red.add_node("Velstrom");
    let nexara   = red.add_node("Nexara");
    let dralion  = red.add_node("Dralion");
    let korveth  = red.add_node("Korveth");
    let myrenth  = red.add_node("Myrenth");
    let caldrix  = red.add_node("Caldrix");
    let zentova  = red.add_node("Zentova");
    let pyloran  = red.add_node("Pyloran");
    let thornex  = red.add_node("Thornex");

    //Se agregan las conexiones entre estaciones usando el metodo add_edge del grafo,
    //el cual toma dos NodeIndex y un peso (en este caso se usa () para
    red.add_edge(auroria, velstrom, ());
    red.add_edge(velstrom, nexara, ());
    red.add_edge(nexara, dralion, ());
    red.add_edge(dralion, korveth, ());
    red.add_edge(korveth, myrenth, ());
    red.add_edge(myrenth, caldrix, ());
    red.add_edge(caldrix, zentova, ());
    red.add_edge(zentova, auroria, ());

    //Igual se hace un centro en este caso igual Pyloran
    red.add_edge(pyloran, auroria, ());
    red.add_edge(pyloran, nexara, ());
    red.add_edge(pyloran, korveth, ());
    red.add_edge(pyloran, caldrix, ());

    //Y se hace una estacion aislada que solo se conecta con dos estaciones,
    //en este caso Thornex
    red.add_edge(thornex, dralion, ());
    red.add_edge(thornex, velstrom, ());

    println!("\nESTADISTICAS");
    println!("Nodos   : {}", red.node_count());
    println!("Aristas : {}", red.edge_count());

mostrar_adyacencia(&red);
buscar_ruta(&red, auroria, myrenth);
mostrar_bfs(&red, auroria);
}