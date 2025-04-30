/// main functions and executions of the project 
/// loads the data and builds the word co-occurance graph, calculates metrics, finds clusters, and analyzes word patterns in review scores 


mod data;
mod graph;
mod tests;


use data::{load_reviews, clean_text};
use graph::Graph;
use std::collections::HashMap;

fn main() {
    let reviews = load_reviews("data/booking_reviews copy 2.csv");

    let mut word_graph = Graph::new();

    for review in reviews.iter() {
        let words = clean_text(&review.review_text);
        word_graph.add_review(&words);
    }

    println!("Graph has {} words (nodes)", word_graph.edges.len());

    word_graph.print_top_connected_words();

    let degree = word_graph.degree_centrality();
    let weighted = word_graph.weighted_degree_centrality();
    let pagerank = word_graph.pagerank_like_score();

    print_top_10(&degree, "Top 10 by Degree Centrality");
    print_top_10(&weighted, "Top 10 by Weighted Degree Centrality");
    print_top_10_float(&pagerank, "Top 10 by PageRank Score");

    let clusters = word_graph.find_clusters();
    println!("Found {} clusters.", clusters.len());

    analyze_review_scores(&reviews);
}
/// prints the top ten key-value pairs from a map of words and counts
/// helps with centrality and frequency outputs 
fn print_top_10(map: &HashMap<String, usize>, title: &str) {
    let mut items: Vec<_> = map.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1));

    println!("\n{}", title);
    for (word, score) in items.iter().take(10) {
        println!("{}: {}", word, score);
    }
}
/// same as function above but used for float scores also filters out words with very low scores 
fn print_top_10_float(map: &HashMap<String, f64>, title: &str) {
    let mut items: Vec<_> = map.iter()
        .filter(|(_, score)| **score > 0.1)  
        .collect();
    
    items.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    println!("\n{}", title);
    for (word, score) in items.iter().take(10) {
        println!("{}: {:.2}", word, score);
    }
}

/// groups reviews by score high reviews -> 9-10 low reviews -> 0-5
/// collects word frequencies from each group 
/// helps compare language in positive vs negative reviews
fn analyze_review_scores(reviews: &[data::Review]) {
    let mut high_score_words = Vec::new();
    let mut low_score_words = Vec::new();

    for review in reviews {
        let words = data::clean_text(&review.review_text);
        if review.rating >= 9.0 {
            high_score_words.extend(words);
        } else if review.rating <= 5.0 {
            low_score_words.extend(words);
        }
    }

    let high_counts = count_words(&high_score_words);
    let low_counts = count_words(&low_score_words);

    println!("\nTop words in HIGH-rated reviews (9-10):");
    print_top_10(&high_counts, "High Rated");

    println!("\nTop words in LOW-rated reviews (0-5):");
    print_top_10(&low_counts, "Low Rated");
}
/// counts how often each word appears in a list 
fn count_words(words: &[String]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in words {
        *counts.entry(word.clone()).or_insert(0) += 1;
    }
    counts
}
