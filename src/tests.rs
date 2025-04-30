/// this part tests for data cleaning and graph construction functionality 


#[cfg(test)]
mod tests {
    use super::super::data::{Review, clean_text};
    use super::super::graph::Graph;
/// tests that 'clean_text' removes punctuation and lowercase inputs 
    #[test]
    fn test_clean_text_basic() {
        let text = "The hotel was GREAT! Very clean and spacious.";
        let cleaned = clean_text(text);
        assert!(cleaned.contains(&"hotel".to_string()));
        assert!(cleaned.contains(&"great".to_string()));
        assert!(!cleaned.contains(&"!".to_string())); // punctuation removed
    }
/// tests that 'add_edge' correctly adds and increments undirected edge weights 
    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();
        graph.add_edge("room", "clean");
        graph.add_edge("room", "clean"); // duplicate edge

        assert_eq!(graph.edges["room"]["clean"], 2);
        assert_eq!(graph.edges["clean"]["room"], 2);
    }
/// tests that 'add_review' connects all unique word pairs from a single review 
    #[test]
    fn test_add_review() {
        let mut graph = Graph::new();
        let review_words = vec![
            "room".to_string(),
            "clean".to_string(),
            "spacious".to_string(),
        ];
        graph.add_review(&review_words);

        // should include edges: room-clean, room-spacious, clean-spacious
        assert_eq!(graph.edges["room"]["clean"], 1);
        assert_eq!(graph.edges["room"]["spacious"], 1);
        assert_eq!(graph.edges["clean"]["spacious"], 1);
    }
/// tests that 'degree_centrality' correctly counts the number of connections 
    #[test]
    fn test_degree_centrality() {
        let mut graph = Graph::new();
        graph.add_edge("a", "b");
        graph.add_edge("a", "c");

        let centrality = graph.degree_centrality();
        assert_eq!(centrality["a"], 2);
        assert_eq!(centrality["b"], 1);
        assert_eq!(centrality["c"], 1);
    }
/// tests that 'find_clusters' returns TWO seperate word clusters 
    #[test]
    fn test_find_clusters_small() {
        let mut graph = Graph::new();
        // Cluster 1
        graph.add_edge("a", "b");
        graph.add_edge("b", "c");

        // Cluster 2
        graph.add_edge("x", "y");

        let clusters = graph.find_clusters();
        assert_eq!(clusters.len(), 2);
    }
}
