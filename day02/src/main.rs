use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;

fn main() {
    let reports: Vec<Report> = read_reports();
    let n_safe_reports: usize = count_safe_reports(reports);
    println!("Number of safe reports: {}", n_safe_reports);
}

#[derive(Debug)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    // Check if the levels are monotonic
    fn is_strictly_monotonic(&self) -> bool {
        let mut increasing: bool = true;
        let mut decreasing: bool = true;

        for window in self.levels.windows(2) {
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

    // Check if the difference between each level is within the bounds
    fn difference_within_bounds(&self, lower: u32, upper: u32) -> bool {
        let mut differences: Vec<u32> = Vec::new();

        for window in self.levels.windows(2) {
            differences.push((window[1] - window[0]).unsigned_abs());
        }

        differences.iter().all(|&x| x >= lower && x <= upper)
    }

    // A report is safe if it is monotonic and the difference between each level is within 1 and 3
    fn is_safe(&self) -> bool {
        self.is_strictly_monotonic() && self.difference_within_bounds(1, 3)
    }
}

// Get the number of safe reports
fn count_safe_reports(reports: Vec<Report>) -> usize {
    reports.into_iter().filter(|r| r.is_safe()).count()
}

// Read reports from the reports.txt file
fn read_reports() -> Vec<Report> {
    let mut reports: Vec<Report> = Vec::new();
    let file: File = File::open("reports.txt").expect("Could not open file");
    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        let line: String = line.expect("Could not read line");
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|s: &str| s.parse().expect("Could not parse number"))
            .collect();
        reports.push(Report { levels });
    }

    reports
}
