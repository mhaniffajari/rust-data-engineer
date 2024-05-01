use textwrap::fill;

// The PageRank struct holds the damping factor and the number of iterations to run the algorithm.
struct PageRank {
    damping: f64,
    iterations: usize,
}

impl PageRank {
    // The new function creates a new instance of the PageRank struct.
    fn new(damping: f64, iterations: usize) -> Self {
        Self { damping, iterations }
    }

    // The rank function calculates and returns the PageRank for each node in the graph.
    fn rank(&self, graph: &Vec<Vec<usize>>) -> Vec<f64> {
        // The number of nodes in the graph.
        let n = graph.len();

        // The initial PageRank value for each node.
        let mut ranks = vec![1.0 / (n as f64); n];

        // Iterates the specified number of times.
        for _ in 0..self.iterations {
            // A new vector to hold the updated PageRank values.
            let mut new_ranks = vec![0.0; n];

            // Iterates over each node and its edges in the graph.
            for (node, edges) in graph.iter().enumerate() {
                // The amount of PageRank value this node contributes to its linked nodes.
                let contribution = ranks[node] / (edges.len() as f64);

                // Distributes the PageRank value to the linked nodes.
                for &edge in edges {
                    new_ranks[edge] += contribution;
                }
            }

            // Updates the PageRank values using the damping factor.
            for rank in &mut new_ranks {
                *rank = *rank * self.damping + (1.0 - self.damping) / (n as f64);
            }

            // Replaces the old PageRank values with the new ones.
            ranks = new_ranks;
        }

        // Returns the final PageRank values.
        ranks
    }
}



fn main() {
    let graph = vec![
        vec![1, 2],
        vec![2],
        vec![0, 3],
        vec![0],
    ];
    let names = vec!["RCTI","MNC","SCTV","ANTV"];
    let pagerank = PageRank::new(0.85,100);
    let ranks = pagerank.rank(&graph);
    for (i,rank) in ranks.iter().enumerate(){
        println!("{}: {}",names[i],fill(&format!("{:.3}",rank),30));
    }
    let explanation ="PageRank is link analysis alorithm used by Google";
    println!("{}",fill(explanation,78));

}
