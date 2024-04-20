use petgraph::graph::{NodeIndex,UnGraph};
use petgraph::Direction;
use std::fmt;

#[derive(Debug)]

struct Fighter {
    name : String
}

impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
    }
    }

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}

fn main() 
{
    let mut graph = UnGraph::new_undirected();
    let fighters = [
        Fighter::new("Ryu"),
        Fighter::new("Ken"),
        Fighter::new("Chun-Li"),
        Fighter::new("Guile"),
        Fighter::new("Zangief"),
        Fighter::new("Dhalsim"),
        Fighter::new("Blanka"),
        Fighter::new("E. Honda"),
        Fighter::new("Balrog"),
        Fighter::new("Vega"),
        Fighter::new("Sagat"),
        Fighter::new("M. Bison")
    ];
    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect();
    add_edge(&mut graph, &fighter_nodes, 0, 1);
    add_edge(&mut graph, &fighter_nodes, 0, 2);
    add_edge(&mut graph, &fighter_nodes, 2, 3);
    add_edge(&mut graph, &fighter_nodes, 2, 4);
    add_edge(&mut graph, &fighter_nodes, 2, 5);
    add_edge(&mut graph, &fighter_nodes, 2, 6);

    for (i, &node) in fighter_nodes.iter().enumerate() 
    {
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;
        println!("the closeness of {} is {}", name, closeness);
        match name.as_str()
        {
            "Ryu" => println!("{} has the lowest centrality", name),
            "Ken"|"Chun-Li" => println!("{} has the highest centrality of {:.2} ", name, closeness),
            _=> {}
        }
        println!("------------------------------------")
    }


    
    

}

