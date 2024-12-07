use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::Result;

fn main() -> Result<()> {
    let path: &str = "puzzle_input.tsv";
    let (mut left_locations, mut right_locations) = read_location_ids(path)?;

    // Sort the lists
    left_locations.sort();
    right_locations.sort();

    let total_distance: i32 = compute_total_distance(&left_locations, &right_locations);
    let similarity_score: i32 = compute_similarity_score(&left_locations, &right_locations);

    // Print the total distance
    println!("Total distance: {}", total_distance);

    // Print the similarity score
    println!("Similarity score: {}", similarity_score);

    Ok(())
}

// Compute the total distance between two sets of locations
fn compute_total_distance(left_locations: &Vec<i32>, right_locations: &Vec<i32>) -> i32 {
    let distances: Vec<i32> = left_locations.into_iter()
        .zip(right_locations.into_iter())
        .map(|(left, right)| (left - right).abs())
        .collect();
    distances.iter().sum()
}

// Compute the similarity score between two sets of location ids
fn compute_similarity_score(left_locations: &Vec<i32>, right_locations: &Vec<i32>) -> i32 {
    let left_counter: HashMap<i32, i32> = _get_counter(left_locations);
    let right_counter: HashMap<i32, i32> = _get_counter(right_locations);
    _compute_similarity_score_from_counters(left_counter, right_counter)
}


// Count number of occurrences of each location id
fn _get_counter(location_ids: &Vec<i32>) -> HashMap<i32, i32> {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for id in location_ids {
        let count = counter.entry(*id).or_insert(0);
        *count += 1;
    }
    counter
}

// Compute the similarity score between two sets of location ids from their counters
fn _compute_similarity_score_from_counters(left_counter: HashMap<i32, i32>, right_counter: HashMap<i32, i32>) -> i32 {
    let mut score: i32 = 0;
    for (id, count) in left_counter.iter() {
        if let Some(right_count) = right_counter.get(id) {
            score += count * right_count;
        }
    }
    score
}

// Split a line into a tuple of location ids
fn split_location_ids(line: String) -> (i32, i32) {
    let split: Vec<&str> = line.split_whitespace().collect();
    let left_id: i32 = split[0].parse().unwrap();
    let right_id: i32 = split[1].parse().unwrap();
    (left_id, right_id)
}

// Read left and right location ids from a file
fn read_location_ids<P>(filename: P) -> Result<(Vec<i32>, Vec<i32>)>
where
    P: AsRef<Path>,
{
    let file: File = File::open(filename)?;
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();
    let mut left_locations: Vec<i32> = Vec::new();
    let mut right_locations: Vec<i32> = Vec::new();

    for line in lines {
        if let Ok(ip) = line {
            let (left_id, right_id) = split_location_ids(ip);
            left_locations.push(left_id);
            right_locations.push(right_id);
        }
    }

    Ok((left_locations, right_locations))
}
