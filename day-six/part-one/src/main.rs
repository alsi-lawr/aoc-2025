/*
--- Day 6: Trash Compactor ---

After helping the Elves in the kitchen, you were taking a break and helping them re-enact a movie scene when you over-enthusiastically jumped into the garbage chute!

A brief fall later, you find yourself in a garbage smasher. Unfortunately, the door's been magnetically sealed.

As you try to find a way out, you are approached by a family of cephalopods! They're pretty sure they can get the door open, but it will take some time. While you wait, they're curious if you can help the youngest cephalopod with her math homework.

Cephalopod math doesn't look that different from normal math. The math worksheet (your puzzle input) consists of a list of problems; each problem has a group of numbers that need to be either added (+) or multiplied (*) together.

However, the problems are arranged a little strangely; they seem to be presented next to each other in a very long horizontal list. For example:

123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +

Each problem's numbers are arranged vertically; at the bottom of the problem is the symbol for the operation that needs to be performed. Problems are separated by a full column of only spaces. The left/right alignment of numbers within each problem can be ignored.

So, this worksheet contains four problems:

    123 * 45 * 6 = 33210
    328 + 64 + 98 = 490
    51 * 387 * 215 = 4243455
    64 + 23 + 314 = 401

To check their work, cephalopod students are given the grand total of adding together all of the answers to the individual problems. In this worksheet, the grand total is 33210 + 490 + 4243455 + 401 = 4277556.

Of course, the actual worksheet is much wider. You'll need to make sure to unroll it completely so that you can read the problems clearly.

Solve the problems on the math worksheet. What is the grand total found by adding together all of the answers to the individual problems?
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
}

type Problem = (Vec<u64>, Operation);

fn calculate_answer_sum(problems: Vec<Problem>) -> u64 {
    problems.iter().fold(0, |acc, problem| {
        acc + match problem.1 {
            Operation::Add => problem.0.iter().sum::<u64>(),
            Operation::Multiply => problem.0.iter().product(),
        }
    })
}

fn make_problems(problem_inputs: &[Vec<u64>], operators: &[Operation]) -> Vec<Problem>
where
    Operation: Clone,
{
    let problem_count = operators.len();
    let inputs_by_problem: Vec<Vec<u64>> = (0..problem_count)
        .map(|c| problem_inputs.iter().map(|v| v[c]).collect::<Vec<u64>>())
        .collect();

    inputs_by_problem
        .into_iter()
        .zip(operators.iter())
        .map(|(inputs, op)| (inputs, *op))
        .collect()
}

fn read_file(file_path: &str) -> Result<Vec<Problem>, Box<dyn Error>> {
    let h_file = File::open(file_path)?;
    let reader = BufReader::new(h_file);
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;
    let mut it = lines.clone().into_iter();
    let problem_inputs = it
        .by_ref()
        .take_while(|l| !l.is_empty() && !l.contains("*") && !l.contains("+"))
        .map(|l| {
            l.split_whitespace()
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let operators = lines
        .into_iter()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|o| match o {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!("unknown operator: {o}"),
        })
        .collect::<Vec<_>>();

    Ok(make_problems(&problem_inputs, &operators))
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).expect("usage: aoc6pt1 <input-file>");
    let problems: Vec<Problem> = read_file(&path)?;
    let total = calculate_answer_sum(problems);
    println!("final = {total}");
    Ok(())
}
