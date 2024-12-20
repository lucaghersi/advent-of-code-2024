use adv_code_2024::start_day;
use anyhow::{Result};
use code_timing_macros::time_snippet;
use colored::Colorize;
use const_format::concatcp;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;

const DAY: &str = "20";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

#[tokio::main]
async fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    async fn calc(input: &str, max_distance: usize, limit: usize) -> Result<u32> {
        let grid = parse(input);
        let savings = grid.cheat(max_distance, limit).await;
        Ok(savings)
    }

    let result = calc(TEST, 2, 0).await?;
    println!("Test Result 1 = {}", result);
    assert_eq!(44, result);

    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);

    let result = time_snippet!(calc(&input, 2, 100).await?;);
    println!("Result 1 = {}", result);
    assert_eq!(1311, result);

    println!("=== Part 2 ===");

    let result = calc(TEST, 20, 50).await?;
    println!("Test Result 2 = {}", result);
    assert_eq!(285, result);

    let result = time_snippet!(calc(&input, 20, 100).await?;);
    println!("Result 2 = {}", result);
    assert_eq!(961364, result);

    anyhow::Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    position: Point,
    cost: u32,     // Cumulative cost
    priority: u32, // Cost + heuristic
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Point {
    c: usize,
    r: usize,
    v: u32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({},{})", self.r, self.c)
    }
}

impl Point {
    fn get_successors(&self, grid: &Grid, cheat_point: Option<&Point>) -> Vec<Point> {
        let mut successors: Vec<Point> = Vec::new();

        for dr in -1i32..=1 {
            for dc in -1i32..=1 {
                // Omit diagonal moves (and moving to the same position)
                if (dr + dc).abs() != 1 {
                    continue;
                }

                let new_r = self.r as i32 + dr;
                let new_c = self.c as i32 + dc;
                if new_r < 0 || new_c < 0 {
                    continue;
                };

                if let Some(mut point) = grid.get(new_r as usize, new_c as usize) {
                    let mut cheat_here: bool = false;
                    if let Some(cheat_point) = cheat_point {
                        cheat_here = point.c == cheat_point.c && point.r == cheat_point.r;
                    }

                    if point.v > 1 && !cheat_here {
                        // impassable
                        continue;
                    }

                    point.v = 1;
                    successors.push(point);
                }
            }
        }
        successors
    }
}

struct Grid {
    map: Vec<Vec<u32>>,
    start_r: usize,
    start_c: usize,
    end_r: usize,
    end_c: usize,
}

impl Grid {
    fn new(size: usize) -> Self {
        let mut grid: Vec<Vec<u32>> = Vec::with_capacity(size);
        for _ in 0..size {
            grid.push(vec![1; size])
        }

        Self {
            map: grid,
            start_r: 0,
            start_c: 0,
            end_r: 0,
            end_c: 0,
        }
    }

    fn get(&self, r: usize, c: usize) -> Option<Point> {
        if let Some(row) = self.map.get(r) {
            if let Some(&v) = row.get(c) {
                return Some(Point { r, c, v });
            }
        }

        None
    }

    fn get_value(&self, pos: Point) -> Option<&u32> {
        if let Some(row) = self.map.get(pos.r) {
            return row.get(pos.c);
        }

        None
    }

    fn get_value_mut(&mut self, r: usize, c: usize) -> Option<&mut u32> {
        if let Some(row) = self.map.get_mut(r) {
            return row.get_mut(c);
        }

        None
    }

    fn set_value(&mut self, r: usize, c: usize, value: u32) -> Option<bool> {
        if let Some(position) = self.get_value_mut(r, c) {
            *position = value;
            return Some(true);
        }

        Some(false)
    }

    fn print(&self) {
        self.print_with_path(&[]);
    }

    fn print_with_path(&self, path: &[Point]) {
        for r in 0..self.map.len() {
            for c in 0..self.map[r].len() {
                let point = Point { r, c, v: 1 };
                let value = *self.get_value(point).unwrap();

                if point == self.get_start() {
                    print!("{}", "S".red());
                } else if point == self.get_goal() {
                    print!("{}", "E".green());
                } else if path.contains(&point) {
                    print!("{}", "P".bright_blue());
                } else if value <= 1 {
                    print!("{}", ".".bright_yellow());
                } else {
                    print!("{}", "#".bright_black());
                }
            }
            println!()
        }
    }

    fn get_start(&self) -> Point {
        Point {
            r: self.start_r,
            c: self.start_c,
            v: 1,
        }
    }

    fn get_goal(&self) -> Point {
        Point {
            r: self.end_r,
            c: self.end_c,
            v: 1,
        }
    }

    fn heuristic(&self, a: &Point, b: &Point) -> u32 {
        (a.r.abs_diff(b.r) + a.c.abs_diff(b.c)) as u32
    }

    async fn cheat(&self, max_distance: usize, limit: usize) -> u32 {
        let mut savings: HashMap<usize, u32> = HashMap::new();
        let honest_path = self.a_star_search(None).await.unwrap();

        for sp_i in 0..honest_path.0.len() {
            let sp = honest_path.0[sp_i];
            let ep_starting_point = sp_i;
            for ep_i in ep_starting_point..honest_path.0.len() {
                let ep = honest_path.0[ep_i];

                let cheating_distance = self.heuristic(&sp, &ep) as usize;
                if cheating_distance > max_distance {
                    continue;
                }

                let honest_distance = ep_i - sp_i;
                if honest_distance <= cheating_distance {
                    continue;
                }

                let saving_on_honest_path = honest_distance - cheating_distance;
                if saving_on_honest_path < limit {
                    continue;
                }

                if let Some(current_savings) = savings.get_mut(&saving_on_honest_path) {
                    *current_savings += 1;
                } else {
                    savings.insert(saving_on_honest_path, 1);
                }
            }
        }

        for k in savings.keys().sorted() {
            println!(
                "There are {} cheats that save {} picoseconds",
                savings[k], k
            )
        }

        savings.values().sum()
    }
    
    async fn a_star_search(&self, cheat_point: Option<&Point>) -> Option<(Vec<Point>, u32)> {
        let mut open_set = BinaryHeap::new();
        open_set.push(Node {
            position: self.get_start(),
            cost: 0,
            priority: self.heuristic(&self.get_start(), &self.get_goal()),
        });

        let mut came_from: HashMap<Point, Point> = HashMap::new();
        let mut cost_so_far: HashMap<Point, u32> = HashMap::new();

        came_from.insert(self.get_start(), self.get_start());
        cost_so_far.insert(self.get_start(), self.get_start().v);

        while let Some(current_node) = open_set.pop() {
            let current = current_node.position;

            if current == self.get_goal() {
                let total_cost = cost_so_far[&current];

                let mut path = Vec::new();
                let mut current_state = current;

                while current_state != self.get_start() {
                    path.push(current_state);
                    current_state = came_from[&current_state];
                }
                path.push(self.get_start());
                path.reverse();

                return Some((path, total_cost));
            }

            let successors = &current.get_successors(self, cheat_point);
            for successor_position in successors {
                let new_cost = cost_so_far[&current] + successor_position.v;

                if !cost_so_far.contains_key(successor_position)
                    || new_cost < cost_so_far[successor_position]
                {
                    cost_so_far.insert(*successor_position, new_cost);
                    let priority = new_cost + self.heuristic(successor_position, &self.get_goal());
                    open_set.push(Node {
                        position: *successor_position,
                        cost: new_cost,
                        priority,
                    });
                    came_from.insert(*successor_position, current);
                }
            }
        }

        None // No path found
    }
}

#[allow(unused_assignments)]
fn parse(input: &str) -> Grid {
    let mut grid = Grid::new(input.lines().count());

    for (r, line) in input.lines().enumerate() {
        for (c, point) in line.chars().enumerate() {
            if point == 'S' {
                grid.start_c = c;
                grid.start_r = r;
            } else if point == 'E' {
                grid.end_c = c;
                grid.end_r = r;
            } else if point == '.' {
                grid.set_value(r, c, 1);
            } else {
                grid.set_value(r, c, 2);
            }
        }
    }

    //grid.print();

    grid
}
