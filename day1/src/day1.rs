use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day1() -> io::Result<()> {
    let path = Path::new("./input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut total = 0;

    // Itera sobre cada línea en el archivo
    for line in reader.lines() {
        let line = line?;

        // Encuentra el primer y último dígito en la línea
        if let (Some(first_digit), Some(last_digit)) = (
            line.chars().find(|c| c.is_digit(10)),
            line.chars().rev().find(|c| c.is_digit(10)),
        ) {
            // Convierte los caracteres a dígitos y suma a la suma total
            let suma_linea: u32 = format!("{}{}", first_digit, last_digit)
                .parse()
                .unwrap_or(0);
            total += suma_linea;
        }
    }

    // Imprime la suma total
    println!("Total is: {}", total);

    Ok(())
}
