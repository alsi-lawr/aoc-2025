use std::{
    collections::HashSet,
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpaceType {
    Empty,
    Beam,
    Splitter,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct CartesianCoordinate {
    pub x: usize,
    pub y: usize,
}

type SpaceMeta = (CartesianCoordinate, SpaceType);
type ProblemMeta = (Vec<SpaceMeta>, Vec<SpaceMeta>, usize, usize);

fn calculate_total_beams(
    beams: Vec<SpaceMeta>,
    splitters: Vec<SpaceMeta>,
    width: usize,
    height: usize,
) -> u64 {
    let splitter_coords: HashSet<CartesianCoordinate> =
        splitters.into_iter().map(|(coord, _)| coord).collect();

    let mut counts = vec![vec![0_u64; width]; height];

    for (coord, _) in beams {
        counts[coord.x][coord.y] += 1;
    }

    let mut total = 0;

    for r in 0..height {
        for c in 0..width {
            let count = counts[r][c];
            if count == 0 {
                continue;
            }

            let next_row = r + 1;
            if next_row >= height {
                total += count;
                continue;
            }

            let down = CartesianCoordinate { x: next_row, y: c };

            if splitter_coords.contains(&down) {
                if c > 0 {
                    counts[next_row][c - 1] += count;
                } else {
                    total += count;
                }

                if c < width - 1 {
                    counts[next_row][c + 1] += count;
                } else {
                    total += count;
                }
            } else {
                counts[next_row][c] += count;
            }
        }
    }
    total
}

fn char_to_space_type(c: &char) -> SpaceType {
    match c {
        '.' => SpaceType::Empty,
        'S' | '|' => SpaceType::Beam,
        '^' => SpaceType::Splitter,
        _ => SpaceType::Empty,
    }
}

fn read_file(file_path: &str) -> Result<ProblemMeta, Box<dyn Error>> {
    let h_file = File::open(file_path)?;
    let reader = BufReader::new(h_file);
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;

    let height = lines.len();
    let width = lines.first().map(|s| s.len()).unwrap_or(0);

    let meta_data: Vec<SpaceMeta> = lines
        .iter()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            line.chars().enumerate().map(move |(char_idx, char)| {
                (
                    CartesianCoordinate {
                        x: line_idx,
                        y: char_idx,
                    },
                    char_to_space_type(&char),
                )
            })
        })
        .collect();

    let beams: Vec<SpaceMeta> = meta_data
        .iter()
        .copied()
        .filter(|(_, space_type)| *space_type == SpaceType::Beam)
        .collect();

    let splitters: Vec<SpaceMeta> = meta_data
        .iter()
        .copied()
        .filter(|(_, space_type)| *space_type == SpaceType::Splitter)
        .collect();

    Ok((beams, splitters, width, height))
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).expect("usage: aoc7pt2 <input-file>");
    let (beams, splitters, width, height): ProblemMeta = read_file(&path)?;
    let total = calculate_total_beams(beams, splitters, width, height);
    println!("final = {total}");
    Ok(())
}

/*
--- Part Two ---

With your analysis of the manifold complete, you begin fixing the teleporter. However, as you open the side of the teleporter to replace the broken manifold, you are surprised to discover that it isn't a classical tachyon manifold - it's a quantum tachyon manifold.

With a quantum tachyon manifold, only a single tachyon particle is sent through the manifold. A tachyon particle takes both the left and right path of each splitter encountered.

Since this is impossible, the manual recommends the many-worlds interpretation of quantum tachyon splitting: each time a particle reaches a splitter, it's actually time itself which splits. In one timeline, the particle went left, and in the other timeline, the particle went right.

To fix the manifold, what you really need to know is the number of timelines active after a single particle completes all of its possible journeys through the manifold.

In the above example, there are many timelines. For instance, there's the timeline where the particle always went left:

.......S.......
.......|.......
......|^.......
......|........
.....|^.^......
.....|.........
....|^.^.^.....
....|..........
...|^.^...^....
...|...........
..|^.^...^.^...
..|............
.|^...^.....^..
.|.............
|^.^.^.^.^...^.
|..............

Or, there's the timeline where the particle alternated going left and right at each splitter:

.......S.......
.......|.......
......|^.......
......|........
......^|^......
.......|.......
.....^|^.^.....
......|........
....^.^|..^....
.......|.......
...^.^.|.^.^...
.......|.......
..^...^|....^..
.......|.......
.^.^.^|^.^...^.
......|........

Or, there's the timeline where the particle ends up at the same point as the alternating timeline, but takes a totally different path to get there:

.......S.......
.......|.......
......|^.......
......|........
.....|^.^......
.....|.........
....|^.^.^.....
....|..........
....^|^...^....
.....|.........
...^.^|..^.^...
......|........
..^..|^.....^..
.....|.........
.^.^.^|^.^...^.
......|........

In this example, in total, the particle ends up on 40 different timelines.

Apply the many-worlds interpretation of quantum tachyon splitting to your manifold diagram. In total, how many different timelines would a single tachyon particle end up on?
*/
