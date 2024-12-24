use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use petgraph::algo;
use petgraph::data::Build;
use petgraph::graphmap::NodeTrait;
use petgraph::prelude::*;
use petgraph::visit::{GetAdjacencyMatrix, IntoNodeIdentifiers, NodeRef};
use std::collections::{HashMap, HashSet};
use std::fmt::{format, Debug};
use std::fs::File;
use std::io::Read;
use std::ops::{BitXor, Index};

const DAY: &str = "23";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<usize> {
        let total = mix(input);
        Ok(total)
    }

    fn part2(input: &str) -> Result<u64> {
        biggest_clique(input);
        Ok(0)
    }

    let result = part1(TEST)?;
    println!("Test Result 1 = {}", result);
    assert_eq!(7, result);

    let result = part2(TEST)?;
    println!("Test Result 2 = {}", result);
    //assert_eq!(23, result);
    //
    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);

    // let result = time_snippet!(part1(&input)?);
    // println!("Result 1 = {}", result);
    // assert_eq!(16999668565, result);

    let _ = time_snippet!(part2(&input)?);
    // println!("Result 2 = {}", result);
    // assert_eq!(1898, result);

    Ok(())
}

fn mix(input: &str) -> usize {
    let mut tokens: HashSet<String> = HashSet::new();
    let mut graph: Graph<&str, i32, Undirected> = Graph::new_undirected();
    let mut node_index: HashMap<String, NodeIndex> = HashMap::new();

    for ns in input.lines().collect_vec() {
        let split = ns.split('-').collect_vec();

        let node_1 = split[0];
        let node_2 = split[1];

        tokens.insert(node_1.to_string());
        tokens.insert(node_2.to_string());

        if !node_index.contains_key(node_1) {
            let n1 = graph.add_node(node_1);
            node_index.insert(node_1.to_string(), n1);
        }

        if !node_index.contains_key(node_2) {
            let n2 = graph.add_node(node_2);
            node_index.insert(node_2.to_string(), n2);
        }

        graph.update_edge(
            node_index[&node_1.to_string()],
            node_index[&node_2.to_string()],
            1,
        );
    }

    let mut total = 0;
    let adj_matrix = graph.adjacency_matrix();

    let combinations = tokens.iter().combinations(3).collect_vec();
    for nodes in combinations.as_slice() {
        let n1_t = nodes[0];
        let n2_t = nodes[1];
        let n3_t = nodes[2];

        if n1_t == n2_t || n2_t == n3_t || n1_t == n3_t {
            continue;
        }

        if !n1_t.starts_with("t") && !n2_t.starts_with("t") && !n3_t.starts_with("t") {
            continue;
        }

        let n1 = node_index[nodes[0]];
        let n2 = node_index[nodes[1]];
        let n3 = node_index[nodes[2]];

        if graph.is_adjacent(&adj_matrix, n1, n2)
            && graph.is_adjacent(&adj_matrix, n2, n3)
            && graph.is_adjacent(&adj_matrix, n1, n3)
        {
            //println!("{}-{}-{}", &nodes[0], &nodes[1], &nodes[2]);
            total += 1;
        }
    }

    total
}

fn biggest_clique(input: &str) {
    let mut graph: GraphMap<&str, i32, Undirected> = GraphMap::new();
    let mut node_index: HashSet<String> = HashSet::new();

    for ns in input.lines().collect_vec() {
        let split = ns.split('-').collect_vec();

        let node_1 = split[0];
        let node_2 = split[1];

        if !node_index.contains(node_1) {
            graph.add_node(node_1);
            node_index.insert(node_1.to_string());
        }

        if !node_index.contains(node_2) {
            graph.add_node(node_2);
            node_index.insert(node_2.to_string());
        }

        graph.update_edge(node_1, node_2, 1);
    }

    let mut bk = BronKerbosch::new(graph);
    bk.compute();

    for c in bk
        .max_cliques
        .iter()
        .sorted_by(|a, b| Ord::cmp(&b.len(), &a.len()))
    {
        let c = c.iter().copied().sorted().collect_vec().join(",");
        println!("{}", c);
    }
}

struct BronKerbosch<N: NodeTrait, E> {
    graph: GraphMap<N, E, Undirected>,
    max_cliques: Vec<HashSet<N>>,
}

impl<N: NodeTrait, E> BronKerbosch<N, E> {
    pub fn new(graphmap: GraphMap<N, E, Undirected>) -> BronKerbosch<N, E> {
        BronKerbosch {
            graph: graphmap,
            max_cliques: Vec::new(),
        }
    }

    pub fn compute(&mut self) {
        let p = self.graph.nodes().collect::<HashSet<N>>();
        let r = HashSet::new();
        let x = HashSet::new();
        self.bronkerbosch(p, r, x);
    }

    pub fn cliques(&self) -> &Vec<HashSet<N>> {
        &self.max_cliques
    }

    fn bronkerbosch(&mut self, p: HashSet<N>, r: HashSet<N>, x: HashSet<N>) {
        let mut p_fp = p.clone();
        let mut x_fp = x.clone();

        if p.is_empty() {
            if x.is_empty() {
                self.max_cliques.push(r.clone());
            }
            return;
        }

        for v in p.iter() {
            let v_neighbours = self.graph.neighbors(*v).collect::<HashSet<N>>();

            let p_intersect_v_neighbors = p_fp.intersection(&v_neighbours).cloned().collect();
            let mut r_union_v = r.clone();
            r_union_v.insert(*v);
            let x_intersect_v_neighbors = x_fp.intersection(&v_neighbours).cloned().collect();

            self.bronkerbosch(p_intersect_v_neighbors, r_union_v, x_intersect_v_neighbors);

            p_fp.remove(v);
            x_fp.insert(*v);
        }
    }
}
