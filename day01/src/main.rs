use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::Result;
use counter::Counter;

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
    let distances: Vec<i32> = left_locations.iter()
        .zip(right_locations.iter())
        .map(|(left, right)| (left - right).abs())
        .collect();
    distances.iter().sum()
}

// Compute the similarity score between two sets of location ids
fn compute_similarity_score(left_locations: &Vec<i32>, right_locations: &Vec<i32>) -> i32 {
    let left_counter: Counter<_> = left_locations.iter().collect();
    let right_counter: Counter<_> = right_locations.iter().collect();
    _compute_similarity_score_from_counters(left_counter, right_counter)
}

// Compute the similarity score between two sets of location ids from their counters
fn _compute_similarity_score_from_counters(left_counter: Counter<&i32>, right_counter: Counter<&i32>) -> i32 {
    let mut score: i32 = 0;
    for (id, count) in left_counter.iter() {
        if let Some(right_count) = right_counter.get(id) {
            score += *id * (*count as i32) * (*right_count as i32);
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