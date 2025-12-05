/*
--- Part Two ---

The Elves start bringing their spoiled inventory to the trash chute at the back of the kitchen.

So that they can stop bugging you when they get new inventory, the Elves would like to know all of the IDs that the fresh ingredient ID ranges consider to be fresh. An ingredient ID is still considered fresh if it is in any range.

Now, the second section of the database (the available ingredient IDs) is irrelevant. Here are the fresh ingredient ID ranges from the above example:

3-5
10-14
16-20
12-18

The ingredient IDs that these ranges consider to be fresh are 3, 4, 5, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, and 20. So, in this example, the fresh ingredient ID ranges consider a total of 14 ingredient IDs to be fresh.

Process the database file again. How many ingredient IDs are considered to be fresh according to the fresh ingredient ID ranges?

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

fn calculate_total_fresh_ingredients(ranges: &Ranges) -> u64 {
    let mut fresh_ids = ranges.to_vec();
    fresh_ids.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    let mut total = 0u64;
    let mut cur_start = fresh_ids[0].0;
    let mut cur_end = fresh_ids[0].1;

    for (start, end) in fresh_ids.iter().skip(1) {
        if *start > cur_end {
            total += cur_end - cur_start + 1;
            (cur_start, cur_end) = (*start, *end);
        } else {
            cur_end = cur_end.max(*end);
        }
    }

    total += cur_end - cur_start + 1;
    total
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
            let lower: Id = range.next().unwrap_or("0").parse()?;
            let upper: Id = range.next().unwrap_or("0").parse()?;
            Ok::<Range, ParseIntError>((lower, upper))
        })
        .collect::<Result<Ranges, ParseIntError>>()
        .map_err(Box::new)?;
    let ids = it
        .map(|l| l.parse::<Id>())
        .collect::<Result<Ids, ParseIntError>>()
        .map_err(Box::new)?;
    Ok((ranges, ids))
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).expect("usage: aoc5pt2 <input-file>");
    let (ranges, _): (Ranges, Ids) = read_file(&path)?;
    let total = calculate_total_fresh_ingredients(&ranges);
    println!("final = {total}");
    Ok(())
}
