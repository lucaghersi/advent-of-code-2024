use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::Read;
use tokio::task::JoinSet;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

#[tokio::main]
async fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    async fn part1(input: &str) -> Result<usize> {
        let map = parse(input)?;
        let path = navigate_grid(map, false, 0).await?;
        Ok(path.points.len())
    }

    async fn part2(input: &str) -> Result<usize> {
        let mut map = parse(input)?;
        let cycles = test_obstacles(&mut map).await?;
        Ok(cycles)
    }

    let result = part1(TEST).await?;
    println!("Test Result 1 = {}", result);
    assert_eq!(41, result);

    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);

    let result = time_snippet!(part1(&input).await?);
    println!("Result 1 = {}", result);

    println!("=== Part 2 ===");

    let result = part2(TEST).await?;
    println!("Test Result 2 = {}", result);
    assert_eq!(6, result);

    let result = time_snippet!(part2(&input).await?;);
    println!("Result 2 = {}", result);

    Ok(())
}

#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Point {
    x: usize,
    y: usize,
    direction: Direction,
}

struct Position {
    next_will_be_outside: bool,
    next_position_x: usize,
    next_position_y: usize,
    moved: bool,
}

struct Path {
    points: Vec<Point>,
    cycle: bool,
}

async fn test_obstacles(map: &Vec<Vec<char>>) -> Result<usize> {
    let guard = find_guard(map)?;
    let mut exclusion = Point {
        x: guard.x,
        y: guard.y,
        direction: guard.direction.clone(),
    };

    match guard.direction {
        Direction::Up => {
            exclusion.x = guard.x - 1;
        }
        Direction::Down => {
            exclusion.x = guard.x + 1;
        }
        Direction::Left => {
            exclusion.y = guard.y - 1;
        }
        Direction::Right => {
            exclusion.y = guard.y + 1;
        }
    }

    let mut set = JoinSet::new();

    let mut current_map = 0;

    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if guard.x == r && guard.y == c {
                continue;
            }
            if exclusion.x == r && exclusion.y == c {
                continue;
            }

            let tile = map[r][c];
            if tile != '.' {
                continue;
            }

            let mut map = map.clone();
            map[r][c] = '#';

            current_map += 1;

            set.spawn(navigate_grid(map, true, current_map));
        }
    }

    let output = set.join_all().await;

    let mut total_cycles = 0;
    for path in output {
        let path = path?;
        if path.cycle {
            total_cycles += 1;
        }
    }

    Ok(total_cycles)
}

async fn navigate_grid(map: Vec<Vec<char>>, break_on_cycle: bool, map_name: usize) -> Result<Path> {
    let mut guard = find_guard(&map)?;
    let mut points: Vec<Point> = Vec::new();
    points.push(Point {
        x: guard.x,
        y: guard.y,
        direction: guard.direction.clone(),
    });

    let print = |text: String| {
        if map_name % 100 == 0 {
            println!("{}", text)
        }
    };

    loop {
        let position = move_to_next_position(&map, &guard).await?;
        if position.next_will_be_outside {
            break;
        }

        if position.moved {
            guard.y = position.next_position_y;
            guard.x = position.next_position_x;

            let find_result = points.iter_mut().find(|p| p.x == guard.x && p.y == guard.y);

            if find_result.is_some() {
                let point: &mut Point = find_result.unwrap();
                if break_on_cycle && point.direction == guard.direction {
                    print(format!("Map {map_name} is a cycle."));
                    return Ok(Path {
                        cycle: true,
                        points,
                    });
                }
                point.direction = guard.direction.clone();
            } else {
                points.push(Point {
                    x: guard.x,
                    y: guard.y,
                    direction: guard.direction.clone(),
                });
            }
        } else {
            match guard.direction {
                Direction::Up => {
                    guard.direction = Direction::Right;
                }
                Direction::Down => {
                    guard.direction = Direction::Left;
                }
                Direction::Left => {
                    guard.direction = Direction::Up;
                }
                Direction::Right => {
                    guard.direction = Direction::Down;
                }
            };
        }
    }

    print(format!("Map {map_name} completed successfully."));
    Ok(Path {
        cycle: false,
        points,
    })
}

async fn move_to_next_position(map: &[Vec<char>], guard: &Point) -> Result<Position> {
    let current_pos_x = guard.x;
    let current_pos_y = guard.y;

    let next_will_be_outside = || -> Result<Position> {
        Ok(Position {
            next_will_be_outside: true,
            next_position_x: current_pos_x,
            next_position_y: current_pos_y,
            moved: true,
        })
    };

    // check for borders
    let mut next_pos_x = guard.x;
    let mut next_pos_y = guard.y;

    match guard.direction {
        Direction::Up => {
            if current_pos_x == 0 {
                return next_will_be_outside();
            }
            next_pos_x -= 1;
        }
        Direction::Down => {
            if current_pos_x == map.len() - 1 {
                return next_will_be_outside();
            }
            next_pos_x += 1;
        }
        Direction::Left => {
            if current_pos_y == 0 {
                return next_will_be_outside();
            }
            next_pos_y -= 1;
        }
        Direction::Right => {
            if current_pos_y == map[current_pos_x].len() - 1 {
                return next_will_be_outside();
            }
            next_pos_y += 1;
        }
    }

    // check if next position is valid
    let next_space = map[next_pos_x][next_pos_y];
    let next_is_empty = next_space == '.' || next_space == '^';

    Ok(Position {
        next_will_be_outside: false,
        next_position_x: if next_is_empty {
            next_pos_x
        } else {
            current_pos_x
        },
        next_position_y: if next_is_empty {
            next_pos_y
        } else {
            current_pos_y
        },
        moved: next_is_empty,
    })
}

fn find_guard(map: &[Vec<char>]) -> Result<Point> {
    let guard = |x: usize, y: usize, direction: Direction| Point { x, y, direction };

    for pos_x in 0..map.len() {
        for pos_y in 0..map[pos_x].len() {
            if map[pos_x][pos_y] == '^' {
                return Ok(guard(pos_x, pos_y, Direction::Up));
            }
            if map[pos_x][pos_y] == '<' {
                return Ok(guard(pos_x, pos_y, Direction::Left));
            }
            if map[pos_x][pos_y] == '>' {
                return Ok(guard(pos_x, pos_y, Direction::Right));
            }
            if map[pos_x][pos_y] == 'v' {
                return Ok(guard(pos_x, pos_y, Direction::Down));
            }
        }
    }

    Err(anyhow!("This is not expected!"))
}

fn parse(input: &str) -> Result<Vec<Vec<char>>> {
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let chars = line.chars().collect_vec();
        map.push(chars);
    }

    Ok(map)
}
