use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use indicatif::ProgressBar;

pub type AdjacencyList = HashMap<u32, Vec<u32>>;

//Function to load the graph from a file
pub fn load_graph(filename: &str) -> io::Result<AdjacencyList> {
    let mut graph: AdjacencyList = HashMap::new();
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') {
            continue;
        }
        let nodes: Vec<&str> = line.split_whitespace().collect();
        if nodes.len() != 2 {
            continue;
        }
        let node1 = nodes[0].parse::<u32>().unwrap();
        let node2 = nodes[1].parse::<u32>().unwrap();
        graph.entry(node1).or_insert_with(Vec::new).push(node2);
        graph.entry(node2).or_insert_with(Vec::new).push(node1); // Undirected graph
    }

    Ok(graph)
}

//Function to load the community data
pub fn load_communities(filename: &str) -> io::Result<HashMap<u32, u32>> {
    let mut communities: HashMap<u32, u32> = HashMap::new();
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut community_id = 0;
    for line in reader.lines() {
        let line = line?;
        let members: Vec<&str> = line.split_whitespace().collect();
        for &member in &members {
            let user_id = member.parse::<u32>().unwrap();
            communities.insert(user_id, community_id);
        }
        community_id += 1;
    }

    Ok(communities)
}

//Count the total number of edges
pub fn count_edges(graph: &AdjacencyList) -> usize {
    graph.values().map(|neighbors| neighbors.len()).sum::<usize>() / 2
}

//Calculate degree of each node
pub fn calculate_degrees(graph: &AdjacencyList) -> HashMap<u32, usize> {
    graph.iter().map(|(node, neighbors)| (*node, neighbors.len())).collect()
}

//Find the node with the highest degree
pub fn find_highest_degree_node(graph: &AdjacencyList) -> Option<(u32, usize)> {
    graph
        .iter()
        .map(|(node, neighbors)| (*node, neighbors.len()))
        .max_by_key(|&(_, degree)| degree)
}

//Breadth-First Search for shortest paths
pub fn bfs_shortest_path(graph: &AdjacencyList, source: u32) -> HashMap<u32, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((source, 0));
    visited.insert(source);

    while let Some((node, dist)) = queue.pop_front() {
        distances.insert(node, dist);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if visited.insert(neighbor) {
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }
    }
    distances
}

//Degree distribution of neighbors at distance 2
pub fn degree_distance_2(graph: &AdjacencyList, node: u32) -> usize {
    let mut distance_2_neighbors: HashSet<u32> = HashSet::new();
    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            if let Some(second_neighbors) = graph.get(&neighbor) {
                for &second_neighbor in second_neighbors {
                    if second_neighbor != node && !neighbors.contains(&second_neighbor) {
                        distance_2_neighbors.insert(second_neighbor);
                    }
                }
            }
        }
    }
    distance_2_neighbors.len()
}

//Jaccard similarity
pub fn jaccard_similarity(graph: &AdjacencyList, node1: u32, node2: u32) -> f64 {
    let empty_vec = Vec::new();
    let neighbors1: HashSet<_> = graph.get(&node1).unwrap_or(&empty_vec).iter().cloned().collect();
    let neighbors2: HashSet<_> = graph.get(&node2).unwrap_or(&empty_vec).iter().cloned().collect();

    let intersection = neighbors1.intersection(&neighbors2).count();
    let union = neighbors1.union(&neighbors2).count();

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

#[derive(Debug)]
pub enum SimilarityLevel {
    High,
    Medium,
    Low,
}

pub fn categorize_similarity(score: f64) -> SimilarityLevel {
    if score > 0.7 {
        SimilarityLevel::High
    } else if score > 0.3 {
        SimilarityLevel::Medium
    } else {
        SimilarityLevel::Low
    }
}

//Betweenness centrality for highest degree nodes
pub fn betweenness_centrality_top_nodes(graph: &AdjacencyList, top_n: usize) -> HashMap<u32, f64> {
    let mut centrality = HashMap::new();

    let mut degrees: Vec<_> = graph.iter().map(|(node, neighbors)| (*node, neighbors.len())).collect();
    degrees.sort_by_key(|&(_, degree)| std::cmp::Reverse(degree));
    let top_nodes: Vec<u32> = degrees.into_iter().take(top_n).map(|(node, _)| node).collect();

    let progress_bar = ProgressBar::new(top_nodes.len() as u64);

    for &source in &top_nodes {
        let mut stack = Vec::new();
        let mut paths = HashMap::new();
        let mut distance = HashMap::new();
        let mut predecessors = HashMap::new();

        for &node in graph.keys() {
            paths.insert(node, 0);
            distance.insert(node, -1);
            predecessors.insert(node, Vec::new());
        }
        paths.insert(source, 1);
        distance.insert(source, 0);

        let mut queue = VecDeque::new();
        queue.push_back(source);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            if let Some(neighbors) = graph.get(&v) {
                for &neighbor in neighbors {
                    if distance[&neighbor] == -1 {
                        distance.insert(neighbor, distance[&v] + 1);
                        queue.push_back(neighbor);
                    }
                    if distance[&neighbor] == distance[&v] + 1 {
                        paths.insert(neighbor, paths[&neighbor] + paths[&v]);
                        predecessors.get_mut(&neighbor).unwrap().push(v);
                    }
                }
            }
        }

        let mut dependencies = HashMap::new();
        for &node in graph.keys() {
            dependencies.insert(node, 0.0);
        }

        while let Some(w) = stack.pop() {
            for &v in &predecessors[&w] {
                let delta = (paths[&v] as f64 / paths[&w] as f64) * (1.0 + dependencies[&w]);
                *dependencies.get_mut(&v).unwrap() += delta;
            }
            if w != source {
                *centrality.entry(w).or_insert(0.0) += dependencies[&w];
            }
        }

        progress_bar.inc(1);
    }

    progress_bar.finish_with_message("Betweenness centrality calculation completed!");
    centrality
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_graph() {
        // Test graph loaded from a file
        let graph = load_graph("data/test-graph.txt").unwrap();
        assert_eq!(graph.len(), 4);
        assert_eq!(graph[&1], vec![2, 3]);
        assert_eq!(graph[&2], vec![1, 4]);
    }

    #[test]
    fn test_load_communities() {
        // Test community data loaded from a file
        let communities = load_communities("data/test-communities.txt").unwrap();
        assert_eq!(communities.len(), 4);
        assert_eq!(communities[&1], 0);
        assert_eq!(communities[&2], 0);
        assert_eq!(communities[&3], 1);
        assert_eq!(communities[&4], 1);
    }

    #[test]
    fn test_count_edges() {
        let mut graph = AdjacencyList::new();
        graph.insert(1, vec![2, 3]);
        graph.insert(2, vec![1]);
        graph.insert(3, vec![1]);
        assert_eq!(count_edges(&graph), 2);
    }

    #[test]
    fn test_calculate_degrees() {
        let mut graph = AdjacencyList::new();
        graph.insert(1, vec![2, 3]);
        graph.insert(2, vec![1]);
        graph.insert(3, vec![1]);

        let degrees = calculate_degrees(&graph);
        assert_eq!(degrees[&1], 2);
        assert_eq!(degrees[&2], 1);
        assert_eq!(degrees[&3], 1);
    }

    #[test]
    fn test_find_highest_degree_node() {
        let mut graph = AdjacencyList::new();
        graph.insert(1, vec![2, 3]);
        graph.insert(2, vec![1]);
        graph.insert(3, vec![1]);

        let highest = find_highest_degree_node(&graph).unwrap();
        assert_eq!(highest, (1, 2));
    }

    #[test]
    fn test_bfs_shortest_path() {
        let mut graph = AdjacencyList::new();
        graph.insert(1, vec![2, 3]);
        graph.insert(2, vec![1, 4]);
        graph.insert(3, vec![1]);
        graph.insert(4, vec![2]);

        let distances = bfs_shortest_path(&graph, 1);
        assert_eq!(distances[&1], 0);
        assert_eq!(distances[&2], 1);
        assert_eq!(distances[&3], 1);
        assert_eq!(distances[&4], 2);
    }

    #[test]
    fn test_degree_distance_2() {
        let mut graph = AdjacencyList::new();
        graph.insert(1, vec![2, 4]);
        graph.insert(2, vec![1, 3]);
        graph.insert(3, vec![2, 4]);
        graph.insert(4, vec![1, 3]);

        assert_eq!(degree_distance_2(&graph, 1), 1);
        assert_eq!(degree_distance_2(&graph, 2), 1);
    }

    #[test]
    fn test_jaccard_similarity() {
        let mut graph = AdjacencyList::new();
        graph.insert(1, vec![3, 4]);
        graph.insert(2, vec![3, 4]);
        graph.insert(3, vec![1, 2]);
        graph.insert(4, vec![1, 2]);

        let similarity = jaccard_similarity(&graph, 1, 2);
        assert!((similarity - 1.0).abs() < 1e-5); 
    }

    #[test]
    fn test_betweenness_centrality_top_nodes() {
        let mut graph = AdjacencyList::new();
        graph.insert(1, vec![2, 4]);
        graph.insert(2, vec![1, 3, 4, 5]);
        graph.insert(3, vec![2, 5]);
        graph.insert(4, vec![1, 2]);
        graph.insert(5, vec![2, 3]);

        let centrality = betweenness_centrality_top_nodes(&graph, 3);

        assert!(centrality[&2] > 0.0);
        assert!(centrality[&2] > centrality[&1]);
        assert!(centrality[&2] > centrality[&3]);
    }
}
