/*
--- Part Two ---

The big cephalopods come back to check on how things are going. When they see that your grand total doesn't match the one expected by the worksheet, they realize they forgot to explain how to read cephalopod math.

Cephalopod math is written right-to-left in columns. Each number is given in its own column, with the most significant digit at the top and the least significant digit at the bottom. (Problems are still separated with a column consisting only of spaces, and the symbol at the bottom of the problem is still the operator to use.)

Here's the example worksheet again:

123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +

Reading the problems right-to-left one column at a time, the problems are now quite different:

    The rightmost problem is 4 + 431 + 623 = 1058
    The second problem from the right is 175 * 581 * 32 = 3253600
    The third problem from the right is 8 + 248 + 369 = 625
    Finally, the leftmost problem is 356 * 24 * 1 = 8544

Now, the grand total is 1058 + 3253600 + 625 + 8544 = 3263827.

Solve the problems on the math worksheet again. What is the grand total found by adding together all of the answers to the individual problems?

*/

use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Multiply,
    Unknown,
}

type Problem = (Vec<u64>, Operation);

fn calculate_answer_sum(problems: Vec<Problem>) -> u64 {
    problems.iter().fold(0, |acc, problem| {
        acc + match problem.1 {
            Operation::Add => problem.0.iter().sum::<u64>(),
            Operation::Multiply => problem.0.iter().product(),
            Operation::Unknown => 0,
        }
    })
}

fn transpose<T: Clone>(v: &[Vec<T>]) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let rows = v.len();
    (0..len)
        .map(|c| (0..rows).map(|r| v[r][c].clone()).collect::<Vec<T>>())
        .collect()
}

fn parse_segment(grid: &[Vec<char>], num_rows: usize, start: usize, end: usize) -> Vec<u64> {
    (start..end)
        .rev()
        .map(|col| {
            grid[col][0..num_rows]
                .iter()
                .collect::<String>()
                .trim()
                .parse::<u64>()
                .unwrap()
        })
        .collect()
}

fn parse_op(op_row: &[char], start: usize, end: usize) -> Operation {
    match op_row[start..end].iter().collect::<String>().trim() {
        "+" => Operation::Add,
        "*" => Operation::Multiply,
        _ => Operation::Unknown,
    }
}

fn read_file(file_path: &str) -> Result<Vec<Problem>, Box<dyn Error>> {
    let h_file = File::open(file_path)?;
    let reader = BufReader::new(h_file);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect();

    let num_operands = lines.len() - 1;
    let width = lines[0].len();

    let grid: Vec<Vec<char>> = transpose(
        &lines[0..num_operands]
            .iter()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>(),
    );

    let mut segments = Vec::new();
    let mut in_seg = false;
    let mut seg_start = 0usize;

    for (col, grid_col) in grid.iter().enumerate().take(width) {
        let all_space = (0..num_operands).all(|r| grid_col[r].is_whitespace());
        let leaving_seg = in_seg && all_space;
        let entering_seg = !in_seg && !all_space;
        if leaving_seg {
            in_seg = false;
            segments.push((seg_start, col));
        }
        if entering_seg {
            in_seg = true;
            seg_start = col;
        }
    }

    if in_seg {
        segments.push((seg_start, width));
    }

    let operator_row = lines[num_operands].chars().collect::<Vec<char>>();
    Ok(segments
        .into_iter()
        .map(|(start, end)| {
            (
                parse_segment(&grid, num_operands, start, end),
                parse_op(&operator_row, start, end),
            )
        })
        .collect())
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).expect("usage: aoc6pt2 <input-file>");
    let problems: Vec<Problem> = read_file(&path)?;
    let total = calculate_answer_sum(problems);
    println!("final = {total}");
    Ok(())
}
