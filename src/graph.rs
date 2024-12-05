use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Struct to represent the graph as an adjacency list
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

    // Display the adjacency list (previously implemented feature)
    pub fn display(&self) {
        for (node, edges) in &self.adjacency_list {
            println!("Node {}: {:?}", node, edges);
        }
    }

    // New: Analyze the graph and compute its properties
    pub fn analyze(&self) {
        let total_nodes = self.adjacency_list.len();
        let total_edges: usize = self.adjacency_list.values().map(|edges| edges.len()).sum::<usize>() / 2;

        // Compute degrees
        let degrees: Vec<usize> = self.adjacency_list.values().map(|edges| edges.len()).collect();
        let max_degree = degrees.iter().max().unwrap_or(&0);
        let min_degree = degrees.iter().min().unwrap_or(&0);

        // Display results
        println!("Total Nodes: {}", total_nodes);
        println!("Total Edges: {}", total_edges);
        println!("Maximum Degree: {}", max_degree);
        println!("Minimum Degree: {}", min_degree);
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
