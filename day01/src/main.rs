use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::Result;

fn main() -> Result<()> {
    let path = "puzzle_input.tsv";
    let (left_locations, right_locations) = read_location_ids(path)?;

    // Print the lists to verify
    println!("Left List: {:?}", left_locations);
    println!("Right List: {:?}", right_locations);

    Ok(())
}

// Read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
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