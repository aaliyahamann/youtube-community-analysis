mod graph;

fn main() {
    println!("Building the YouTube social network graph...");

    // Load the graph from the dataset file
    let graph = graph::Graph::from_file("data/com-youtube.ungraph.txt");

    // Display the adjacency list (previous feature)
    println!("Adjacency List:");
    graph.display();

    // Analyze the graph (new feature)
    println!("\nGraph Analysis:");
    graph.analyze();

    println!("Graph analysis complete!");
}
