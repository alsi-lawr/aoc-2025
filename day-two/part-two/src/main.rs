/*
--- Day 2: Gift Shop ---
--- Part Two ---

The clerk quickly discovers that there are still invalid IDs in the ranges in your list. Maybe the young Elf was doing other silly patterns as well?

Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice. So, 12341234 (1234 two times), 123123123 (123 three times), 1212121212 (12 five times), and 1111111 (1 seven times) are all invalid IDs.

From the same example as before:

    11-22 still has two invalid IDs, 11 and 22.
    95-115 now has two invalid IDs, 99 and 111.
    998-1012 now has two invalid IDs, 999 and 1010.
    1188511880-1188511890 still has one invalid ID, 1188511885.
    222220-222224 still has one invalid ID, 222222.
    1698522-1698528 still contains no invalid IDs.
    446443-446449 still has one invalid ID, 446446.
    38593856-38593862 still has one invalid ID, 38593859.
    565653-565659 now has one invalid ID, 565656.
    824824821-824824827 now has one invalid ID, 824824824.
    2121212118-2121212124 now has one invalid ID, 2121212121.

Adding up all the invalid IDs in this example produces 4174379265.

What do you get if you add up all of the invalid IDs using these new rules?
*/

mod products;
use products::{ProductInfo, ProductParsingError};
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const fn gen_multiple(factor: u32, num_digits: u32) -> u64 {
    let min_base = 10u64.pow(factor);
    let mut multiple = 1;
    let mut current_base = min_base;
    while current_base < 10u64.pow(num_digits) {
        multiple += current_base;
        current_base *= min_base;
    }
    multiple
}

const fn gen_factors<const N: usize>(d: usize) -> [u32; N] {
    let mut factors: [u32; N] = [0; N];
    let mut i: u32 = 1;
    while i * i <= d as u32 {
        if d.is_multiple_of(i as usize) {
            factors[i as usize] = 1;
            if i * i != d as u32 {
                factors[(d as u32 / i) as usize] = 1;
            }
        }
        i += 1;
    }
    factors
}

const fn gen_multiples<const N: usize>() -> [[u64; N]; N] {
    let mut multiples: [[u64; N]; N] = [[0; N]; N];
    let mut i: usize = 0;
    while i < N {
        let factors: [u32; N] = gen_factors::<N>(i);
        let mut j: usize = 0;
        while j < N {
            if factors[j] == 0 {
                j += 1;
                continue;
            }
            multiples[i][j] = gen_multiple(j as u32, i as u32);
            j += 1;
        }
        i += 1;
    }
    multiples
}

const MULTIPLES: [[u64; 12]; 12] = gen_multiples::<12>();

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
        let min_base = 10u64.pow(factor);
        let expected = MULTIPLES[num_digits as usize][factor as usize] * (id % min_base);
        if expected == id {
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
