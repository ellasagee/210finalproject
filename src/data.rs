/// this part is used for loading and cleaning the data set 
/// it does this by going through the data applying different cleaning and reading functions to it
/// the functions -> lowercase, remove punctuation, tokenize, and filter stopwords 


/// loading and cleaning the dataset 
use csv::Reader;
use serde::Deserialize;
use regex::Regex;
use stopwords::{Stopwords, Language, Spark, NLTK};

/// making struct called review for a single review 
#[derive(Debug, Deserialize)]
pub struct Review {
    #[serde(rename = "review_text")]
    pub review_text: String,
    
    #[serde(rename = "rating")]
    pub rating: f64,
}
/// loads reviews from the CSV file into a list of review structs 
/// inputs -> 'path': where the CSV file is -> data/booking_reviews.csv
/// outputs -> a list 'Vec' of review structs with the text review and score (number) of each review
/// will also skip any rows that are messed up

pub fn load_reviews(path: &str) -> Vec<Review> {
    let mut rdr = Reader::from_path(path).expect("Failed to open CSV file.");
    let mut reviews = Vec::new();

    for result in rdr.deserialize() {
        match result {
            Ok(record) => reviews.push(record),
            Err(e) => eprintln!("Skipping invalid record: {}", e),
        }
    }
    
    reviews
}

/// cleaning the data -> lowercasing all letters, removing punctuation, and splitting each text review into words 
/// inputs -> 'text' -> a string slice that contains the raw review text 
/// outputs -> a Vec<String> with cleaned individual words 

pub fn clean_text(text: &str) -> Vec<String> {
    // part 1 -> Lowercases the text
    let lowercase = text.to_lowercase();

    // part 2 -> Remove punctuation (keep only letters, numbers, spaces)
    let re = Regex::new(r"[^\w\s]").unwrap(); // Regex explanation: \w = word characters, \s = spaces
    let cleaned = re.replace_all(&lowercase, "");

    // part 3 - > Tokenize the cleaned text into words by splitting on whitespace
    let words: Vec<String> = cleaned
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    // part 4 -> add in stopwords
    let stopwords = Spark::stopwords(Language::English).unwrap();

    // part 5 -> filter out stopwords while returning
    let cleaned_words: Vec<String> = words
        .into_iter()
        .filter(|word| !stopwords.contains(&word.as_str()))
        .collect();

    cleaned_words
}

