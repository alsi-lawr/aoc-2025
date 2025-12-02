mod safe_combo;
use safe_combo::{SafeCombo, SafeComboParsingError};
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

/*
 * NOTE: dial 0-99 inclusive
 * NOTE: L = anticlockwise, R = clockwise
 * NOTE: MOD 99
 * NOTE: starts at 50
--- Day 1: Secret Entrance ---

To save Christmas, the Elves need you to finish decorating the North Pole by December 12th.

note: "Due to new security protocols, the password is locked in the safe below."

- the safe has a dial with only an arrow on it;
- around the dial are the numbers 0 through 99 in order.
- As you turn the dial, it makes a small click noise as it reaches each number.

Program input is a sequence of rotations, one per line, which tell you how to open the safe.
The dial starts by pointing at 50.

You could follow the instructions, but your recent required official North Pole secret entrance security training seminar taught you that the safe is actually a decoy.
The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence.

For example, suppose the attached document contained the following rotations:

L68
L30
R48
L5
R60
L55
L1
L99
R14
L82


OUTPUT NUMBER OF TIMES THE DIAL POINTS AT 0

Analyze the rotations in your attached document. What's the actual password to open the door?
 */

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
    let path = env::args().nth(1).expect("usage: aoc-one <input-file>");
    let combinations: Vec<SafeCombo> = read_file(&path)?;
    let (final_rot, pw) = calculate_password(combinations);
    println!("final = {final_rot}, password = {pw}");
    Ok(())
}
