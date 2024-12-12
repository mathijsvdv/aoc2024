use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let word_search = load_word_search();
    let n_matches_per_direction = get_n_matches_per_direction(&word_search, "XMAS");

    // Print the number of matches per direction
    for (direction, n_matches) in n_matches_per_direction.iter() {
        println!("{}: {}", direction, n_matches);
    }
}

fn load_word_search() -> Vec<Vec<char>> {
    let file = File::open("word_search.txt").expect("File not found");
    let reader = BufReader::new(file);

    let mut word_search = Vec::new();
    let mut max_cols = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        max_cols = max_cols.max(chars.len());
        word_search.push(chars);
    }

    word_search
}


fn get_n_matches_per_direction(word_search: &Vec<Vec<char>>, word: &str) -> HashMap<String, usize> {
    let directions = get_directions(word_search);
    let mut n_matches_per_direction: HashMap<String, usize> = HashMap::new();

    for direction in directions.iter() {
        n_matches_per_direction.insert(direction.to_string(), _get_n_matches_in_direction(direction, word));
    }

    n_matches_per_direction
}


fn _get_n_matches_in_direction(direction: &str, word: &str) -> usize {
    let mut n_matches = 0;
    let chars: Vec<String> = word.chars().map(|c| c.to_string()).collect();
    let regex_string = chars.join(".*");
    let regex = Regex::new(&regex_string).unwrap();

    for _ in regex.captures_iter(&direction) {
        n_matches += 1;
    }

    for _ in regex.captures_iter(&reverse_direction(&direction)) {
        n_matches += 1;
    }

    n_matches
}


fn reverse_direction(directions: &str) -> String {
    let mut reversed = String::new();

    for c in directions.chars().rev() {
        reversed.push(c);
    }

    reversed
}


fn get_directions(word_search: &Vec<Vec<char>>) -> Vec<String> {
    let rows = get_rows(word_search);
    let diagonals = get_diagonals(word_search);

    let transposed = transpose(word_search);
    let cols = get_rows(&transposed);
    let tdiagonals = get_diagonals(&transposed);

    let direction_vecs: [Vec<String>; 4] = [rows, cols, diagonals, tdiagonals];
    let n_directions = direction_vecs.iter().map(|vec| vec.len()).sum();
    let mut directions = Vec::with_capacity(n_directions);

    for vec in direction_vecs.iter() {
        for direction in vec {
            directions.push(direction.to_string());
        }
    }

    directions
}

fn transpose(word_search: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut transposed = Vec::new();

    for col in 0..word_search[0].len() {
        let mut transposed_row = Vec::new();
        for row in word_search {
            if col < row.len() {
                transposed_row.push(row[col]);
            }
        }
        transposed.push(transposed_row);
    }

    transposed
}

fn get_rows(word_search: &Vec<Vec<char>>) -> Vec<String> {
    let mut rows = Vec::new();

    for row in word_search {
        let row_str: String = row.iter().collect();
        rows.push(row_str);
    }

    rows
}

fn get_diagonals(word_search: &Vec<Vec<char>>) -> Vec<String> {
    let mut diagonals = Vec::new();

    for row in 0..word_search.len() {
        let diagonal = _get_diagonal(word_search, row, 0);
        diagonals.push(diagonal);
    }

    for col in 1..word_search[0].len() {
        let diagonal = _get_diagonal(word_search, 0, col);
        diagonals.push(diagonal);
    }

    diagonals
}

fn _get_diagonal(word_search: &Vec<Vec<char>>, row: usize, col: usize) -> String {
    let mut diagonal = String::new();

    let mut r = row;
    let mut c = col;

    while r < word_search.len() && c < word_search[0].len() {
        diagonal.push(word_search[r][c]);
        r += 1;
        c += 1;
    }

    diagonal
}
