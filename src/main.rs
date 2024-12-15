use std::collections::HashMap;

mod graph;

fn main() {
    let graph_filename = "data/com-youtube.ungraph.txt";
    let community_filename = "data/com-youtube.all.cmty.txt";

    //Load the graph
    match graph::load_graph(graph_filename) {
        Ok(graph) => {
            println!("Graph loaded successfully with {} nodes.", graph.len());

            //Count the number of edges
            let edge_count = graph::count_edges(&graph);
            println!("The graph has {} edges.", edge_count);

            //Calculate the degree distribution
            let degrees = graph::calculate_degrees(&graph);

            //Print the degree distribution summary
            println!("Degree distribution calculated:");
            println!("Minimum degree: {}", degrees.values().min().unwrap());
            println!("Maximum degree: {}", degrees.values().max().unwrap());
            println!(
                "Average degree: {:.2}",
                degrees.values().sum::<usize>() as f64 / graph.len() as f64
            );

            //Find the node with the highest degree
            if let Some((node, degree)) = graph::find_highest_degree_node(&graph) {
                println!(
                    "The node with the highest degree is {} with {} connections.",
                    node, degree
                );
            }

            //Load the communities
            match graph::load_communities(community_filename) {
                Ok(communities) => {
                    println!("\nCommunities loaded successfully.");

                    //Analyze the communities
                    let mut community_sizes = communities.values().fold(HashMap::new(), |mut acc, &community| {
                        *acc.entry(community).or_insert(0) += 1;
                        acc
                    });

                    //Filter out small communities (<3 members)
                    community_sizes.retain(|_, &mut size| size >= 3);

                    //Sort communities by size
                    let mut sorted_communities: Vec<_> = community_sizes.iter().collect();
                    sorted_communities.sort_by_key(|&(_, size)| std::cmp::Reverse(*size));

                    //Print the community sizes
                    println!("\nTop Communities (size >= 3):");
                    for (community, size) in sorted_communities.iter().take(20) {
                        println!("Community {}: {} members", community, size);
                    }

                    //Find the largest community
                    if let Some((largest_community, largest_size)) = sorted_communities.first() {
                        println!(
                            "\nThe largest community is Community {} with {} members.",
                            largest_community, largest_size
                        );
                    }
                }
                Err(e) => eprintln!("Error loading communities: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Error loading graph: {}", e);
        }
    }
}
