use std::{
    cmp::{max, min},
    collections::HashMap,
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Write},
};

type Vertex = (i64, i64);
type Edge = (Vertex, Vertex);

// orthogonal rectilinear convex hull problem

fn get_edge_length_rank(v1: &Vertex, v2: &Vertex) -> i64 {
    (v1.0 - v2.0) * (v1.0 - v2.0) + (v1.1 - v2.1) * (v1.1 - v2.1)
}

fn get_all_edges(vertices: &[Vertex]) -> Vec<Vertex> {
    let mut ordered_vertices: Vec<Vertex> = Vec::<Vertex>::with_capacity(vertices.len());
    let mut queue: Vec<Vertex> = Vec::new();
    queue.extend_from_slice(vertices);

    let mut p: Vertex = queue[0].clone();
    let last: Vertex = queue.last().unwrap().clone();
    queue.remove(0);
    ordered_vertices.push(last);
    ordered_vertices.push(p);
    while !queue.is_empty() {
        let mut matching_vertices: Vec<(usize, &Vertex)> = queue
            .iter()
            .enumerate()
            .filter(|(_, q)| q.0 == p.0 || q.1 == p.1)
            .map(|(j, q)| (j, q))
            .collect();

        matching_vertices.sort_unstable_by(|a, b| {
            let rank_a = get_edge_length_rank(a.1, &p);
            let rank_b = get_edge_length_rank(b.1, &p);
            rank_a.cmp(&rank_b)
        });
        let q = &matching_vertices[0];
        p = q.1.clone();
        ordered_vertices.push(*q.1);
        queue.remove(q.0);
    }
    ordered_vertices
}

fn get_largest_rectangle_area(ordered_border_vertices: &[Vertex]) -> i64 {
    let mut edges = ordered_border_vertices
        .windows(2)
        .map(|window| (window[0], window[1]))
        .collect::<Vec<Edge>>();

    if let (Some(&first), Some(&last)) = (
        ordered_border_vertices.last(),
        ordered_border_vertices.first(),
    ) {
        edges.push((last, first));
    }

    let mut point_in_polygon: HashMap<Vertex, bool> = HashMap::new();
    let mut largest_area = 0i64;
    for i in 0..ordered_border_vertices.len() {
        for j in i + 1..ordered_border_vertices.len() {
            let v1 = ordered_border_vertices[i];
            let v3 = ordered_border_vertices[j];
            let v2 = (v1.0, v3.1);
            let v4 = (v3.0, v1.1);

            let e1 = (v1, v2);
            let e2 = (v2, v3);
            let e3 = (v3, v4);
            let e4 = (v4, v1);

            let mut check_in_polygon = |p: &Vertex| -> bool {
                if point_in_polygon.contains_key(p) {
                    point_in_polygon[p]
                } else {
                    let in_polygon = point_is_in_polygon(p, &ordered_border_vertices);
                    point_in_polygon.insert(*p, in_polygon);
                    in_polygon
                }
            };

            let points_in_polygon = [v1, v2, v3, v4].iter().all(|p| check_in_polygon(p));

            if !points_in_polygon {
                continue;
            }
            let edges_intersect = edges.iter().any(|e| {
                [e1, e2, e3, e4]
                    .iter()
                    .any(|e_rect| segments_strictly_cross(e.0, e.1, e_rect.0, e_rect.1))
            });

            if edges_intersect {
                continue;
            }
            let area = ((v1.0 - v3.0).abs() + 1) * ((v1.1 - v3.1).abs() + 1);
            if area > largest_area {
                largest_area = area;
            }
        }
    }
    largest_area
}

fn segments_strictly_cross(a: Vertex, b: Vertex, c: Vertex, d: Vertex) -> bool {
    let (ax, ay) = a;
    let (bx, by) = b;
    let (cx, cy) = c;
    let (dx, dy) = d;

    let rx = bx - ax;
    let ry = by - ay;
    let sx = dx - cx;
    let sy = dy - cy;

    let denom = rx * sy - ry * sx;
    if denom == 0 {
        return false;
    }

    let cx_ax = cx - ax;
    let cy_ay = cy - ay;
    let num_t = cx_ax * sy - cy_ay * sx;
    let num_u = cx_ax * ry - cy_ay * rx;

    if denom > 0 {
        (0 < num_t && num_t < denom) && (0 < num_u && num_u < denom)
    } else {
        (denom < num_t && num_t < 0) && (denom < num_u && num_u < 0)
    }
}

fn point_is_in_polygon(point: &Vertex, polygon: &[Vertex]) -> bool {
    let (px, py) = point;
    let n = polygon.len();
    let mut inside = false;
    for i in 0..n {
        let j = (i + 1) % n;
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        if point_is_on_segment(point, &(polygon[i], polygon[j])) {
            return true;
        }

        if (yi > *py) != (yj > *py) {
            let x_intersect = ((py - yi) * (xj - xi)) / (yj - yi) + xi as i64;
            if *px < x_intersect {
                inside = !inside;
            }
        }
    }
    inside
}

fn point_is_on_segment(point: &Vertex, segment: &(Vertex, Vertex)) -> bool {
    let (px, py) = point;
    let ((x1, y1), (x2, y2)) = segment;

    min(x1, x2) <= px
        && px <= max(x1, x2)
        && min(y1, y2) <= py
        && py <= max(y1, y2)
        && (px - x1) * (y2 - y1) == (py - y1) * (x2 - x1)
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
    let path = env::args().nth(1).expect("usage: aoc9pt2 <input-file>");
    let vertices: Vec<Vertex> = read_file(&path)?;
    let edges = get_all_edges(&vertices);

    // generate_svg(&vertices, &edges, "output.svg")?;
    let total_area = get_largest_rectangle_area(&edges);
    println!("total area: {}", total_area);

    Ok(())
}

// fn test_collinear() {
//     // identical
//     assert!(edges_are_collinear(&((0, 0), (0, 5)), &((0, 0), (0, 5))));
//     assert!(edges_are_collinear(&((0, 0), (0, 5)), &((0, 5), (0, 0)))); // reversed e2
//
//     // strict subset
//     assert!(edges_are_collinear(&((0, 0), (0, 5)), &((0, 1), (0, 4))));
//     assert!(edges_are_collinear(&((0, 0), (0, 5)), &((0, 0), (0, 4)))); // shares start
//     assert!(edges_are_collinear(&((0, 0), (0, 5)), &((0, 1), (0, 5)))); // shares end
//     //
//     // // e2 extends beyond e1
//     assert!(!edges_are_collinear(&((0, 0), (0, 5)), &((0, -1), (0, 3))));
//     assert!(!edges_are_collinear(&((0, 0), (0, 5)), &((0, 3), (0, 6))));
//     assert!(!edges_are_collinear(&((0, 0), (0, 5)), &((0, -1), (0, 6)))); // superset
//
//     // disjoint but collinear
//     assert!(!edges_are_collinear(&((0, 0), (0, 5)), &((0, 6), (0, 7))));
//     assert!(!edges_are_collinear(&((0, 0), (0, 5)), &((0, -3), (0, -1))));
//
//     assert!(edges_are_collinear(&((0, 0), (5, 0)), &((1, 0), (4, 0))));
//     assert!(edges_are_collinear(&((0, 0), (5, 0)), &((0, 0), (4, 0))));
//     assert!(edges_are_collinear(&((0, 0), (5, 0)), &((1, 0), (5, 0))));
//     assert!(edges_are_collinear(&((0, 0), (5, 0)), &((5, 0), (0, 0))));
//     assert!(!edges_are_collinear(&((0, 0), (5, 0)), &((-1, 0), (3, 0))));
//     assert!(!edges_are_collinear(&((0, 0), (5, 0)), &((3, 0), (7, 0))));
//     assert!(!edges_are_collinear(&((0, 0), (5, 0)), &((-1, 0), (7, 0))));
//     assert!(!edges_are_collinear(&((0, 0), (5, 0)), &((6, 0), (8, 0))));
//     // perpendicular crossing
//     assert!(!edges_are_collinear(&((0, 0), (5, 0)), &((2, -1), (2, 1))));
//
//     // share only a point but not collinear
//     assert!(!edges_are_collinear(&((0, 0), (5, 0)), &((5, 0), (5, 3))));
//     assert!(!edges_are_collinear(&((0, 0), (0, 5)), &((0, 5), (3, 5))));
//
//     // completely unrelated
//     assert!(!edges_are_collinear(
//         &((0, 0), (5, 0)),
//         &((10, 10), (12, 12))
//     ));
// }

pub fn generate_svg(
    coordinates: &[Vertex],
    border_coordinates: &[Vertex],
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    if border_coordinates.is_empty() {
        return Ok(());
    }

    // Find bounds
    let mut min_x = i64::MAX;
    let mut max_x = 0i64;
    let mut min_y = i64::MAX;
    let mut max_y = 0i64;

    for &(x, y) in border_coordinates {
        min_x = min(min_x, x);
        max_x = max(max_x, x);
        min_y = min(min_y, y);
        max_y = max(max_y, y);
    }

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    // Scale factor - limit SVG to reasonable size
    let max_svg_size = 2000.0_f64;
    let mut scale = 1.0_f64;

    if width as f64 > max_svg_size || height as f64 > max_svg_size {
        scale = (max_svg_size / width as f64).min(max_svg_size / height as f64);
    }

    let svg_width = width as f64 * scale;
    let svg_height = height as f64 * scale;

    let mut svg = File::create(filename)?;

    writeln!(svg, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
    write!(svg, "<svg xmlns=\"http://www.w3.org/2000/svg\" ")?;
    write!(
        svg,
        "width=\"{}\" height=\"{}\" ",
        svg_width + 40.0,
        svg_height + 40.0
    )?;
    writeln!(
        svg,
        "viewBox=\"{} {} {} {}\">",
        min_x as f64 - 20.0 / scale,
        min_y as f64 - 20.0 / scale,
        width as f64 + 40.0 / scale,
        height as f64 + 40.0 / scale
    )?;

    // White background
    writeln!(
        svg,
        "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"white\"/>",
        min_x, min_y, width, height
    )?;

    // Draw border as a polygon
    write!(svg, "  <polygon points=\"")?;
    for (i, &(x, y)) in coordinates.iter().enumerate() {
        write!(svg, "{},{}", x, y)?;
        if i < coordinates.len() - 1 {
            write!(svg, " ")?;
        }
    }
    writeln!(
        svg,
        "\" fill=\"none\" stroke=\"green\" stroke-width=\"{}\"/>",
        2.0 / scale
    )?;

    // Draw corner vertices
    writeln!(svg, "  <g id=\"vertices\" fill=\"red\">")?;
    for &(x, y) in coordinates {
        writeln!(
            svg,
            "    <circle cx=\"{}\" cy=\"{}\" r=\"{}\"/>",
            x,
            y,
            3.0 / scale
        )?;
    }
    writeln!(svg, "  </g>")?;

    writeln!(svg, "</svg>")?;

    println!("SVG written to {}", filename);
    Ok(())
}

/*
--- Part Two ---

The Elves just remembered: they can only switch out tiles that are red or green. So, your rectangle can only include red or green tiles.

In your list, every red tile is connected to the red tile before and after it by a straight line of green tiles. The list wraps, so the first red tile is also connected to the last red tile. Tiles that are adjacent in your list will always be on either the same row or the same column.

Using the same example as before, the tiles marked X would be green:

..............
.......#XXX#..
.......X...X..
..#XXXX#...X..
..X........X..
..#XXXXXX#.X..
.........X.X..
.........#X#..
..............

In addition, all of the tiles inside this loop of red and green tiles are also green. So, in this example, these are the green tiles:

..............
.......#XXX#..
.......XXXXX..
..#XXXX#XXXX..
..XXXXXXXXXX..
..#XXXXXX#XX..
.........XXX..
.........#X#..
..............

The remaining tiles are never red nor green.

The rectangle you choose still must have red tiles in opposite corners, but any other tiles it includes must now be red or green. This significantly limits your options.

For example, you could make a rectangle out of red and green tiles with an area of 15 between 7,3 and 11,1:

..............
.......OOOOO..
.......OOOOO..
..#XXXXOOOOO..
..XXXXXXXXXX..
..#XXXXXX#XX..
.........XXX..
.........#X#..
..............

Or, you could make a thin rectangle with an area of 3 between 9,7 and 9,5:

..............
.......#XXX#..
.......XXXXX..
..#XXXX#XXXX..
..XXXXXXXXXX..
..#XXXXXXOXX..
.........OXX..
.........OX#..
..............

The largest rectangle you can make in this example using only red and green tiles has area 24. One way to do this is between 9,5 and 2,3:

..............
.......#XXX#..
.......XXXXX..
..OOOOOOOOXX..
..OOOOOOOOXX..
..OOOOOOOOXX..
.........XXX..
.........#X#..
..............

Using two red tiles as opposite corners, what is the largest area of any rectangle you can make using only red and green tiles?
*/
