mod safe_combo;
use safe_combo::{SafeCombo, SafeComboParsingError};
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_password(input: Vec<SafeCombo>) -> (i16, usize) {
    const START_ROT: i16 = 50;
    input
        .iter()
        .fold((START_ROT, 0usize), |(total, pw), combo| {
            let new_total = total + combo;
            let new_pw = pw + usize::from(new_total == 0);
            (new_total, new_pw)
        })
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
    let path = env::args().nth(1).expect("usage: part-one <input-file>");
    let combinations: Vec<SafeCombo> = read_file(&path)?;
    let (final_rot, pw) = calculate_password(combinations);
    println!("final = {final_rot}, password = {pw}");
    Ok(())
}
