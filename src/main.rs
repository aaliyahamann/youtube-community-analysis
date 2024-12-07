mod graph;

fn main() {
    println!("Building the YouTube social network graph...");

    // Load the graph from the dataset file
    let graph = graph::Graph::from_file("data/com-youtube.ungraph.txt");

    // Compute neighbors at distance 2
    println!("\nComputing Neighbors at Distance 2...");
    let neighbors_2 = graph.neighbors_at_distance_2();

    // Save results to CSV
    println!("\nSaving results to file...");
    graph.save_neighbors_at_distance_2(&neighbors_2, "neighbors_at_distance_2.csv");

    // Display top N nodes
    println!("\nDisplaying top nodes by neighbors at distance 2:");
    graph.display_top_neighbors_at_distance_2(&neighbors_2, 10);

    println!("Graph analysis complete!");
}
