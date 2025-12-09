use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

type Vertex = (i64, i64);

fn get_largest_rectangle_area(vertices: &[Vertex]) -> i64 {
    let mut largest_area = 0;
    for i in 0..vertices.len() {
        for j in i + 1..vertices.len() {
            let v1 = vertices[i];
            let v2 = vertices[j];
            let area = ((v1.0 - v2.0 + 1) * (v1.1 - v2.1 + 1)).abs();
            if area > largest_area {
                largest_area = area;
            }
        }
    }
    largest_area
}

fn read_file(file_path: &str) -> Result<Vec<Vertex>, Box<dyn Error>> {
    let h_file = File::open(file_path)?;
    let reader = BufReader::new(h_file);
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;

    let points: Vec<Vertex> = lines
        .iter()
        .map(|line| {
            let mut split = line.split(',');
            let x = split.next().unwrap().parse().unwrap();
            let y = split.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();

    Ok(points)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).expect("usage: aoc9pt1 <input-file>");
    let vertices: Vec<Vertex> = read_file(&path)?;
    let total = get_largest_rectangle_area(&vertices);
    println!("final = {total}");
    Ok(())
}

/*
--- Day 9: Movie Theater ---

You slide down the firepole in the corner of the playground and land in the North Pole base movie theater!

The movie theater has a big tile floor with an interesting pattern. Elves here are redecorating the theater by switching out some of the square tiles in the big grid they form. Some of the tiles are red; the Elves would like to find the largest rectangle that uses red tiles for two of its opposite corners. They even have a list of where the red tiles are located in the grid (your puzzle input).

For example:

7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3

Showing red tiles as # and other tiles as ., the above arrangement of red tiles would look like this:

..............
.......#...#..
..............
..#....#......
..............
..#......#....
..............
.........#.#..
..............

You can choose any two red tiles as the opposite corners of your rectangle; your goal is to find the largest rectangle possible.

For example, you could make a rectangle (shown as O) with an area of 24 between 2,5 and 9,7:

..............
.......#...#..
..............
..#....#......
..............
..OOOOOOOO....
..OOOOOOOO....
..OOOOOOOO.#..
..............

Or, you could make a rectangle with area 35 between 7,1 and 11,7:

..............
.......OOOOO..
.......OOOOO..
..#....OOOOO..
.......OOOOO..
..#....OOOOO..
.......OOOOO..
.......OOOOO..
..............

You could even make a thin rectangle with an area of only 6 between 7,3 and 2,3:

..............
.......#...#..
..............
..OOOOOO......
..............
..#......#....
..............
.........#.#..
..............

Ultimately, the largest rectangle you can make in this example has area 50. One way to do this is between 2,5 and 11,1:

..............
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..............
.........#.#..
..............

Using two red tiles as opposite corners, what is the largest area of any rectangle you can make?
*/
