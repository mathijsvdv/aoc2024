use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let rules = read_page_ordering_rules("page_ordering_rules.txt");
    println!("Page ordering rules: {:?}", rules);

    let mut updates = read_page_updates("page_updates.txt");
    println!("Page updates: {:?}", updates);

    let rule_map: HashMap<(i8, i8), &PageOrderingRule> = page_ordering_rules_to_map(&rules);

    let correctly_ordered_updates: Vec<_> = updates
        .iter()
        .filter(|update| update.is_correctly_ordered(&rule_map))
        .collect();
    println!("Correctly ordered updates: {:?}", correctly_ordered_updates);

    let answer_1 = correctly_ordered_updates
        .iter()
        .map(|update| update.get_middle_page() as i32)
        .sum::<i32>();
    println!("Answer 1: {}", answer_1);

    let mut incorrectly_ordered_updates: Vec<_> = updates
        .iter_mut()
        .filter(|update| !update.is_correctly_ordered(&rule_map))
        .collect();

    for update in &mut incorrectly_ordered_updates {
        update.order(&rule_map);
    }

    let answer_2 = incorrectly_ordered_updates
        .iter()
        .map(|update| update.get_middle_page() as i32)
        .sum::<i32>();
    println!("Answer 2: {}", answer_2);

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
            if !rule.satisfied(&self) {
                return false;
            }
        }
        true
    }

    fn order(&mut self, rule_map: &HashMap<(i8, i8), &PageOrderingRule>) {
        let pairs = find_pairs(&self.pages);
        let applicable_rules = find_applicable_rules(&pairs, rule_map);
        for rule in applicable_rules {
            rule.apply(self);
        }
    }

    fn get_middle_page(&self) -> i8 {
        get_middle_element(&self.pages)
    }
}


fn get_middle_element(vec: &[i8]) -> i8 {
    vec[vec.len() / 2]
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
    fn satisfied(&self, update: &PageUpdate) -> bool {
        let before_index = update.positions.get(&self.before).unwrap();
        let after_index = update.positions.get(&self.after).unwrap();
        before_index < after_index
    }

    fn apply(&self, update: &mut PageUpdate) {
        if self.satisfied(update) {
            return;
        }
        let before_index = *update.positions.get(&self.before).unwrap();
        let after_index = *update.positions.get(&self.after).unwrap();
        update.pages.swap(before_index, after_index);
        update.positions.insert(self.before, after_index);
        update.positions.insert(self.after, before_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        vec![
            PageOrderingRule { before: 47, after: 53 },
            PageOrderingRule { before: 97, after: 13 },
            PageOrderingRule { before: 97, after: 61 },
            PageOrderingRule { before: 97, after: 47 },
            PageOrderingRule { before: 75, after: 29 },
            PageOrderingRule { before: 61, after: 13 },
            PageOrderingRule { before: 75, after: 53 },
            PageOrderingRule { before: 29, after: 13 },
            PageOrderingRule { before: 97, after: 29 },
            PageOrderingRule { before: 53, after: 29 },
            PageOrderingRule { before: 61, after: 53 },
            PageOrderingRule { before: 97, after: 53 },
            PageOrderingRule { before: 61, after: 29 },
            PageOrderingRule { before: 47, after: 13 },
            PageOrderingRule { before: 75, after: 47 },
            PageOrderingRule { before: 97, after: 75 },
            PageOrderingRule { before: 47, after: 61 },
            PageOrderingRule { before: 75, after: 61 },
            PageOrderingRule { before: 47, after: 29 },
            PageOrderingRule { before: 75, after: 13 },
            PageOrderingRule { before: 53, after: 13 },
        ],
        vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ]
    )]
    fn test_page_update_order_is_correctly_ordered(rules: Vec<PageOrderingRule>, updates: Vec<Vec<i8>>) {
        let rule_map: HashMap<(i8, i8), &PageOrderingRule> = page_ordering_rules_to_map(&rules);

        let mut updates: Vec<PageUpdate> = updates.into_iter().map(PageUpdate::new).collect();

        for update in &mut updates {
            update.order(&rule_map);
            assert!(update.is_correctly_ordered(&rule_map));
        }
    }


}
