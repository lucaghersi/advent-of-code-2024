use adv_code_2024::start_day;
use anyhow::{anyhow, Result};
use code_timing_macros::time_snippet;
use colored::Colorize;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::ops::BitXor;

const DAY: &str = "18";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    async fn part1(input: &str, size: usize, limit: usize) -> Result<u32> {
        let grid = parse(input, size, limit);
        let path = grid.a_star_search().unwrap();
        //grid.print_with_path(&path.0);
        Ok(path.1 - 1)
    }

    async fn part2(input: &str, size: usize, limit: usize) -> Result<String> {
        for i in limit..10000000 {
            let grid = parse(input, size, i);
            let path = grid.a_star_search();
            if path.is_none() {
                return Ok(input.lines().collect_vec()[i - 1].to_string());
            }
        }

        Err(anyhow!("Error"))
    }

    let result = part1(TEST, 6, 12).await?;
    println!("Test Result 1 = {}", result);
    assert_eq!(22, result);

    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);

    let result = time_snippet!(part1(&input, 70, 1024).await?;);
    println!("Result 1 = {}", result);
    assert_eq!(356, result);

    println!("=== Part 2 ===");

    let result = part2(TEST, 6, 12).await?;
    println!("Test Result 2 = {}", result);
    assert_eq!("6,1", result);

    let result = time_snippet!(part2(&input, 70, 1024).await?;);
    println!("Result 2 = {}", result);
    assert_eq!("22,33", result);

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
    fn get_successors(&self, grid: &Grid) -> Vec<Point> {
        let mut successors: Vec<Point> = Vec::new();

        for dr in -1i8..=1 {
            for dc in -1i8..=1 {
                // Omit diagonal moves (and moving to the same position)
                if (dr + dc).abs() != 1 {
                    continue;
                }

                let new_r = self.r as i8 + dr;
                let new_c = self.c as i8 + dc;
                if new_r < 0 || new_c < 0 {
                    continue;
                };

                if let Some(point) = grid.get(new_r as usize, new_c as usize) {
                    if point.v > 1 {
                        continue;
                    }

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
        let mut grid: Vec<Vec<u32>> = Vec::with_capacity(size + 1);
        for _ in 0..=size {
            grid.push(vec![1; size + 1])
        }

        Self {
            map: grid,
            start_r: 0,
            start_c: 0,
            end_r: size,
            end_c: size,
        }
    }

    fn get_value(&self, pos: Point) -> Option<&u32> {
        if let Some(row) = self.map.get(pos.r) {
            return row.get(pos.c);
        }

        None
    }

    fn get(&self, r: usize, c: usize) -> Option<Point> {
        if let Some(row) = self.map.get(r) {
            if let Some(&v) = row.get(c) {
                return Some(Point { r, c, v });
            }
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
                let value = self.get_value(point).unwrap().to_string();
                if path.contains(&point) {
                    if point == self.get_start() {
                        print!("{}", "P".red());
                    } else if point == self.get_goal() {
                        print!("{}", "P".green());
                    } else {
                        print!("{}", "P".bright_blue());
                    }
                } else if point == self.get_start() {
                    print!("{}", value.red());
                } else if point == self.get_goal() {
                    print!("{}", value.green());
                } else if value == "0" {
                    print!("{}", value.bright_yellow());
                } else {
                    print!("{}", value.bright_black());
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

    fn a_star_search(&self) -> Option<(Vec<Point>, u32)> {
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

            let successors = &current.get_successors(self);
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
fn parse(input: &str, size: usize, limit: usize) -> Grid {
    let mut grid = Grid::new(size);

    for (i, line) in input.lines().enumerate() {
        if i == limit {
            break;
        }

        let coordinates = line.split(',').collect_vec();
        let c = coordinates[0].parse::<usize>().unwrap();
        let r = coordinates[1].parse::<usize>().unwrap();
        let set_result = grid.set_value(r, c, 2);
        if !set_result.unwrap() {
            panic!();
        }
    }

    //grid.print();

    grid
}
