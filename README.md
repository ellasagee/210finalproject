# Hotel Review Word Graph Analysis

My project analyzes hotel reviews from Booking.com to find common themes, relationships between words, and patterns tied to review scores. Using a word co-occurrence graph, my project identifies frequently associated terms, clusters of related language, and central keywords in positive and negative reviews.

## Project Structure

- `data.rs` — Loads and cleans the dataset using `csv`, `serde`, and `regex`
- `graph.rs` — Builds and analyzes the word co-occurrence graph
- `main.rs` — Coordinates the program: loads reviews, builds the graph, and prints insights
- `tests.rs` — Unit tests for key functions (text cleaning, graph logic, etc.)
- `data/` — Contains the CSV dataset

##  What the Project Does

- Cleans and tokenizes thousands of hotel reviews.
- Builds a graph where:
  - **Nodes** = unique words.
  - **Edges** = co-occurrence in the same review.
- Computes:
  - Degree centrality
  - Weighted centrality
  - PageRank-like scores
- Clusters related words via connected components.
- Analyzes language in high (9–10) vs low (0–5) rated reviews.
