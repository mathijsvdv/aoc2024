use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let word_search = load_word_search();
    let n_matches_per_direction = get_n_matches_per_direction(&word_search, "XMAS");

    // Print the number of matches per direction
    for (direction, n_matches) in n_matches_per_direction.iter() {
        println!("{}: {}", direction, n_matches);
    }

    // Print the total number of matches
    let n_matches = get_n_matches(&word_search, "XMAS");
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
        n_matches_per_direction.insert(direction.to_string(), _get_n_matches_in_direction(direction, word));
    }

    n_matches_per_direction
}


#[derive(Debug, Clone)]
struct Match {
    n_matches_per_char: Vec<usize>,
}

impl Match {
    fn get_n_matches(&self) -> usize {
        self.n_matches_per_char.iter().product()
    }
}


fn _update_matches(
    matches: &mut HashMap<char, Vec<Match>>,
    char_to_index: &HashMap<char, usize>,
    word: &[char],
    char: char,
) {
    let &i = char_to_index.get(&char).expect("Character is not in word");

    if let Some(match_vec) = matches.get_mut(&char) {
        for m in match_vec.iter_mut() {
            m.n_matches_per_char[i] += 1;
        }
    }

    if i == 0 {
        matches.entry(char).or_insert_with(Vec::new).push(Match { n_matches_per_char: vec![1] });
        return;
    }

    let prev_char = word[i - 1];
    if let Some(prev_matches) = matches.get(&prev_char) {
        let prev_matches: Vec<Match> = prev_matches.iter().cloned().collect();
        for m in prev_matches {
            let mut new_match = m.clone();
            new_match.n_matches_per_char.push(1);
            matches.entry(char).or_insert_with(Vec::new).push(new_match);
        }
    }
}


fn _get_n_matches_in_direction(direction: &str, word: &str) -> usize {
    let word: Vec<char> = word.chars().collect();

    let mut matches: HashMap<char, Vec<Match>> = HashMap::new();

    let mut char_to_index: HashMap<char, usize> = HashMap::new();
    for (i, char) in word.iter().enumerate() {
        char_to_index.insert(*char, i);
    }

    let mut n_matches: usize = 0;
    for char in direction.chars() {
        _update_matches(&mut matches, &char_to_index, &word, char);
    }

    // The matches mapped to the last character represent full words that we've found
    // So we sum the number of matches for each of these
    if let Some(last_matches) = matches.get(&word[word.len() - 1]) {
        n_matches += last_matches.iter().map(|m| m.get_n_matches()).sum::<usize>();
    }

    let mut matches: HashMap<char, Vec<Match>> = HashMap::new();

    for char in direction.chars().rev() {
        _update_matches(&mut matches, &char_to_index, &word, char);
    }

    // The matches mapped to the last character represent full words that we've found
    // So we sum the number of matches for each of these
    if let Some(last_matches) = matches.get(&word[word.len() - 1]) {
        n_matches += last_matches.iter().map(|m| m.get_n_matches()).sum::<usize>();
    }
    n_matches
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
