use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use tokio::task::JoinSet;

const DAY: &str = "15";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

const TEST2: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

const TEST3: &str = "\
#######
#...#.#
#.....#
#..OO.#
#..O..#
#..@..#
#######

^
";

#[tokio::main]
async fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    async fn part1(input: &str) -> Result<usize> {
        let mut warehouse = parse(input)?;
        move_bot_around_warehouse(&mut warehouse).await;
        let gps = gps(&warehouse);
        Ok(gps)
    }

    async fn part2(input: &str) -> Result<usize> {
        let mut warehouse = parse(input)?;
        warehouse = enlarge(warehouse);
        print_map(&warehouse);
        move_bot_around_warehouse(&mut warehouse).await;
        let gps = gps(&warehouse);
        Ok(gps)
    }

    // let result = part1(TEST).await?;
    // println!("Test Result 1.1 = {}", result);
    // assert_eq!(2028, result);
    //
    // let result = part1(TEST2).await?;
    // println!("Test Result 1.2 = {}", result);
    // assert_eq!(10092, result);
    //
    // let mut input_file = File::open(INPUT_FILE)?;
    // let mut buffer = Vec::new();
    // input_file.read_to_end(&mut buffer)?;
    // let input = String::from_utf8_lossy(&buffer);
    //
    // let result = time_snippet!(part1(&input).await?);
    // println!("Result 1 = {}", result);
    // assert_eq!(1413675, result);

    println!("=== Part 2 ===");

    let result = part2(TEST3).await?;
    println!("Test Result 2.1 = {}", result);

    // let result = part2(TEST2).await?;
    // println!("Test Result 2.2 = {}", result);
    //assert_eq!(9021, result);
    //
    // let result = time_snippet!(part2(&input).await?);
    // println!("Result 2 = {}", result);

    Ok(())
}

#[derive(Debug)]
struct Warehouse {
    map: Vec<Vec<char>>,
    bot_initial_row: usize,
    bot_initial_column: usize,
    movements: Vec<char>,
}

async fn move_bot_around_warehouse(warehouse: &mut Warehouse) {
    let mut row = warehouse.bot_initial_row;
    let mut column = warehouse.bot_initial_column;

    for movement in warehouse.movements.clone() {
        print_map(warehouse);
        println!("Move {}", movement);
        match movement {
            '<' => {
                if is_wall(warehouse, row, column - 1) {
                    continue;
                }
                if is_free(warehouse, row, column, movement)
                    || move_box_pile(warehouse, '<', row, column - 1)
                {
                    warehouse.map[row][column] = '.';
                    column -= 1;
                    warehouse.map[row][column] = '@';
                    continue;
                }
            }
            '^' => {
                if is_wall(warehouse, row - 1, column) {
                    continue;
                }
                if is_free(warehouse, row, column, movement)
                    || move_box_pile(warehouse, '^', row - 1, column)
                {
                    warehouse.map[row][column] = '.';
                    row -= 1;
                    warehouse.map[row][column] = '@';
                    continue;
                }
            }
            'v' => {
                if is_wall(warehouse, row + 1, column) {
                    continue;
                }
                if is_free(warehouse, row, column, movement)
                    || move_box_pile(warehouse, 'v', row + 1, column)
                {
                    warehouse.map[row][column] = '.';
                    row += 1;
                    warehouse.map[row][column] = '@';
                    continue;
                }
            }
            '>' => {
                if is_wall(warehouse, row, column + 1) {
                    continue;
                }
                if is_free(warehouse, row, column, movement)
                    || move_box_pile(warehouse, '>', row, column + 1)
                {
                    warehouse.map[row][column] = '.';
                    column += 1;
                    warehouse.map[row][column] = '@';
                    continue;
                }
            }
            _ => panic!("This character is unexpected on the map!"),
        }
    }

    print_map(warehouse);
}

fn move_box_pile(warehouse: &mut Warehouse, direction: char, row: usize, column: usize) -> bool {
    match direction {
        '<' => {
            if is_wall(warehouse, row, column - 1) {
                return false;
            }

            if can_box_be_moved(warehouse, row, column, direction)
                || move_box_pile(warehouse, direction, row, column - 1)
            {
                if warehouse.map[row][column] == 'O' {
                    warehouse.map[row][column - 1] = 'O';
                    warehouse.map[row][column] = '.';
                } else if warehouse.map[row][column] == ']' {
                    warehouse.map[row][column - 2] = '[';
                    warehouse.map[row][column - 1] = ']';
                    warehouse.map[row][column] = '.';
                }

                return true;
            }
        }
        '^' => {
            if is_wall(warehouse, row - 1, column) {
                return false;
            }

            if warehouse.map[row][column] == 'O'
                && (can_box_be_moved(warehouse, row, column, direction)
                    || move_box_pile(warehouse, direction, row - 1, column))
            {
                warehouse.map[row - 1][column] = 'O';
                warehouse.map[row][column] = '.';
                return true;
            }

            if warehouse.map[row][column] == '['
                && (can_box_be_moved(warehouse, row, column, direction)
                    || (move_box_pile(warehouse, direction, row - 1, column)
                        && move_box_pile(warehouse, direction, row - 1, column + 1)))
            {
                warehouse.map[row - 1][column] = '[';
                warehouse.map[row - 1][column + 1] = ']';
                warehouse.map[row][column] = '.';
                warehouse.map[row][column + 1] = '.';

                return true;
            }

            if can_box_be_moved(warehouse, row, column, direction)
                || (move_box_pile(warehouse, direction, row - 1, column)
                    && move_box_pile(warehouse, direction, row - 1, column - 1))
            {
                warehouse.map[row - 1][column] = ']';
                warehouse.map[row - 1][column - 1] = '[';
                warehouse.map[row][column] = '.';
                warehouse.map[row][column - 1] = '.';

                return true;
            }
        }
        'v' => {
            if is_wall(warehouse, row + 1, column) {
                return false;
            }

            if warehouse.map[row][column] == 'O'
                && (can_box_be_moved(warehouse, row, column, direction)
                    || move_box_pile(warehouse, direction, row + 1, column))
            {
                warehouse.map[row + 1][column] = 'O';
                warehouse.map[row][column] = '.';
                return true;
            }

            if warehouse.map[row][column] == '['
                && (can_box_be_moved(warehouse, row, column, direction)
                    || (move_box_pile(warehouse, direction, row + 1, column)
                        && move_box_pile(warehouse, direction, row + 1, column + 1)))
            {
                warehouse.map[row + 1][column] = '[';
                warehouse.map[row + 1][column + 1] = ']';
                warehouse.map[row][column] = '.';
                warehouse.map[row][column + 1] = '.';

                return true;
            }

            if can_box_be_moved(warehouse, row, column, direction)
                || (move_box_pile(warehouse, direction, row + 1, column)
                    && move_box_pile(warehouse, direction, row + 1, column - 1))
            {
                warehouse.map[row + 1][column] = ']';
                warehouse.map[row + 1][column - 1] = '[';
                warehouse.map[row][column] = '.';
                warehouse.map[row][column - 1] = '.';

                return true;
            }
        }
        '>' => {
            if is_wall(warehouse, row, column + 1) {
                return false;
            }
            if can_box_be_moved(warehouse, row, column, direction)
                || move_box_pile(warehouse, direction, row, column + 1)
            {
                if warehouse.map[row][column] == 'O' {
                    warehouse.map[row][column + 1] = 'O';
                    warehouse.map[row][column] = '.';
                } else if warehouse.map[row][column] == '[' {
                    warehouse.map[row][column + 2] = ']';
                    warehouse.map[row][column + 1] = '[';
                    warehouse.map[row][column] = '.';
                }

                return true;
            }
        }
        _ => panic!("This character is unexpected on the map!"),
    }

    false
}

fn is_free(warehouse: &Warehouse, row: usize, column: usize, direction: char) -> bool {
    match direction {
        '<' => warehouse.map[row][column - 1] == '.',
        '^' => warehouse.map[row - 1][column] == '.',
        'v' => warehouse.map[row + 1][column] == '.',
        '>' => warehouse.map[row][column + 1] == '.',
        _ => panic!("This character is unexpected on the map!"),
    }
}

fn can_box_be_moved(warehouse: &Warehouse, row: usize, column: usize, direction: char) -> bool {
    let box_type = warehouse.map[row][column];

    match direction {
        '<' => {
            if box_type == 'O' {
                return warehouse.map[row][column - 1] == '.';
            }

            warehouse.map[row][column - 2] == '.'
        }
        '^' => {
            if box_type == 'O' {
                warehouse.map[row - 1][column] == '.'
            } else if box_type == '[' {
                warehouse.map[row - 1][column] == '.' && warehouse.map[row - 1][column + 1] == '.'
            } else {
                warehouse.map[row - 1][column] == '.' && warehouse.map[row - 1][column - 1] == '.'
            }
        }
        'v' => {
            if box_type == 'O' {
                warehouse.map[row + 1][column] == '.'
            } else if box_type == '[' {
                warehouse.map[row + 1][column] == '.' && warehouse.map[row + 1][column + 1] == '.'
            } else {
                warehouse.map[row + 1][column] == '.' && warehouse.map[row + 1][column - 1] == '.'
            }
        }
        '>' => {
            if box_type == 'O' {
                return warehouse.map[row][column + 1] == '.';
            }

            warehouse.map[row][column + 2] == '.'
        }
        _ => panic!("This character is unexpected on the map!"),
    }
}

fn is_wall(warehouse: &Warehouse, row: usize, column: usize) -> bool {
    warehouse.map[row][column] == '#'
}

fn gps(warehouse: &Warehouse) -> usize {
    let mut total = 0usize;
    for (y, row) in warehouse.map.as_slice().iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if *point == 'O' || *point == '[' {
                total += x + (100 * y);
                continue;
            }
        }
    }

    total
}

fn parse(input: &str) -> Result<Warehouse> {
    let mut map: Vec<Vec<char>> = Vec::new();

    let input_split = input.split("\n\n").collect_vec();

    let mut line_add = 1usize;
    let mut bot_initial_row = 0usize;
    let mut bot_initial_column = 0usize;
    for line in input_split[0].lines() {
        if line.is_empty() {
            break;
        }

        let chars = line.chars().collect_vec();
        if let Some(guard_column) = chars.iter().position(|&x| x == '@') {
            bot_initial_column = guard_column;
            line_add = 0;
        }
        map.push(chars);

        bot_initial_row += line_add;
    }

    let valid_movements = ['v', '>', '<', '^'];
    let movements: Vec<char> = input_split[1]
        .trim()
        .chars()
        .filter(|m| valid_movements.contains(m))
        .collect_vec();

    Ok(Warehouse {
        map,
        movements,
        bot_initial_row,
        bot_initial_column,
    })
}

fn enlarge(warehouse: Warehouse) -> Warehouse {
    let mut new_map: Vec<Vec<char>> = Vec::new();

    for map_row in warehouse.map.as_slice().iter() {
        let mut row: Vec<char> = Vec::new();

        for point in map_row.iter() {
            match point {
                '#' => {
                    row.push('#');
                    row.push('#')
                }
                'O' => {
                    row.push('[');
                    row.push(']')
                }
                '.' => {
                    row.push('.');
                    row.push('.')
                }
                '@' => {
                    row.push('@');
                    row.push('.')
                }
                _ => {
                    panic!("Unexpected char!")
                }
            }
        }

        new_map.push(row);
    }

    let mut row: usize = 0;
    let mut column: usize = 0;

    for (x, r) in new_map.iter().enumerate() {
        for (y, c) in r.iter().enumerate() {
            if *c == '@' {
                row = x;
                column = y;
            }
        }
    }

    Warehouse {
        movements: warehouse.movements,
        bot_initial_column: column,
        bot_initial_row: row,
        map: new_map,
    }
}

fn print_map(warehouse: &Warehouse) {
    for row in warehouse.map.as_slice() {
        println!("{}", row.iter().collect::<String>())
    }
}
