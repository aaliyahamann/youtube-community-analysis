mod graph;

fn main() {
    println!("Building the YouTube social network graph...");

    // Load the graph from the dataset file
    let graph = graph::Graph::from_file("data/com-youtube.ungraph.txt");

    // Analyze the graph
    println!("\nGraph Analysis:");
    graph.analyze();

    // Detect communities
    println!("\nDetecting Communities...");
    let communities = graph.detect_communities();
    println!("Number of Communities Detected: {}", communities.len());

    // Compute Degree Centrality
    println!("\nComputing Degree Centrality...");
    let degree_centrality = graph.degree_centrality();

    // // Compute Neighbors at Distance 2
    // println!("\nComputing Neighbors at Distance 2...");
    // let neighbors_2 = graph.neighbors_at_distance_2();

    // // Save results to CSV
    // println!("\nSaving results to file...");
    // graph.save_neighbors_at_distance_2(&neighbors_2, "neighbors_at_distance_2.csv");

    // // Display top N nodes by neighbors at distance 2
    // println!("\nDisplaying top nodes by neighbors at distance 2:");
    // graph.display_top_neighbors_at_distance_2(&neighbors_2, 10);

    // println!("\nTop Nodes by Degree Centrality:");
    // let mut top_degree: Vec<_> = degree_centrality.iter().collect();
    // top_degree.sort_by(|a, b| b.1.cmp(&a.1));
    // for (node, centrality) in top_degree.iter().take(10) {
    //     println!("Node {}: Degree Centrality {}", node, centrality);
    // }

    println!("Graph analysis complete!");
}
