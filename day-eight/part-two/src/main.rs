use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Vertex {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

type Edge = (Vertex, Vertex);

impl Vertex {
    fn distance_from(self, p: Vertex) -> f64 {
        let dx = self.x as f64 - p.x as f64;
        let dy = self.y as f64 - p.y as f64;
        let dz = self.z as f64 - p.z as f64;
        dx * dx + dy * dy + dz * dz
    }
}

// kruskal's
// https://www.geeksforgeeks.org/dsa/kruskals-minimum-spanning-tree-algorithm-greedy-algo-2/
// https://en.wikipedia.org/wiki/Kruskal's_algorithm
fn find(parent: &mut [usize], i: usize) -> usize {
    if parent[i] != i {
        let root = find(parent, parent[i]);
        parent[i] = root;
    }
    parent[i]
}

fn union(parent: &mut [usize], rank: &mut [usize], a: usize, b: usize) -> bool {
    let mut ra = find(parent, a);
    let mut rb = find(parent, b);
    if ra == rb {
        return false;
    }

    if rank[ra] < rank[rb] {
        std::mem::swap(&mut ra, &mut rb);
    }

    parent[rb] = ra;
    if rank[ra] == rank[rb] {
        rank[ra] += 1;
    }
    true
}

fn edges_weighted_by_distance(points: &[Vertex]) -> Vec<Edge> {
    let mut items: Vec<_> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let distance = points[i].distance_from(points[j]);
            let _ = items.push(((points[i], points[j]), distance));
        }
    }
    items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    items.iter().map(|(j, _)| *j).collect()
}

fn calculate_last_mst_forest_edge(edges: &[Edge]) -> Edge {
    let mut id: HashMap<Vertex, usize> = HashMap::new();
    let mut next = 0usize;

    for &(u, v) in edges {
        for vert in [u, v] {
            id.entry(vert).or_insert_with(|| {
                let idx = next;
                next += 1;
                idx
            });
        }
    }

    let mut parent: Vec<usize> = (0..next).collect();
    let mut rank = vec![0; next];

    let mut used = 0usize;

    for &(u, v) in edges {
        if union(&mut parent, &mut rank, id[&u], id[&v]) {
            used += 1;
            if used == next - 1 {
                return (u, v);
            }
        }
    }
    unreachable!();
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
            let z = split.next().unwrap().parse().unwrap();
            Vertex { x, y, z }
        })
        .collect();

    Ok(points)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).expect("usage: aoc8pt2 <input-file>");
    let vertices: Vec<Vertex> = read_file(&path)?;
    let edges = edges_weighted_by_distance(&vertices);
    let last_edge = calculate_last_mst_forest_edge(&edges);
    let total = last_edge.1.x * last_edge.0.x;
    println!("final = {total}");
    Ok(())
}

/*
--- Part Two ---

The Elves were right; they definitely don't have enough extension cables. You'll need to keep connecting junction boxes together until they're all in one large circuit.

Continuing the above example, the first connection which causes all of the junction boxes to form a single circuit is between the junction boxes at 216,146,977 and 117,168,530. The Elves need to know how far those junction boxes are from the wall so they can pick the right extension cable; multiplying the X coordinates of those two junction boxes (216 and 117) produces 25272.

Continue connecting the closest unconnected pairs of junction boxes together until they're all in the same circuit. What do you get if you multiply together the X coordinates of the last two junction boxes you need to connect?
*/
