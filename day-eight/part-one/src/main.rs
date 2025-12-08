use std::{
    collections::HashMap,
    collections::HashSet,
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

fn n_shortest_edges<const N: usize>(points: &[Vertex]) -> Vec<Edge> {
    let mut items: Vec<_> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let distance = points[i].distance_from(points[j]);
            let _ = items.push(((points[i], points[j]), distance));
        }
    }
    items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    items.truncate(N.min(items.len()));
    items.iter().map(|(j, _)| *j).collect()
}

fn create_mst_forest(edges: &[Edge]) -> Vec<Vec<Edge>> {
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

    let mut chosen: Vec<Edge> = Vec::new();

    for &(u, v) in edges {
        if union(&mut parent, &mut rank, id[&u], id[&v]) {
            chosen.push((u, v));
        }
    }

    let mut forest: HashMap<usize, Vec<Edge>> = HashMap::new();
    for (u, v) in chosen {
        let root = find(&mut parent, id[&u]);
        forest.entry(root).or_default().push((u, v));
    }

    forest.into_values().collect()
}

fn count_vertices_in_tree(tree: &[Edge]) -> usize {
    let mut vertices: HashSet<Vertex> = HashSet::new();
    for &(u, v) in tree {
        vertices.insert(u);
        vertices.insert(v);
    }
    vertices.len()
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
    let path = env::args().nth(1).expect("usage: aoc8pt1 <input-file>");
    let vertices: Vec<Vertex> = read_file(&path)?;
    let edges = n_shortest_edges::<1000>(&vertices);
    let mut forest = create_mst_forest(&edges);
    forest.sort_by_key(|tree| std::cmp::Reverse(count_vertices_in_tree(tree)));
    forest.truncate(3);
    let total = forest
        .iter()
        .fold(1, |acc, tree| acc * count_vertices_in_tree(tree));
    println!("final = {total}");
    Ok(())
}

/*
--- Day 8: Playground ---

Equipped with a new understanding of teleporter maintenance, you confidently step onto the repaired teleporter pad.

You rematerialize on an unfamiliar teleporter pad and find yourself in a vast underground space which contains a giant playground!

Across the playground, a group of Elves are working on setting up an ambitious Christmas decoration project. Through careful rigging, they have suspended a large number of small electrical junction boxes.

Their plan is to connect the junction boxes with long strings of lights. Most of the junction boxes don't provide electricity; however, when two junction boxes are connected by a string of lights, electricity can pass between those two junction boxes.

The Elves are trying to figure out which junction boxes to connect so that electricity can reach every junction box. They even have a list of all of the junction boxes' positions in 3D space (your puzzle input).

For example:

162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689

This list describes the position of 20 junction boxes, one per line. Each position is given as X,Y,Z coordinates. So, the first junction box in the list is at X=162, Y=817, Z=812.

To save on string lights, the Elves would like to focus on connecting pairs of junction boxes that are as close together as possible according to straight-line distance. In this example, the two junction boxes which are closest together are 162,817,812 and 425,690,689.

By connecting these two junction boxes together, because electricity can flow between them, they become part of the same circuit. After connecting them, there is a single circuit which contains two junction boxes, and the remaining 18 junction boxes remain in their own individual circuits.

Now, the two junction boxes which are closest together but aren't already directly connected are 162,817,812 and 431,825,988. After connecting them, since 162,817,812 is already connected to another junction box, there is now a single circuit which contains three junction boxes and an additional 17 circuits which contain one junction box each.

The next two junction boxes to connect are 906,360,560 and 805,96,715. After connecting them, there is a circuit containing 3 junction boxes, a circuit containing 2 junction boxes, and 15 circuits which contain one junction box each.

The next two junction boxes are 431,825,988 and 425,690,689. Because these two junction boxes were already in the same circuit, nothing happens!

This process continues for a while, and the Elves are concerned that they don't have enough extension cables for all these circuits. They would like to know how big the circuits will be.

After making the ten shortest connections, there are 11 circuits: one circuit which contains 5 junction boxes, one circuit which contains 4 junction boxes, two circuits which contain 2 junction boxes each, and seven circuits which each contain a single junction box. Multiplying together the sizes of the three largest circuits (5, 4, and one of the circuits of size 2) produces 40.

Your list contains many junction boxes; connect together the 1000 pairs of junction boxes which are closest together. Afterward, what do you get if you multiply together the sizes of the three largest circuits?
*/
