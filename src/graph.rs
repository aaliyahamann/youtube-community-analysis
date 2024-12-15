use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

pub type AdjacencyList = HashMap<u32, Vec<u32>>;

//Function to load the graph from a file
pub fn load_graph(filename: &str) -> io::Result<AdjacencyList> {
    let mut graph: AdjacencyList = HashMap::new();

    //Open the file
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    //Process each line
    for line in reader.lines() {
        let line = line?;
        //Ignore comment lines
        if line.starts_with('#') {
            continue;
        }

        //Split the line into two nodes
        let nodes: Vec<&str> = line.split_whitespace().collect();
        if nodes.len() != 2 {
            continue;
        }

        //Parse nodes as u32
        let node1 = nodes[0].parse::<u32>().unwrap();
        let node2 = nodes[1].parse::<u32>().unwrap();

        //Add the edge to the graph
        graph.entry(node1).or_insert_with(Vec::new).push(node2);
        graph.entry(node2).or_insert_with(Vec::new).push(node1); // Undirected graph
    }

    Ok(graph)
}

//Function to load the community data from the community file
pub fn load_communities(filename: &str) -> io::Result<HashMap<u32, u32>> {
    let mut communities: HashMap<u32, u32> = HashMap::new();
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut community_id = 0;

    //Process each line (each community)
    for line in reader.lines() {
        let line = line?;
        let members: Vec<&str> = line.split_whitespace().collect();
        
        //Assign the same community ID to all members
        for &member in &members {
            let user_id = member.parse::<u32>().unwrap();
            communities.insert(user_id, community_id);
        }
        community_id += 1;
    }

    Ok(communities)
}

//Count the total number of edges in the graph
pub fn count_edges(graph: &AdjacencyList) -> usize {
    graph.values().map(|neighbors| neighbors.len()).sum::<usize>() / 2
}

//Calculate the degree of each node
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
