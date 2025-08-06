# Community Detection Using Graph

This Rust project demonstrates simple community detection on a network of Twitter usernames using the [petgraph](https://github.com/petgraph/petgraph) crate. It constructs a directed graph from a list of usernames, detects strongly connected components (SCCs) as communities, and prints them, highlighting the largest community.

## Features

- **Graph Construction:** Each Twitter username is a node. Edges represent "retweet" relationships between consecutive usernames.
- **Community Detection:** Uses Kosaraju's algorithm to find strongly connected components (SCCs), which are interpreted as communities.
- **Reporting:** Prints all detected communities and highlights the largest one.

## Usage

1. **Clone the repository:**
   ```sh
   git clone https://github.com/yourusername/Community-Detection-Using-Graph.git
   cd Community-Detection-Using-Graph
   ```

2. **Add your list of Twitter usernames**  
   Edit the `TWITTER_USERNAMES` constant in `src/lib.rs` or the appropriate module.

3. **Build and run:**
   ```sh
   cargo run
   ```

4. **Output:**  
   The program will print all detected communities and the largest community to the console.

## Dependencies

- [petgraph](https://crates.io/crates/petgraph)

## Example Output

```
Community (3 users): ["user1", "user2", "user3"]

🌟 Largest Community (5 users): ["user4", "user5", "user6", "user7", "user8"]
```

## License

MIT