// src/graph.rs

/// Builds a word co-occurrence graph from cleaned hotel reviews

use std::collections::{HashMap, HashSet, VecDeque}; 

/// represens a word co-occurance graph
/// each word is a node and the edges (with weights) represent how often words appeared together 
/// the graph is stored as a nested 'HashMap<String, HashMap<String, usize>>'
/// helps and makes it easier to track neighbors and co-occurance frequency 

pub struct Graph {
    pub edges: HashMap<String, HashMap<String, usize>>,
}

impl Graph {
    /// Creates a new empty Graph
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    /// Adds an edge between two words
    pub fn add_edge(&mut self, word1: &str, word2: &str) {
        if word1 == word2 {
            return; 
        }

        self.edges
            .entry(word1.to_string())
            .or_insert_with(HashMap::new)
            .entry(word2.to_string())
            .and_modify(|weight| *weight += 1)
            .or_insert(1);

        self.edges
            .entry(word2.to_string())
            .or_insert_with(HashMap::new)
            .entry(word1.to_string())
            .and_modify(|weight| *weight += 1)
            .or_insert(1);
    }

    /// Adds all unique word pairs from a single cleaned review
    pub fn add_review(&mut self, words: &[String]) {
        for i in 0..words.len() {
            for j in (i + 1)..words.len() {
                self.add_edge(&words[i], &words[j]);
            }
        }
    }

    /// Prints the top 10 words with the most connections
    pub fn print_top_connected_words(&self) {
        let mut word_degrees: Vec<(&String, usize)> = self.edges
            .iter()
            .map(|(word, neighbors)| (word, neighbors.len()))
            .collect();

        word_degrees.sort_by(|a, b| b.1.cmp(&a.1));

        println!("Top 10 most connected words:");
        for (word, degree) in word_degrees.iter().take(10) {
            println!("{}: {} connections", word, degree);
        }
    }

    /// Finds how many neighbors each word has 
    pub fn degree_centrality(&self) -> HashMap<String, usize> {
        self.edges
            .iter()
            .map(|(word, neighbors)| (word.clone(), neighbors.len()))
            .collect()
    }

    /// Finds the sum of edge weights for each word 
    pub fn weighted_degree_centrality(&self) -> HashMap<String, usize> {
        self.edges
            .iter()
            .map(|(word, neighbors)| {
                let total_weight: usize = neighbors.values().sum();
                (word.clone(), total_weight)
            })
            .collect()
    }

    /// Computes a PageRank score -> how important a word is based on its neighbors
    pub fn pagerank_like_score(&self) -> HashMap<String, f64> {
        let mut scores = HashMap::new();
        for (word, neighbors) in &self.edges {
            let mut score = 0.0;
            for (neighbor, weight) in neighbors {
                let neighbor_degree = self.edges.get(neighbor).map(|n| n.len()).unwrap_or(1);
                score += *weight as f64 / neighbor_degree as f64;
            }
            scores.insert(word.clone(), score);
        }
        scores
    }

    /// Finds clusters of words based on connected components (BFS)
    /// uses BFS to explore connected components 
    /// skips visited words and avoids revisiting nodes 
    /// also each component is treated as a seperate cluster 
    pub fn find_clusters(&self) -> Vec<Vec<String>> {
        let mut visited = HashSet::new();
        let mut clusters = Vec::new();

        for word in self.edges.keys() {
            if !visited.contains(word) {
                let mut cluster = Vec::new();
                let mut queue = VecDeque::new();
                queue.push_back(word.clone());

                while let Some(current_word) = queue.pop_front() {
                    if visited.insert(current_word.clone()) {
                        cluster.push(current_word.clone());
                        if let Some(neighbors) = self.edges.get(&current_word) {
                            for neighbor in neighbors.keys() {
                                if !visited.contains(neighbor) {
                                    queue.push_back(neighbor.clone());
                                }
                            }
                        }
                    }
                }

                if !cluster.is_empty() {
                    clusters.push(cluster);
                }
            }
        }

        clusters
    }
}
