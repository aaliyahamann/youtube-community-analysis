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

            //Run BFS for Six Degrees of Separation
            let sample_node = 1;
            println!("\nRunning BFS from node {} to calculate shortest paths:", sample_node);
            let distances = graph::bfs_shortest_path(&graph, sample_node);
            println!("Node {} can reach {} nodes.", sample_node, distances.len());
            println!(
                "Average shortest path length: {:.2}",
                distances.values().sum::<usize>() as f64 / distances.len() as f64
            );

            //Degree distribution at distance 2
            let degree_2 = graph::degree_distance_2(&graph, sample_node);
            println!(
                "\nNode {} has {} neighbors at distance 2.",
                sample_node, degree_2
            );

            //Jaccard similarity between two nodes
            let node_a = 1;
            let node_b = 2;
            let similarity = graph::jaccard_similarity(&graph, node_a, node_b);
            let similarity_category = graph::categorize_similarity(similarity);
            println!(
                "\nJaccard similarity between nodes {} and {}: {:.3} ({:?})",
                node_a, node_b, similarity, similarity_category
            );

            //Calculate betweenness centrality for nodes with highest degree
            println!("\nCalculating Betweenness Centrality for top 50 highest degree nodes...");
            let betweenness = graph::betweenness_centrality_top_nodes(&graph, 50);
            
            //Print the top 5 nodes by betweenness centrality
            let mut sorted_betweenness: Vec<_> = betweenness.iter().collect();
            sorted_betweenness.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
            
            println!("\nTop 5 Nodes by Betweenness Centrality:");
            for (node, centrality) in sorted_betweenness.iter().take(5) {
                println!("Node {}: Betweenness Centrality = {:.4}", node, centrality);
            }
        }
        Err(e) => {
            eprintln!("Error loading graph: {}", e);
        }
    }
}
