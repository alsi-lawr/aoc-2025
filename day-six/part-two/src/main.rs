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

type Problem = (Vec<i128>, Operation);

fn to_cephalopod(mut split_nums: Vec<Vec<char>>) -> Vec<i128> {
    let cols = split_nums
        .iter()
        .map(|inner| inner.len())
        .max()
        .unwrap_or(0);
    split_nums
        .iter_mut()
        .for_each(|inner| inner.resize(cols, ' '));
    let rows = split_nums.len();
    let rotated = (0..cols)
        .map(|c| {
            (0..rows)
                .map(|r| split_nums[r][cols - 1 - c])
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    rotated
        .iter()
        .map(|inner| inner.iter().collect::<String>())
        .map(|s| s.trim().parse::<i128>().unwrap_or(-1))
        .filter(|n| *n != -1)
        .collect::<Vec<i128>>()
}

fn calculate_answer_sum(problems: Vec<Problem>) -> i128 {
    problems.iter().fold(0, |acc, problem| {
        acc + match problem.1 {
            Operation::Add => problem.0.iter().sum::<i128>(),
            Operation::Multiply => problem.0.iter().product(),
            Operation::Unknown => 0,
        }
    })
}

fn make_problems(problem_inputs: &[Vec<Vec<char>>], operators: &[Operation]) -> Vec<Problem>
where
    Operation: Clone,
{
    let problem_count = operators.len();
    let inputs_by_problem: Vec<Vec<Vec<char>>> = (0..problem_count)
        .map(|c| {
            problem_inputs
                .iter()
                .map(|v| v[c].clone())
                .collect::<Vec<Vec<char>>>()
        })
        .collect();

    inputs_by_problem
        .into_iter()
        .zip(operators.iter())
        .map(|(inputs, op)| (to_cephalopod(inputs), *op))
        .collect()
}

fn read_file(file_path: &str) -> Result<Vec<Problem>, Box<dyn Error>> {
    let h_file = File::open(file_path)?;
    let reader = BufReader::new(h_file);
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;
    let lines = lines
        .iter()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let num_rows = 4;
    let mut problem_inputs: Vec<Vec<Vec<char>>> = Vec::new();
    for i in 0..num_rows {
        let mut line_parsed: Vec<Vec<char>> = Vec::new();
        let mut parsed: Vec<char> = Vec::new();
        let other_indexes: Vec<usize> = (0..num_rows).filter(|j| *j != i).collect();
        let chars = &lines[i];
        for j in 0..chars.len() {
            let c = chars[j];
            let mut is_end = true;
            for k in other_indexes.iter() {
                if lines[*k][j] != ' ' {
                    is_end = false;
                }
            }
            if c == ' ' && is_end {
                line_parsed.push(parsed);
                parsed = Vec::new();
            } else {
                parsed.push(c);
            }
        }
        line_parsed.push(parsed);
        problem_inputs.push(line_parsed);
    }
    let operators = lines[num_rows]
        .iter()
        .map(|o| match *o {
            '+' => Operation::Add,
            '*' => Operation::Multiply,
            _ => Operation::Unknown,
        })
        .filter(|o| *o != Operation::Unknown)
        .collect::<Vec<_>>();

    Ok(make_problems(&problem_inputs, &operators))
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).expect("usage: aoc6pt2 <input-file>");
    let problems: Vec<Problem> = read_file(&path)?;
    let total = calculate_answer_sum(problems);
    println!("final = {total}");
    Ok(())
}
