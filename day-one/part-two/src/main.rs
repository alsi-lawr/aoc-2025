mod safe_combo;
use safe_combo::{PasswordCounter, SafeCombo, SafeComboParsingError};
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_password(input: Vec<SafeCombo>) -> PasswordCounter {
    const START_ROT: i16 = 50;
    let init: PasswordCounter = PasswordCounter {
        total: START_ROT,
        password: 0,
    };
    input.iter().fold(init, |count, combo| count + combo)
}

fn read_file(file_path: &str) -> Result<Vec<SafeCombo>, Box<dyn Error>> {
    let h_file = File::open(file_path)?;
    let reader = BufReader::new(h_file);
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<String>, _>>()?;
    Ok(lines
        .into_iter()
        .map(|s| {
            s.trim()
                .to_owned()
                .parse::<SafeCombo>()
                .map_err(|e: SafeComboParsingError| -> Box<dyn Error> { Box::new(e) })
        })
        .collect::<Result<Vec<SafeCombo>, Box<dyn Error>>>()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).expect("usage: part-two <input-file>");
    let combinations: Vec<SafeCombo> = read_file(&path)?;
    let PasswordCounter { total, password } = calculate_password(combinations);
    println!("final = {total}, password = {password}");
    Ok(())
}
