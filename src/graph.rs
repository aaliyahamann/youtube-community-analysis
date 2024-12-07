use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions}; 
use std::io::{self, BufRead, Write};
use std::path::Path;

pub struct Graph {
    adjacency_list: HashMap<i32, Vec<i32>>,
}

impl Graph {
    // Create a new, empty graph
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    // Add an edge to the graph
    pub fn add_edge(&mut self, node1: i32, node2: i32) {
        self.adjacency_list
            .entry(node1)
            .or_insert_with(Vec::new)
            .push(node2);
        self.adjacency_list
            .entry(node2)
            .or_insert_with(Vec::new)
            .push(node1); // Undirected graph
    }

    // Build the graph from a file
    pub fn from_file(file_path: &str) -> Self {
        let mut graph = Graph::new();

        if let Ok(lines) = read_lines(file_path) {
            for line in lines {
                if let Ok(edge) = line {
                    let nodes: Vec<i32> = edge
                        .split_whitespace()
                        .filter_map(|n| n.parse::<i32>().ok())
                        .collect();

                    if nodes.len() == 2 {
                        graph.add_edge(nodes[0], nodes[1]);
                    }
                }
            }
        }
        graph
    }

    // Analyze the graph
    pub fn analyze(&self) {
        let total_nodes = self.adjacency_list.len();
        let total_edges: usize = self.adjacency_list.values().map(|edges| edges.len()).sum::<usize>() / 2;

        // Compute degrees
        let degrees: Vec<usize> = self.adjacency_list.values().map(|edges| edges.len()).collect();
        let max_degree = degrees.iter().max().unwrap_or(&0);
        let min_degree = degrees.iter().min().unwrap_or(&0);
        let avg_degree = degrees.iter().sum::<usize>() as f64 / total_nodes as f64;

        // Display results
        println!("Total Nodes: {}", total_nodes);
        println!("Total Edges: {}", total_edges);
        println!("Maximum Degree: {}", max_degree);
        println!("Minimum Degree: {}", min_degree);
        println!("Average Degree: {:.2}", avg_degree);
    }

    // Compute the number of neighbors at distance 2 for each node
    pub fn neighbors_at_distance_2(&self) -> HashMap<i32, usize> {
        let mut result = HashMap::new();
        let mut counter = 0;

        for (node, neighbors) in &self.adjacency_list {
            let mut distance_2_neighbors = HashSet::new();

            for &neighbor in neighbors {
                if let Some(second_neighbors) = self.adjacency_list.get(&neighbor) {
                    for &second_neighbor in second_neighbors {
                        if second_neighbor != *node {
                            distance_2_neighbors.insert(second_neighbor);
                        }
                    }
                }
            }

            result.insert(*node, distance_2_neighbors.len());
            counter += 1;

            // Log progress every 10,000 nodes
            if counter % 10_000 == 0 {
                println!("Processed {} nodes...", counter);
            }
        }

        result
    }

    // Save neighbors at distance 2 to a CSV file
    pub fn save_neighbors_at_distance_2(
        &self,
        results: &HashMap<i32, usize>,
        file_path: &str,
    ) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .expect("Unable to open file");

        writeln!(file, "Node,NeighborsAtDistance2").expect("Unable to write header");

        for (node, count) in results {
            writeln!(file, "{},{}", node, count).expect("Unable to write data");
        }

        println!("Results saved to {}", file_path);
    }

    // Display top N nodes by neighbors at distance 2
    pub fn display_top_neighbors_at_distance_2(
        &self,
        results: &HashMap<i32, usize>,
        top_n: usize,
    ) {
        let mut sorted_results: Vec<_> = results.iter().collect();
        sorted_results.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count descending

        println!("Top {} Nodes by Neighbors at Distance 2:", top_n);
        for (node, count) in sorted_results.into_iter().take(top_n) {
            println!("Node {}: {} neighbors at distance 2", node, count);
        }
    }
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
