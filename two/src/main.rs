use std::io;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let filename = "input";

    let part_one = read_lines(filename)
        .expect("Failed to read file")
        .map(|l| l.expect("Failed to read line")
                    .split_whitespace()
                    .into_iter()
                    .map(|s| s.parse::<i32>()
                        .expect("Failed to parse integer"))
                    .collect::<Vec<_>>())
        .filter(|s| is_safe_increasing(s.as_slice()) || is_safe_decreasing(s.as_slice()))
        .count();
    println!("Part one: {}", part_one);

    let part_two = read_lines(filename)
        .expect("Failed to read file")
        .map(|l| l.expect("Failed to read line")
                    .split_whitespace()
                    .into_iter()
                    .map(|s| s.parse::<i32>()
                        .expect("Failed to parse integer"))
                    .collect::<Vec<_>>())
        .filter(|s| is_safe_increasing_two(s.as_slice()) || is_safe_decreasing_two(s.as_slice()))
        .count();
    println!("Part one: {}", part_two);
}

fn is_safe_increasing(seq: &[i32]) -> bool {
    seq.windows(2)
        .all(|w| {
            let (l, r) = (w[0], w[1]);
            0 < (l - r) && (l - r) <= 3
        })
}

fn is_safe_decreasing(seq: &[i32]) -> bool {
    seq.windows(2)
        .all(|w| {
            let (l, r) = (w[0], w[1]);
            -3 <= (l - r) && (l - r) < 0
        })
}

fn is_safe_increasing_two(seq: &[i32]) -> bool {
    for i in 0..seq.len() {
        if is_safe_increasing(&[&seq[..i], &seq[i+1..]].concat()) {
            return true
        }
    }
    false
}

fn is_safe_decreasing_two(seq: &[i32]) -> bool {
    for i in 0..seq.len() {
        if is_safe_decreasing(&[&seq[..i], &seq[i+1..]].concat()) {
            return true
        }
    }
    false
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}
