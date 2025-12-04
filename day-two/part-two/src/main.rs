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

const fn calc_multiple(factor: u32, num_digits: u32) -> u64 {
    let min_base = 10u64.pow(factor);
    let mut multiple = 1;
    let mut current_base = min_base;
    while current_base < 10u64.pow(num_digits) {
        multiple += current_base;
        current_base *= min_base;
    }
    multiple
}

const fn calc_factors<const N: usize>(d: usize) -> [usize; N] {
    let mut factors: [usize; N] = [0; N];
    let mut i: u32 = 1;
    while i * i <= d as u32 {
        if d.is_multiple_of(i as usize) {
            factors[i as usize] = i as usize;
            if i * i != d as u32 {
                factors[(d as u32 / i) as usize] = d / i as usize;
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
        let factors: [usize; N] = calc_factors::<N>(i);
        let mut j: usize = 0;
        while j < N {
            if factors[j] == 0 {
                j += 1;
                continue;
            }
            multiples[i][j] = calc_multiple(j as u32, i as u32);
            j += 1;
        }
        i += 1;
    }
    multiples
}

const fn gen_factors<const N: usize>() -> [[usize; N]; N] {
    let mut factors: [[usize; N]; N] = [[0; N]; N];
    let mut i: usize = 0;
    while i < N {
        factors[i] = calc_factors::<N>(i);
        i += 1;
    }
    factors
}

const fn gen_powers<const N: usize>() -> [u64; N] {
    let mut bases: [u64; N] = [0; N];
    let mut i: usize = 0;
    while i < N {
        bases[i] = 10u64.pow(i as u32);
        i += 1;
    }
    bases
}

const FACTORS: [[usize; 12]; 12] = gen_factors::<12>();
const POW10: [u64; 12] = gen_powers::<12>();
const REP: [[u64; 12]; 12] = gen_multiples::<12>();

fn num_digits(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

fn sum_invalid_in_range(low: u64, high: u64) -> u64 {
    let min_d = num_digits(low);
    let max_d = num_digits(high.saturating_sub(1));

    let mut vals: Vec<u64> = Vec::<u64>::with_capacity(1000);
    vals.extend((min_d..=max_d).flat_map(|d| {
        let d_us = d as usize;
        let p = ProductInfo {
            lower_id: low.max(POW10[d_us - 1]),
            upper_id: high.min(POW10[d_us]),
        };
        FACTORS[d_us]
            .iter()
            .filter_map(move |&k| {
                if k == 0 || k >= d_us {
                    return None;
                }

                let rep = REP[d_us][k];

                let mut chunk_lo = p.lower_id.div_ceil(rep);
                let mut chunk_hi = (p.upper_id - 1) / rep;

                let k_lo = POW10[k - 1];
                let k_hi = POW10[k] - 1;

                chunk_lo = chunk_lo.max(k_lo);
                chunk_hi = chunk_hi.min(k_hi);

                if chunk_lo > chunk_hi {
                    return None;
                }

                Some((chunk_lo..=chunk_hi).map(move |chunk| rep * chunk))
            })
            .flatten()
    }));
    vals.dedup();
    vals.into_iter().sum::<u64>()
}

fn calculate_invalid_id_sum(products: Vec<ProductInfo>) -> u64 {
    use rayon::prelude::*;
    products
        .par_iter()
        .map(|p| sum_invalid_in_range(p.lower_id, p.upper_id))
        .sum()
}

fn read_file(file_path: &str) -> Result<Vec<ProductInfo>, Box<dyn Error>> {
    let h_file = File::open(file_path)?;
    let mut reader = BufReader::new(h_file);
    let mut line = String::new();
    let bytes = reader.read_line(&mut line)?;
    if bytes == 0 {
        return Err(Box::new(ProductParsingError::EmptyFile {
            raw: file_path.to_string(),
        }));
    }
    line.split(',')
        .map(|p| p.parse::<ProductInfo>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e: ProductParsingError| -> Box<dyn Error> { Box::new(e) })
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).expect("usage: aoc2pt1 <input-file>");
    let products: Vec<ProductInfo> = read_file(&path)?;
    let total = calculate_invalid_id_sum(products);
    println!("final = {total}");
    Ok(())
}
