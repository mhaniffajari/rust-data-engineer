use petgraph::algo::dijkstra;
use petgraph::prelude::*;
fn main() {
    let mut graph = Graph::<&str,u32,Undirected>::new_undirected();
    let belem_tower = graph.add_node("Belem Tower");
    let monestery = graph.add_node("monestry");
    let lx_factory = graph.add_node("lx_factory");
    let commerce_square = graph.add_node("Commerce Square");
    let lisbon_cathedral = graph.add_node("Lisbon Cathedral");

    graph.extend_with_edges([
        (belem_tower,monestery,1),
        (belem_tower,lx_factory,3),
        (belem_tower,commerce_square,7),
        (monestery,lx_factory,3),
        (monestery,commerce_square,6),
        (lx_factory,commerce_square,5),
        (commerce_square,lisbon_cathedral,1)
    ]);
    let node_map = dijkstra(&graph, belem_tower, Some(lisbon_cathedral), |e| *e.weight());
    if let Some(distance) = node_map.get(&lisbon_cathedral){
        println!("The shortest distance from Belem Tower to Lisbon Catedhral is {} km",distance);
    }else{
        println!("No route found from Belem Tower to Lisbon Cathedral.");
    }
}
