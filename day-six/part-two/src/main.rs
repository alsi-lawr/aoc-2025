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

impl Operation {
    fn from_str(s: &str) -> Operation {
        match s.trim() {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => Operation::Unknown,
        }
    }

    fn parse_from_line(op_line: &[char], start: usize, end: usize) -> Operation {
        Operation::from_str(&op_line[start..end].iter().collect::<String>())
    }
}

struct Operand(u64);
struct Operands(Vec<Operand>);
struct Problem(Operands, Operation);
struct Problems(Vec<Problem>);
struct Matrix2D<T: Clone>(Vec<Vec<T>>);

impl<T: std::fmt::Debug + Clone + PartialEq> Matrix2D<T> {
    fn transpose(&self) -> Matrix2D<T> {
        assert!(!self.0.is_empty());
        let len = self.0[0].len();
        let rows = self.0.len();
        Matrix2D::<T>(
            (0..len)
                .map(|c| (0..rows).map(|r| self.0[r][c].clone()).collect::<Vec<T>>())
                .collect(),
        )
    }

    fn from_strings(s: &[String]) -> Matrix2D<char> {
        let num_operands = s.len() - 1;
        Matrix2D::<char>(
            s[0..=num_operands]
                .iter()
                .map(|l| l.chars().collect())
                .collect::<Vec<Vec<char>>>(),
        )
    }

    fn take_width(
        &self,
        width: usize,
    ) -> std::iter::Take<std::iter::Enumerate<std::slice::Iter<'_, std::vec::Vec<T>>>> {
        self.0.iter().enumerate().take(width)
    }

    fn take_column_slice(&self, column: usize, rows: usize) -> &[T] {
        &self.0[column][0..rows]
    }

    fn column_contains_only(&self, column: usize, c: T) -> bool {
        self.take_column_slice(column, self.0[0].len())
            .iter()
            .all(|v| *v == c)
    }
}

impl Operand {
    fn parse_from_grid(grid: &Matrix2D<char>, num_rows: usize, col: usize) -> Operand {
        Operand(
            grid.take_column_slice(col, num_rows)
                .iter()
                .collect::<String>()
                .trim()
                .parse::<u64>()
                .unwrap(),
        )
    }
}

impl Operands {
    fn parse_from_grid(
        grid: &Matrix2D<char>,
        num_rows: usize,
        start: usize,
        end: usize,
    ) -> Operands {
        Operands(
            (start..end)
                .rev()
                .map(|col| Operand::parse_from_grid(grid, num_rows, col))
                .collect(),
        )
    }

    fn sum(&self) -> u64 {
        self.0.iter().map(|o| o.0).sum::<u64>()
    }

    fn product(&self) -> u64 {
        self.0.iter().map(|o| o.0).product()
    }
}

impl Problem {
    fn answer(&self) -> u64 {
        match self.1 {
            Operation::Add => self.0.sum(),
            Operation::Multiply => self.0.product(),
            Operation::Unknown => 0,
        }
    }
}

impl Problems {
    fn sum(&self) -> u64 {
        self.0.iter().map(|p| p.answer()).sum()
    }
}

fn read_file(file_path: &str) -> Result<Problems, Box<dyn Error>> {
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

    let grid: Matrix2D<char> = Matrix2D::<char>::from_strings(&lines[0..num_operands]).transpose();

    let mut segments = Vec::new();
    let mut in_seg = false;
    let mut seg_start = 0usize;

    for (col, _) in grid.take_width(width) {
        let all_space = grid.column_contains_only(col, ' ');
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
    Ok(Problems(
        segments
            .into_iter()
            .map(|(start, end)| {
                Problem(
                    Operands::parse_from_grid(&grid, num_operands, start, end),
                    Operation::parse_from_line(&operator_row, start, end),
                )
            })
            .collect(),
    ))
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).expect("usage: aoc6pt2 <input-file>");
    let problems: Problems = read_file(&path)?;
    let total = problems.sum();
    println!("final = {total}");
    Ok(())
}
