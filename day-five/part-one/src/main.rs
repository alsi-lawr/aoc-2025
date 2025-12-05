/*
--- Day 5: Cafeteria ---

As the forklifts break through the wall, the Elves are delighted to discover that there was a cafeteria on the other side after all.

You can hear a commotion coming from the kitchen. "At this rate, we won't have any time left to put the wreaths up in the dining hall!" Resolute in your quest, you investigate.

"If only we hadn't switched to the new inventory management system right before Christmas!" another Elf exclaims. You ask what's going on.

The Elves in the kitchen explain the situation: because of their complicated new inventory management system, they can't figure out which of their ingredients are fresh and which are spoiled. When you ask how it works, they give you a copy of their database (your puzzle input).

The database operates on ingredient IDs. It consists of a list of fresh ingredient ID ranges, a blank line, and a list of available ingredient IDs. For example:

3-5
10-14
16-20
12-18

1
5
8
11
17
32

The fresh ID ranges are inclusive: the range 3-5 means that ingredient IDs 3, 4, and 5 are all fresh. The ranges can also overlap; an ingredient ID is fresh if it is in any range.

The Elves are trying to determine which of the available ingredient IDs are fresh. In this example, this is done as follows:

    Ingredient ID 1 is spoiled because it does not fall into any range.
    Ingredient ID 5 is fresh because it falls into range 3-5.
    Ingredient ID 8 is spoiled.
    Ingredient ID 11 is fresh because it falls into range 10-14.
    Ingredient ID 17 is fresh because it falls into range 16-20 as well as range 12-18.
    Ingredient ID 32 is spoiled.

So, in this example, 3 of the available ingredient IDs are fresh.

Process the database file from the new inventory management system. How many of the available ingredient IDs are fresh?
*/

use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

type Range = (u64, u64);
type Ranges = Vec<Range>;
type Id = u64;
type Ids = Vec<Id>;

fn calculate_total_fresh_ingredients(ranges: Ranges, ids: Ids) -> u64 {
    ids.iter()
        .map(|id| {
            ranges
                .iter()
                .any(|(lower, upper)| id >= lower && id <= upper)
        })
        .filter(|is_fresh| *is_fresh)
        .map(|_| 1)
        .sum()
}

fn read_file(file_path: &str) -> Result<(Ranges, Ids), Box<dyn Error>> {
    let h_file = File::open(file_path)?;
    let reader = BufReader::new(h_file);
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;
    let mut it = lines.into_iter();
    let ranges = it
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut range = l.split("-");
            let lower: u64 = range.next().unwrap_or("0").parse()?;
            let upper: u64 = range.next().unwrap_or("0").parse()?;
            Ok::<(u64, u64), ParseIntError>((lower, upper))
        })
        .collect::<Result<Vec<(u64, u64)>, ParseIntError>>()
        .map_err(Box::new)?;
    let ids = it
        .map(|l| l.parse::<u64>())
        .collect::<Result<Vec<u64>, ParseIntError>>()
        .map_err(Box::new)?;
    Ok((ranges, ids))
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).expect("usage: aoc5pt1 <input-file>");
    let (ranges, ids): (Ranges, Ids) = read_file(&path)?;
    let total = calculate_total_fresh_ingredients(ranges, ids);
    println!("final = {total}");
    Ok(())
}
