use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;

fn main() {
    let reports: Vec<Report> = read_reports();
    let n_safe_reports: usize = count_safe_reports(reports);
    println!("Number of safe reports: {}", n_safe_reports);
}


#[derive(Clone)]
#[derive(Debug)]
struct ProblemDampener {}

#[derive(Debug)]
struct Report {
    levels: Vec<i32>,
    problem_dampener: Option<ProblemDampener>,
}


// Levels are strictly monotonic if they are either strictly increasing or strictly decreasing
fn are_strictly_monotonic(levels: &Vec<i32>) -> bool {
    let mut increasing: bool = true;
    let mut decreasing: bool = true;

    for window in levels.windows(2) {
        match window[0].cmp(&window[1]) {
            Ordering::Less => decreasing = false,
            Ordering::Greater => increasing = false,
            Ordering::Equal => {decreasing = false; increasing = false;}
        }
        if !increasing && !decreasing {
            break;
        }
    }

    increasing || decreasing
}


// The difference between each level must be within the bounds of lower and upper
fn difference_within_bounds(levels: &Vec<i32>, lower: i32, upper: i32) -> bool {
    let mut differences: Vec<i32> = Vec::new();

    for window in levels.windows(2) {
        differences.push((window[1] - window[0]).abs());
    }

    differences.iter().all(|&x| x >= lower && x <= upper)
}


// Levels are considered safe if they are strictly monotonic and the difference between each level is within 1 and 3
fn safe_levels(levels: &Vec<i32>) -> bool {
    are_strictly_monotonic(levels) && difference_within_bounds(levels, 1, 3)
}


impl ProblemDampener {
    // Dampen any problems in the levels by removing one level at a time and checking if the levels are safe
    fn dampen(&self, levels: &mut Vec<i32>) -> bool {
        let mut safe: bool = true;
        for i in 0..levels.len() {
            let level = levels[i];
            levels.remove(i);
            safe = safe_levels(&levels);
            if safe {
                break;
            }
            levels.insert(i, level);
        }
        safe
    }
}


impl Report {
    // A report is safe if it is monotonic and the difference between each level is within 1 and 3
    fn is_safe(&self) -> bool {
        let mut safe: bool = safe_levels(&self.levels);
        let mut levels: Vec<i32> = self.levels.clone();

        if !safe {
            match &self.problem_dampener {
                Some(dampener) => {safe = dampener.dampen(&mut levels)},
                None => {}
            }
        }

        safe
    }
}


// Get the number of safe reports
fn count_safe_reports(reports: Vec<Report>) -> usize {
    reports.into_iter().filter(|r: &Report| r.is_safe()).count()
}

// Read reports from the reports.txt file
fn read_reports() -> Vec<Report> {
    let mut reports: Vec<Report> = Vec::new();
    let file: File = File::open("reports.txt").expect("Could not open file");
    let reader: BufReader<File> = BufReader::new(file);
    let problem_dampener: Option<ProblemDampener> = Some(ProblemDampener {});
    for line in reader.lines() {
        let line: String = line.expect("Could not read line");
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|s: &str| s.parse().expect("Could not parse number"))
            .collect();
        reports.push(Report { levels, problem_dampener: problem_dampener.clone() });
    }

    reports
}
