/*
--- Day 4: Printing Department ---

You ride the escalator down to the printing department. They're clearly getting ready for Christmas; they have lots of large rolls of paper everywhere, and there's even a massive printer in the corner (to handle the really big print jobs).

Decorating here will be easy: they can make their own decorations. What you really need is a way to get further into the North Pole base while the elevators are offline.

"Actually, maybe we can help with that," one of the Elves replies when you ask for help. "We're pretty sure there's a cafeteria on the other side of the back wall. If we could break through the wall, you'd be able to keep moving. It's too bad all of our forklifts are so busy moving those big rolls of paper around."

If you can optimize the work the forklifts are doing, maybe they would have time to spare to break through the wall.

The rolls of paper (@) are arranged on a large grid; the Elves even have a helpful diagram (your puzzle input) indicating where everything is located.

For example:

..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.

The forklifts can only access a roll of paper if there are fewer than four rolls of paper in the eight adjacent positions. If you can figure out which rolls of paper the forklifts can access, they'll spend less time looking and more time breaking down the wall to the cafeteria.

In this example, there are 13 rolls of paper that can be accessed by a forklift (marked with x):

..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.

Consider your complete diagram of the paper roll locations. How many rolls of paper can be accessed by a forklift?

*/

use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_total_rolls(rolls: Vec<Vec<bool>>) -> u64 {
    let mut reachable_rolls = 0;
    for i in 0..rolls.len() {
        for j in 0..rolls[i].len() {
            if !rolls[i][j] {
                continue;
            }

            let surrounding = (i > 0 && j > 0 && rolls[i - 1][j - 1]) as u64
                + (i > 0 && rolls[i - 1][j]) as u64
                + (i > 0 && j < rolls[i].len() - 1 && rolls[i - 1][j + 1]) as u64
                + (j > 0 && rolls[i][j - 1]) as u64
                + (j < rolls[i].len() - 1 && rolls[i][j + 1]) as u64
                + (i < rolls.len() - 1 && j > 0 && rolls[i + 1][j - 1]) as u64
                + (i < rolls.len() - 1 && rolls[i + 1][j]) as u64
                + (i < rolls.len() - 1 && j < rolls[i].len() - 1 && rolls[i + 1][j + 1]) as u64;
            if surrounding < 4 {
                reachable_rolls += 1;
            }
        }
    }
    reachable_rolls
}

fn read_file(file_path: &str) -> Result<Vec<Vec<bool>>, Box<dyn Error>> {
    let h_file = File::open(file_path)?;
    let reader = BufReader::new(h_file);
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<String>, _>>()?;
    Ok(lines
        .iter()
        .map(|l| {
            l.to_string()
                .as_bytes()
                .iter()
                .map(|b| *b == b'@')
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>())
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).expect("usage: aoc4pt2 <input-file>");
    let roll_layout: Vec<Vec<bool>> = read_file(&path)?;
    let total = calculate_total_rolls(roll_layout);
    println!("final = {total}");
    Ok(())
}
