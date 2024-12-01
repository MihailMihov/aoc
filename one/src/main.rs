use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let filename = "res/input";

    match part_one(filename) {
        Ok(result) => println!("Part one: {}", result),
        Err(e) => eprintln!("Error in part one: {}", e),
    }

    match part_two(filename) {
        Ok(result) => println!("Part two: {}", result),
        Err(e) => eprintln!("Error in part two: {}", e),
    }
}

fn part_one<P>(filename: P) -> Result<i64, io::Error> 
where P: AsRef<Path>, {
    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let mut split = line.split_whitespace();

            if let (Some(first), Some(second)) = (split.next(), split.next()) {
                match (first.parse::<i64>(), second.parse::<i64>()) {
                    (Ok(a), Ok(b)) => {
                        vec_a.push(a);
                        vec_b.push(b);
                    }
                    _ => panic!("Invalid input line: {}", line)
                }
            }
        }
    }

    vec_a.sort();
    vec_b.sort();

    Ok(vec_a.into_iter().zip(vec_b)
        .map(|(a, b)| (a - b).abs())
        .sum())
}

fn part_two<P>(filename: P) -> Result<i64, io::Error>
where P: AsRef<Path>, {
    let mut vec = Vec::new();
    let mut map = HashMap::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let mut split = line.split_whitespace();

            if let (Some(first), Some(second)) = (split.next(), split.next()) {
                match (first.parse::<i64>(), second.parse::<i64>()) {
                    (Ok(a), Ok(b)) => {
                        vec.push(a);
                        *map.entry(b).or_insert(0) += 1;
                    }
                    _ => panic!("Invalid input line: {}", line)
                }
            }
        }
    }

    Ok(vec.into_iter()
        .map(|a| a * map.get(&a).unwrap_or(&0))
        .sum())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
