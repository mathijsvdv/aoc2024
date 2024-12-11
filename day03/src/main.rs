use regex::Regex;

fn main() {
    let memory = read_memory();
    let multiplications = parse_multiplications(&memory);
    let mut result = 0;
    for multiplication in multiplications {
        result += mul(multiplication);
    }
    println!("Result: {}", result);
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Multiplication {
    x: i32,
    y: i32,
}

fn mul(multiplication: Multiplication) -> i32 {
    multiplication.x * multiplication.y
}


fn parse_multiplications(s: &str) -> Vec<Multiplication> {
    let mut vec = Vec::new();
    let mut enable_mul = true;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    for cap in re.captures_iter(s) {
        if cap.get(0).unwrap().as_str() == "do()" {
            enable_mul = true;
            continue;
        }
        if cap.get(0).unwrap().as_str() == "don't()" {
            enable_mul = false;
            continue;
        }
        if !enable_mul {
            continue;
        }
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        vec.push(Multiplication { x, y });
    }

    vec
}


// Read memory from the memory.txt file
fn read_memory() -> String{
    let memory: String = std::fs::read_to_string("memory.txt").unwrap();
    memory
}