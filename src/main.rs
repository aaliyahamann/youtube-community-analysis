mod graph;

fn main() {
    println!("Building the YouTube social network graph...");

    // Load the graph from the dataset file
    let graph = graph::Graph::from_file("data/com-youtube.ungraph.txt");

    // Display the adjacency list
    graph.display();

    println!("Graph built successfully!");
}