use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let word_search = load_word_search();
    let word = "XMAS";

    // Print the number of matches in a specific direction
    let direction = "MXMXAXMASX";
    let n_matches = _get_n_matches_in_direction(direction, word);
    println!("Matches in direction '{}': {}", direction, n_matches);

    let n_matches_per_direction = get_n_matches_per_direction(&word_search, word);
    // Print the number of matches per direction
    for (direction, n_matches) in n_matches_per_direction.iter() {
        println!("{}: {}", direction, n_matches);
    }

    // Print the total number of matches
    let n_matches = get_n_matches(&word_search, word);
    println!("Total matches: {}", n_matches);
}

fn load_word_search() -> Vec<Vec<char>> {
    let file = File::open("word_search_small.txt").expect("File not found");
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

fn get_n_matches(word_search: &Vec<Vec<char>>, word: &str) -> usize {
    let directions = get_directions(word_search);
    let mut n_matches: usize = 0;

    for direction in directions.iter() {
        n_matches += _get_n_matches_in_direction(direction, word);
    }

    n_matches
}

fn get_n_matches_per_direction(word_search: &Vec<Vec<char>>, word: &str) -> HashMap<String, usize> {
    let directions = get_directions(word_search);
    let mut n_matches_per_direction: HashMap<String, usize> = HashMap::new();

    for direction in directions.iter() {
        n_matches_per_direction.insert(
            direction.to_string(),
            _get_n_matches_in_direction(direction, word),
        );
    }

    n_matches_per_direction
}

fn _update_matches(
    n_matches_per_subword: &mut HashMap<String, usize>,
    char_to_index: &HashMap<char, usize>,
    char_to_subword: &HashMap<char, String>,
    word: &str,
    char: char,
) {
    // println!("Current character: {}", char);

    let &i = char_to_index.get(&char).expect("Character is not in word");
    let subword = char_to_subword
        .get(&char)
        .expect("Character is not in word");

    if i == 0 {
        *n_matches_per_subword
            .entry(subword.to_string())
            .or_insert(0) += 1;
        // println!("Matches per subword after character: {:?}", n_matches_per_subword);
        return;
    }

    let prev_char = word
        .chars()
        .nth(i - 1)
        .expect("Character index out of bounds");
    let n_matches = n_matches_per_subword
        .get(&char_to_subword[&prev_char])
        .cloned()
        .unwrap_or(0);
    *n_matches_per_subword
        .entry(subword.to_string())
        .or_insert(0) += n_matches;

    // println!("Matches per subword after character: {:?}", n_matches_per_subword);

    return;
}

fn _get_n_matches_in_direction(direction: &str, word: &str) -> usize {
    // TODO Still too many matches, check for example this string: "MASAMXXAM"
    // Should match twice, but matches 3 times

    let mut char_to_index: HashMap<char, usize> = HashMap::new();
    for (i, char) in word.chars().enumerate() {
        char_to_index.insert(char, i);
    }

    let mut char_to_subword: HashMap<char, String> = HashMap::new();
    let mut subword = String::new();
    for char in word.chars() {
        subword.push(char);
        char_to_subword.insert(char, subword.clone());
    }

    let mut n_matches: usize = 0;

    let mut n_matches_per_subword: HashMap<String, usize> = HashMap::new();
    for char in direction.chars() {
        _update_matches(
            &mut n_matches_per_subword,
            &char_to_index,
            &char_to_subword,
            &word,
            char,
        );
    }

    // The matches mapped to the last character represent full words that we've found
    // So we sum the number of matches for each of these
    if let Some(n_matches_of_full_word) = n_matches_per_subword.get(word) {
        n_matches += n_matches_of_full_word;
    }

    let mut n_matches_per_subword: HashMap<String, usize> = HashMap::new();
    for char in direction.chars().rev() {
        _update_matches(
            &mut n_matches_per_subword,
            &char_to_index,
            &char_to_subword,
            &word,
            char,
        );
    }

    // The matches mapped to the last character represent full words that we've found
    // So we sum the number of matches for each of these
    if let Some(n_matches_of_full_word) = n_matches_per_subword.get(word) {
        n_matches += n_matches_of_full_word;
    }

    n_matches
}

fn get_directions(word_search: &Vec<Vec<char>>) -> Vec<String> {
    let rows = get_rows(word_search);
    let transposed = transpose(word_search);
    let cols = get_rows(&transposed);

    let diagonals = get_diagonals(word_search);
    let rotated = rotate(word_search);
    // let antidiagonals = get_diagonals(&rotated);

    let direction_vecs: [Vec<String>; 3] = [rows, cols, diagonals];
    let n_directions = direction_vecs.iter().map(|vec| vec.len()).sum();
    let mut directions = Vec::with_capacity(n_directions);

    for vec in direction_vecs.iter() {
        for direction in vec {
            directions.push(direction.to_string());
        }
    }

    directions
}


// Rotates the word search by 90 degrees
fn rotate(word_search: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rotated = Vec::new();

    for col in 0..word_search[0].len() {
        let mut rotated_row = Vec::new();
        for row in word_search.iter().rev() {
            rotated_row.push(row[col]);
        }
        rotated.push(rotated_row);
    }

    rotated
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
