/*
--- Part Two ---

The escalator doesn't move. The Elf explains that it probably needs more joltage to overcome the static friction of the system and hits the big red "joltage limit safety override" button. You lose count of the number of times she needs to confirm "yes, I'm sure" and decorate the lobby a bit while you wait.

Now, you need to make the largest joltage by turning on exactly twelve batteries within each bank.

The joltage output for the bank is still the number formed by the digits of the batteries you've turned on; the only difference is that now there will be 12 digits in each bank's joltage output instead of two.

Consider again the example from before:

987654321111111
811111111111119
234234234234278
818181911112111

Now, the joltages are much larger:

    In 987654321111111, the largest joltage can be found by turning on everything except some 1s at the end to produce 987654321111.
    In the digit sequence 811111111111119, the largest joltage can be found by turning on everything except some 1s, producing 811111111119.
    In 234234234234278, the largest joltage can be found by turning on everything except a 2 battery, a 3 battery, and another 2 battery near the start to produce 434234234278.
    In 818181911112111, the joltage 888911112111 is produced by turning on everything except some 1s near the front.

The total output joltage is now much larger: 987654321111 + 811111111119 + 434234234278 + 888911112111 = 3121910778619./
*/

mod joltage;
use joltage::{JoltageLayout, JoltageParsingError};
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_joltage<const N: usize, const M: usize>(joltage: JoltageLayout<N>) -> u64 {
    assert!(M > 0 && M <= N);

    let mut to_drop = N - M;
    let mut stack = Vec::with_capacity(M);
    joltage.batteries.iter().for_each(|b| {
        while to_drop > 0 && stack.last().is_some_and(|last| *last < *b) {
            stack.pop();
            to_drop -= 1;
        }
        stack.push(*b);
    });
    stack.truncate(M);
    let mut out = [0u8; M];
    out.copy_from_slice(&stack[..M]);
    out.iter().fold(0, |acc, b| acc * 10 + *b as u64)
}

fn calculate_joltage_sum<const N: usize>(joltages: Vec<JoltageLayout<N>>) -> u64 {
    joltages
        .iter()
        .map(|j| calculate_joltage::<N, 12>(*j))
        .sum::<u64>()
}

fn read_file<const N: usize>(file_path: &str) -> Result<Vec<JoltageLayout<N>>, Box<dyn Error>> {
    let h_file = File::open(file_path)?;
    let reader = BufReader::new(h_file);
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<String>, _>>()?;
    lines
        .iter()
        .map(|l| l.to_string().parse::<JoltageLayout<N>>())
        .collect::<Result<Vec<JoltageLayout<N>>, _>>()
        .map_err(|e: JoltageParsingError| -> Box<dyn Error> { Box::new(e) })
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).expect("usage: aoc3pt2 <input-file>");
    let products: Vec<JoltageLayout<100>> = read_file(&path)?;
    let total = calculate_joltage_sum(products);
    println!("final = {total}");
    Ok(())
}
