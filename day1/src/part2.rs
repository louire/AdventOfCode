use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part2() -> io::Result<()> {
    let path = Path::new("./input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut word_to_digit: HashMap<&str, u32> = HashMap::new();
    word_to_digit.insert("one", 1);
    word_to_digit.insert("two", 2);
    word_to_digit.insert("three", 3);
    word_to_digit.insert("four", 4);
    word_to_digit.insert("five", 5);
    word_to_digit.insert("six", 6);
    word_to_digit.insert("seven", 7);
    word_to_digit.insert("eight", 8);
    word_to_digit.insert("nine", 9);

    let mut total = 0;

    for line in reader.lines() {
        let line = line?;

        let (first_digit, last_digit) = line.split_at(line.find(|c: char| c.is_digit(10)).unwrap_or(line.len()));
        let first_digit = first_digit.trim();
        let last_digit = last_digit.trim();

        let line: u32 = if !first_digit.is_empty() && !last_digit.is_empty() {
            word_to_digit.get(first_digit).unwrap_or(&0) * 10
                + word_to_digit.get(last_digit).unwrap_or(&0)
        } else {
            0
        };
        total += line;
    }

    println!("Total result for part 2 is: {}", total);

    Ok(())
}