use community_detection::TWITTER_USERNAMES;
use petgraph::algo::kosaraju_scc;
use petgraph::prelude::*;
use std::collections::HashMap;

fn main() {
    // Create a directed graph where nodes are Twitter usernames
    let mut graph = DiGraph::<&str, &str>::new();
    let mut node_indices = HashMap::new();

    // Add nodes and edges based on consecutive usernames
    for window in TWITTER_USERNAMES.windows(2) {
        let user = window[0];
        let mention = window[1];

        let user_idx = *node_indices.entry(user).or_insert_with(|| graph.add_node(user));
        let mention_idx = *node_indices.entry(mention).or_insert_with(|| graph.add_node(mention));

        graph.add_edge(user_idx, mention_idx, "retweets");
    }

    // Find strongly connected components (communities)
    let components = kosaraju_scc(&graph);

    // Print all communities
    for community in &components {
        let usernames: Vec<&str> = community.iter().map(|&idx| graph[idx]).collect();
        println!("\nCommunity ({} users): {:?}", usernames.len(), usernames);
    }

    // Find and print the largest community
    if let Some(largest) = components.iter().max_by_key(|c| c.len()) {
        let largest_usernames: Vec<&str> = largest.iter().map(|&idx| graph[idx]).collect();
        println!("\n🌟 Largest Community ({} users): {:?}", largest_usernames.len(), largest_usernames);
    }
}