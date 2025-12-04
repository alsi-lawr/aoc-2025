/*
--- Part Two ---

Now, the Elves just need help accessing as much of the paper as they can.

Once a roll of paper can be accessed by a forklift, it can be removed. Once a roll of paper is removed, the forklifts might be able to access more rolls of paper, which they might also be able to remove. How many total rolls of paper could the Elves remove if they keep repeating this process?

Starting with the same example as above, here is one way you could remove as many rolls of paper as possible, using highlighted @ to indicate that a roll of paper is about to be removed, and using x to indicate that a roll of paper was just removed:

Initial state:
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

Remove 13 rolls of paper:
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

Remove 12 rolls of paper:
.......x..
.@@.x.x.@x
x@@@@...@@
x.@@@@..x.
.@.@@@@.x.
.x@@@@@@.x
.x.@.@.@@@
..@@@.@@@@
.x@@@@@@@.
....@@@...

Remove 7 rolls of paper:
..........
.x@.....x.
.@@@@...xx
..@@@@....
.x.@@@@...
..@@@@@@..
...@.@.@@x
..@@@.@@@@
..x@@@@@@.
....@@@...

Remove 5 rolls of paper:
..........
..x.......
.x@@@.....
..@@@@....
...@@@@...
..x@@@@@..
...@.@.@@.
..x@@.@@@x
...@@@@@@.
....@@@...

Remove 2 rolls of paper:
..........
..........
..x@@.....
..@@@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@x.
....@@@...

Remove 1 roll of paper:
..........
..........
...@@.....
..x@@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@..
....@@@...

Remove 1 roll of paper:
..........
..........
...x@.....
...@@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@..
....@@@...

Remove 1 roll of paper:
..........
..........
....x.....
...@@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@..
....@@@...

Remove 1 roll of paper:
..........
..........
..........
...x@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@..
....@@@...

Stop once no more rolls of paper are accessible by a forklift. In this example, a total of 43 rolls of paper can be removed.

Start with your original diagram. How many rolls of paper in total can be removed by the Elves and their forklifts?

*/

use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_total_rolls(rolls: Vec<Vec<bool>>) -> u64 {
    let mut rolls = rolls;
    let mut reachable_rolls = 0;
    let mut removed = true;
    let mut marked_for_removal: Vec<(usize, usize)> = Vec::new();
    while removed {
        removed = false;
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
                    marked_for_removal.push((i, j));
                    removed = true;
                }
            }
        }
        for (i, j) in marked_for_removal.iter() {
            rolls[*i][*j] = false;
        }
        marked_for_removal.clear();
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
