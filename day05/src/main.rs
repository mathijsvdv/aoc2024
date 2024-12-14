use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let rules = read_page_ordering_rules("page_ordering_rules.txt");
    println!("Page ordering rules: {:?}", rules);

    let updates = read_page_updates("page_updates.txt");
    println!("Page updates: {:?}", updates);

    let rule_map: HashMap<(i8, i8), &PageOrderingRule> = page_ordering_rules_to_map(&rules);

    let correctly_ordered_updates: Vec<_> = updates
        .iter()
        .filter(|update| update.is_correctly_ordered(&rule_map))
        .collect();
    println!("Correctly ordered updates: {:?}", correctly_ordered_updates);

    let answer = correctly_ordered_updates
        .iter()
        .map(|update| update.get_middle_page() as i32)
        .sum::<i32>();
    println!("Answer: {}", answer);
}

#[derive(Debug)]
struct PageOrderingRule {
    before: i8,
    after: i8,
}

// Struct to represent a page update and the positions of the pages
#[derive(Debug)]
struct PageUpdate {
    pages: Vec<i8>,
    positions: HashMap<i8, usize>,
}

impl PageUpdate {
    fn new(pages: Vec<i8>) -> Self {
        let positions = pages_to_positions(&pages);
        PageUpdate { pages, positions }
    }

    fn is_correctly_ordered(&self, rule_map: &HashMap<(i8, i8), &PageOrderingRule>) -> bool {
        let pairs = find_pairs(&self.pages);
        let applicable_rules = find_applicable_rules(&pairs, rule_map);
        for rule in applicable_rules {
            if !rule.satisfied(&self.positions) {
                return false;
            }
        }
        true
    }

    fn get_middle_page(&self) -> i8 {
        get_middle_element(&self.pages)
    }
}


// Read the page ordering rules from a file
fn read_page_ordering_rules(path: &str) -> Vec<PageOrderingRule> {
    let file = File::open(path).expect("File not found");
    let reader = BufReader::new(file);
    let mut rules = Vec::new();
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue;
            }
        };
        let parts: Vec<&str> = line.split('|').collect();
        let before: i8 = parts[0].parse().unwrap();
        let after: i8 = parts[1].parse().unwrap();
        rules.push(PageOrderingRule { before, after });
    }
    rules
}

// Read the page updates from a file
fn read_page_updates(path: &str) -> Vec<PageUpdate> {
    let file = File::open(path).expect("File not found");
    let reader = BufReader::new(file);
    let mut updates = Vec::new();
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue;
            }
        };
        let update: Vec<&str> = line.split(',').collect();
        let update: Vec<i8> = update.iter().map(|s| s.parse().unwrap()).collect();
        let update: PageUpdate = PageUpdate::new(update);
        updates.push(update);
    }

    updates
}

// Find all pairs in a vector
fn find_pairs<T: Clone>(vec: &[T]) -> Vec<(T, T)> {
    let mut pairs = Vec::new();
    for i in 0..vec.len() {
        for j in i + 1..vec.len() {
            pairs.push((vec[i].clone(), vec[j].clone()));
        }
    }
    pairs
}

// Trait which allows an object to translate itself to an unordered key.
// The key is the same, regardless of the order of the pair
trait ToUnorderedKey {
    type Key;
    fn to_unordered_key(&self) -> Self::Key;
}

// Implement the trait for (i8, i8)
impl ToUnorderedKey for (i8, i8) {
    type Key = (i8, i8);
    fn to_unordered_key(&self) -> Self::Key {
        if self.0 < self.1 {
            *self
        } else {
            (self.1, self.0)
        }
    }
}

// Implement the trait for PageOrderingRule
impl ToUnorderedKey for PageOrderingRule {
    type Key = (i8, i8);
    fn to_unordered_key(&self) -> Self::Key {
        if self.before < self.after {
            (self.before, self.after)
        } else {
            (self.after, self.before)
        }
    }
}

fn page_ordering_rules_to_map(
    rules: &[PageOrderingRule],
) -> HashMap<(i8, i8), &PageOrderingRule> {
    let mut map: HashMap<(i8, i8), &PageOrderingRule> = std::collections::HashMap::new();
    for rule in rules {
        map.insert(rule.to_unordered_key(), rule);
    }
    map
}

fn find_applicable_rules<'a>(
    pairs: &[(i8, i8)],
    rule_map: &HashMap<(i8, i8), &'a PageOrderingRule>,
) -> Vec<&'a PageOrderingRule> {
    let mut applicable_rules = Vec::new();
    for pair in pairs {
        if let Some(rule) = rule_map.get(&pair.to_unordered_key()) {
            applicable_rules.push(*rule);
        }
    }
    applicable_rules
}

fn pages_to_positions(pages: &[i8]) -> HashMap<i8, usize> {
    let mut map: HashMap<i8, usize> = std::collections::HashMap::new();
    for (i, page) in pages.iter().enumerate() {
        map.insert(*page, i);
    }
    map
}

impl PageOrderingRule {
    fn satisfied(&self, positions: &HashMap<i8, usize>) -> bool {
        let before_index = positions.get(&self.before).unwrap();
        let after_index = positions.get(&self.after).unwrap();
        before_index < after_index
    }

    // fn apply(&self, update: &mut Vec<i8>) {
    //     let before_index = update.iter().position(|&x| x == self.before).unwrap();
    //     let after_index = update.iter().position(|&x| x == self.after).unwrap();
    //     update.swap(before_index, after_index);
    // }
}

fn get_middle_element(vec: &[i8]) -> i8 {
    vec[vec.len() / 2]
}
