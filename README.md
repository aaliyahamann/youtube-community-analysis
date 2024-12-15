# youtube-community-analysis
DS 210 Final Project: Analyzing community structures within the YouTube social network and identify influential nodes within and across these communities using Rust

# YouTube Social Network Analysis

## Project Overview
This project analyzes the **YouTube social network** using graph algorithms to detect **community structures** and identify **influential nodes**. The analysis is based on the **SNAP YouTube dataset**, which contains user connections and predefined communities.

## Dataset
- **Source**: [SNAP YouTube Social Network Dataset](https://snap.stanford.edu/data/com-Youtube.html)
- **Files Used**:
   - `com-youtube.ungraph.txt`: User connections (nodes and edges).
   - `com-youtube.all.cmty.txt`: Predefined communities.

## Features
- **Graph Loading**: Reads the YouTube network into an adjacency list.
- **Degree Distribution**: Calculates the degree of each node to identify highly connected users.
- **Breadth-First Search (BFS)**: Computes shortest paths and verifies small-world properties.
- **Jaccard Similarity**: Measures similarity between nodes based on neighbors.
- **Betweenness Centrality**: Identifies bridge nodes connecting communities.
- **Community Analysis**: Analyzes and filters the largest communities.

## How to Run the Code

### Prerequisites
- **Rust**: Install from [rust-lang.org](https://www.rust-lang.org/tools/install).

### Steps
1. **Clone the Repository**:
   ```bash
   git clone nano https://github.com/aaliyahamann/youtube-community-analysis.git
   cd youtube-community-analysis
2. **Prepare the Dataset**:
- Place `com-youtube.ungraph.txt` and `com-youtube.all.cmty.txt` in a `data/` folder in the project root.
3. **Build the Project**:
- cargo build
4. **Run the Program**:
- cargo run
5. **Run tests**:
- Ensure that all functions work as intended
- cargo test

##Example Output
Graph loaded successfully with 1134890 nodes.
The graph has 2987624 edges.

Degree distribution calculated:
Minimum degree: 1
Maximum degree: 28754
Average degree: 5.27
The node with the highest degree is 1072 with 28754 connections.

Communities loaded successfully.

Top Communities (size >= 3):
Community 267: 1460 members
Community 821: 141 members
...

The largest community is Community 267 with 1460 members.

Running BFS from node 1 to calculate shortest paths:
Node 1 can reach 1134890 nodes.
Average shortest path length: 4.16

Node 1 has 7151 neighbors at distance 2.

Jaccard similarity between nodes 1 and 2: 0.004 (Low)

Calculating Betweenness Centrality for top 50 highest degree nodes...
[---------------------------------------------------] 100% 50/50

Top 5 Nodes by Betweenness Centrality:
Node 363: Betweenness Centrality = 731888.5341
Node 1072: Betweenness Centrality = 665940.0458
Node 1034018: Betweenness Centrality = 895436.4483
Node 663560: Betweenness Centrality = 854459.6228
Node 106: Betweenness Centrality = 81856.4557

