use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let rules = read_page_ordering_rules("page_ordering_rules.txt");
    println!("Page ordering rules: {:?}", rules);

    let updates = read_page_updates("page_updates.txt");
    println!("Page updates: {:?}", updates);

    let update: Vec<i8> = updates[updates.len() - 1].clone();
    let pairs = find_pairs(&update);
    println!("Pairs: {:?}", pairs);
}

// Find all pairs in a vector
fn find_pairs<T: Clone>(vec: &Vec<T>) -> Vec<(T, T)> {
    let mut pairs = Vec::new();
    for i in 0..vec.len() {
        for j in i + 1..vec.len() {
            pairs.push((vec[i].clone(), vec[j].clone()));
        }
    }
    pairs
}

// Translate an unordered pair to a key. The key is the same, regardless of the order of the pair
fn _pair_to_key(pair: (i8, i8)) -> (i8, i8) {
    if pair.0 < pair.1 {
        return pair;
    } else {
        return (pair.1, pair.0);
    }
}

#[derive(Debug)]
struct PageOrderingRule {
    before: i8,
    after: i8,
}

// Read the page ordering rules from a file
fn read_page_ordering_rules(path: &str) -> Vec<PageOrderingRule> {
    let file = File::open(path).expect("File not found");
    let reader = BufReader::new(file);
    let mut rules = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('|').collect();
        let before: i8 = parts[0].parse().unwrap();
        let after: i8 = parts[1].parse().unwrap();
        rules.push(PageOrderingRule { before, after });
    }
    rules
}

// Read the page updates from a file
fn read_page_updates(path: &str) -> Vec<Vec<i8>> {
    let file = File::open(path).expect("File not found");
    let reader = BufReader::new(file);
    let mut updates = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let update: Vec<&str> = line.split(',').collect();
        let update: Vec<i8> = update.iter().map(|s| s.parse().unwrap()).collect();
        updates.push(update);
    }

    updates
}
