use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part2() -> io::Result<()> {
    let path = Path::new("./input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let line = line?;

        let digit_mappings = vec![
            ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
            ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)
        ];

        let mut first_digit = None;
        let mut last_digit = None;

        for (digit_str, digit_val) in digit_mappings.iter() {
            if let Some(start_idx) = line.find(digit_str) {
                if first_digit.is_none() {
                    first_digit = Some(digit_val);
                }
            }
            if let Some(end_idx) = line.rfind(digit_str) {
                if last_digit.is_none() {
                    last_digit = Some(digit_val);
                }
            }
        }

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            total += first + last;
        }
    }

    println!("Total result for part 2 is: {}", total);

    Ok(())
}
