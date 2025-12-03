/*
--- Day 2: Gift Shop ---

You get inside and take the elevator to its only other stop: the gift shop. "Thank you for visiting the North Pole!" gleefully exclaims a nearby sign. You aren't sure who is even allowed to visit the North Pole, but you know you can access the lobby through here, and from there you can access the rest of the North Pole base.

As you make your way through the surprisingly extensive selection, one of the clerks recognizes you and asks for your help.

As it turns out, one of the younger Elves was playing on a gift shop computer and managed to add a whole bunch of invalid product IDs to their gift shop database! Surely, it would be no trouble for you to identify the invalid product IDs for them, right?

They've even checked most of the product ID ranges already; they only have a few product ID ranges (your puzzle input) that you'll need to check. For example:

11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124

(The ID ranges are wrapped here for legibility; in your input, they appear on a single long line.)

The ranges are separated by commas (,); each range gives its first ID and last ID separated by a dash (-).

Since the young Elf was just doing silly patterns, you can find the invalid IDs by looking for any ID which is made only of some sequence of digits repeated twice. So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.

None of the numbers have leading zeroes; 0101 isn't an ID at all. (101 is a valid ID that you would ignore.)

Your job is to find all of the invalid IDs that appear in the given ranges. In the above example:

    11-22 has two invalid IDs, 11 and 22.
    95-115 has one invalid ID, 99.
    998-1012 has one invalid ID, 1010.
    1188511880-1188511890 has one invalid ID, 1188511885.
    222220-222224 has one invalid ID, 222222.
    1698522-1698528 contains no invalid IDs.
    446443-446449 has one invalid ID, 446446.
    38593856-38593862 has one invalid ID, 38593859.
    The rest of the ranges contain no invalid IDs.

Adding up all the invalid IDs in this example produces 1227775554.
*/

mod products;
use products::{ProductInfo, ProductParsingError};
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn list_factors(number: u32) -> Vec<u32> {
    let mut factors: Vec<u32> = Vec::new();
    let mut i: u32 = 1;
    while i * i <= number {
        if number.is_multiple_of(i) {
            factors.push(i);
            if i * i != number {
                factors.push(number / i);
            }
        }
        i += 1;
    }
    factors
}

fn is_invalid_id(id: u64) -> bool {
    let num_digits: u32 = id.checked_ilog10().unwrap_or(0) + 1;
    if num_digits == 1 {
        return false;
    }
    for factor in list_factors(num_digits) {
        if factor == num_digits {
            continue;
        }

        let base = 10u64.pow(num_digits - factor);
        let min_base = 10u64.pow(factor);
        let mut cur_base = base;
        let mut cur_id = id;
        let first_digit = cur_id / cur_base;
        let mut found_repeat = true;
        while cur_base >= 1 {
            let digit = cur_id / cur_base;
            if digit != first_digit {
                found_repeat = false;
                break;
            }
            cur_id %= cur_base;
            cur_base /= min_base;
        }
        if found_repeat {
            return true;
        }
    }
    false
}

fn calculate_invalid_id_sum(products: Vec<ProductInfo>) -> u64 {
    let mut invalid_ids: Vec<u64> = Vec::new();
    for product in products {
        for id in product.lower_id..product.upper_id {
            if is_invalid_id(id) {
                invalid_ids.push(id);
            }
        }
    }
    invalid_ids.iter().sum::<u64>()
}

fn read_file(file_path: &str) -> Result<Vec<ProductInfo>, Box<dyn Error>> {
    let h_file = File::open(file_path)?;
    let reader = BufReader::new(h_file);
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<String>, _>>()?;
    match lines.first() {
        Some(line) => line
            .split(',')
            .map(|p| p.to_string().parse::<ProductInfo>())
            .collect(),
        None => Err(ProductParsingError::EmptyFile {
            raw: file_path.to_string(),
        }),
    }
    .map_err(|e: ProductParsingError| -> Box<dyn Error> { Box::new(e) })
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).expect("usage: aoc2pt1 <input-file>");
    let products: Vec<ProductInfo> = read_file(&path)?;
    let total = calculate_invalid_id_sum(products);
    println!("final = {total}");
    Ok(())
}
