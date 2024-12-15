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
                distance_2_neighbors.extend(second_neighbors);
            }
        }
    }
    distance_2_neighbors.len()
}

//Jaccard similarity
pub fn jaccard_similarity(graph: &AdjacencyList, node1: u32, node2: u32) -> f64 {
    let empty_vec = Vec::new();
    let neighbors1 = graph.get(&node1).unwrap_or(&empty_vec);
    let neighbors2 = graph.get(&node2).unwrap_or(&empty_vec);

    let set1: HashSet<_> = neighbors1.iter().cloned().collect();
    let set2: HashSet<_> = neighbors2.iter().cloned().collect();

    let intersection = set1.intersection(&set2).count();
    let union = set1.union(&set2).count();

    if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
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
