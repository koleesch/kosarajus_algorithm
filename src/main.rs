use std::{collections::HashMap};

use kosarajus_algorithm::TWITTER_USERNAMES;
use petgraph::algo::kosaraju_scc;
use petgraph::graph::DiGraph;

fn main() {
    // Create a new directed graph
    let mut graph = DiGraph::<&str, &str>::new();

    // Create a Hashmap to store node indices by user name
    let mut nodes = HashMap::new();

    // iterate over the data to populate the graph
    for window in TWITTER_USERNAMES.windows(2) {
        let user = window[0];
        let mention = window[1];

        // add nodes to the graph and the hashmap
        let user_node = *nodes.entry(user).or_insert_with(|| graph.add_node(user));
        let mention_node = *nodes
            .entry(mention)
            .or_insert_with(|| graph.add_node(mention));

        // add edge to the node
        graph.add_edge(user_node, mention_node, "retweets");
    }

    // use kosarajus algorithm to find strongly connected components
    let scc = kosaraju_scc(&graph);
    for component in scc {
        println!("{} nodes in community discovered", component.len());
        let usernames: Vec<&str> = component
            .iter()
            .map(|&node_index| graph[node_index])
            .collect();
        println!("Usernames: {:?}", usernames);
    }
}
