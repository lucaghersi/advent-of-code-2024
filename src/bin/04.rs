use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{all, Itertools};
use regex::Regex;
use std::fs::File;
use std::io::Read;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

const TEST2: &str = "\
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
";

const X_CHAR: u8 = 88;
const M_CHAR: u8 = 77;
const A_CHAR: u8 = 65;
const S_CHAR: u8 = 83;

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<usize> {
        let parsing = parse(input)?;
        let result = analyze_xmas(&parsing)?;
        Ok(result)
    }

    fn part2(input: &str) -> Result<usize> {
        let parsing = parse(input)?;
        let result = analyze_x_mas(&parsing)?;
        Ok(result)
    }

    let result = part1(TEST)?;
    println!("Test Result 1 = {}", result);
    assert_eq!(18, result);

    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);

    let result = time_snippet!(part1(&input)?);
    println!("Result 1 = {}", result);

    println!("=== Part 2 ===");

    let result = part2(TEST2)?;
    println!("Test Result 2 = {}", result);
    assert_eq!(9, result);

    let result = time_snippet!(part2(&input)?);
    println!("Result 2 = {}", result);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<Vec<u8>>> {
    let height = input.lines().count();
    let mut width = 0;
    if let Some(line) = input.lines().next() {
        width = line.len();
    }

    let mut state = vec![vec![0u8; width]; height];

    let mut row_index = 0;
    let mut column_index;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        column_index = 0;
        for char in line.chars() {
            state[row_index][column_index] = char as u8;
            column_index += 1;
        }
        row_index += 1;
    }

    Ok(state)
}

fn analyze_x_mas(matrix: &Vec<Vec<u8>>) -> Result<usize> {
    let mut total = 0usize;
    let mas = vec![M_CHAR, A_CHAR, S_CHAR];

    for row_index in 0..matrix.len() {
        for column_index in 0..matrix[row_index].len() {
            total += explore_cross(matrix, &row_index, &column_index, &mas, &1)?;
        }
    }

    Ok(total)
}

fn analyze_xmas(matrix: &Vec<Vec<u8>>) -> Result<usize> {
    let mut total = 0usize;
    let xmas = vec![X_CHAR, M_CHAR, A_CHAR, S_CHAR];

    for row_index in 0..matrix.len() {
        for column_index in 0..matrix[row_index].len() {
            total += explore_line(matrix, &row_index, &column_index, &xmas, &0)?;
            total += explore_column(matrix, &row_index, &column_index, &xmas, &0)?;
            total += explore_diagonals(matrix, &row_index, &column_index, &xmas, &0)?;
        }
    }

    Ok(total)
}

fn explore_diagonals(
    matrix: &Vec<Vec<u8>>,
    pos_row: &usize,
    pos_column: &usize,
    word: &Vec<u8>,
    index: &usize,
) -> Result<usize> {
    let current_value = matrix[*pos_row][*pos_column];
    if current_value != word[*index] {
        return Ok(0);
    }

    let total = explore_diagonal_left_up(matrix, pos_row, pos_column, word)?
        + explore_diagonal_left_down(matrix, pos_row, pos_column, word)?
        + explore_diagonal_right_up(matrix, pos_row, pos_column, word)?
        + explore_diagonal_right_down(matrix, pos_row, pos_column, word)?;

    Ok(total)
}

fn explore_cross(
    matrix: &[Vec<u8>],
    x_pos_row: &usize,
    x_pos_column: &usize,
    word: &[u8],
    index: &usize,
) -> Result<usize> {
    let x_pos_column = *x_pos_column;
    let x_pos_row = *x_pos_row;
    let word_len = word.len();
    let l_index = *index;
    let r_index = word_len - l_index - 1;

    if matrix[x_pos_row][x_pos_column] != word[l_index]
    {
        return Ok(0);
    }

    if x_pos_column < l_index || x_pos_column + r_index > matrix[x_pos_row].len() - 1 {
        return Ok(0);
    }

    if x_pos_row + r_index > matrix.len() - 1 || x_pos_row < l_index {
        return Ok(0);
    }

    if (matrix[x_pos_row - 1][x_pos_column - 1] == word[0] && matrix[x_pos_row + 1][x_pos_column + 1] == word[2]) &&
        (matrix[x_pos_row - 1][x_pos_column + 1] == word[0] && matrix[x_pos_row + 1][x_pos_column - 1] == word[2])
    {
        println!("{} {}", matrix[x_pos_row - 1][x_pos_column - 1] as char, matrix[x_pos_row + 1][x_pos_column + 1] as char);
        println!("{:>2}", matrix[x_pos_row][x_pos_column] as char);
        println!("{} {}", matrix[x_pos_row - 1][x_pos_column + 1] as char, matrix[x_pos_row + 1][x_pos_column - 1] as char);
        return Ok(1);
    }

    if (matrix[x_pos_row - 1][x_pos_column - 1] == word[2] && matrix[x_pos_row + 1][x_pos_column + 1] == word[0]) &&
        (matrix[x_pos_row - 1][x_pos_column + 1] == word[2] && matrix[x_pos_row + 1][x_pos_column - 1] == word[0])
    {
        println!("{} {}", matrix[x_pos_row - 1][x_pos_column - 1] as char, matrix[x_pos_row + 1][x_pos_column + 1] as char);
        println!("{:>2}", matrix[x_pos_row][x_pos_column] as char);
        println!("{} {}", matrix[x_pos_row - 1][x_pos_column + 1] as char, matrix[x_pos_row + 1][x_pos_column - 1] as char);
        return Ok(1);
    }

    if (matrix[x_pos_row - 1][x_pos_column - 1] == word[0] && matrix[x_pos_row + 1][x_pos_column + 1] == word[2]) &&
        (matrix[x_pos_row - 1][x_pos_column + 1] == word[2] && matrix[x_pos_row + 1][x_pos_column - 1] == word[0])
    {
        println!("{} {}", matrix[x_pos_row - 1][x_pos_column - 1] as char, matrix[x_pos_row + 1][x_pos_column + 1] as char);
        println!("{:>2}", matrix[x_pos_row][x_pos_column] as char);
        println!("{} {}", matrix[x_pos_row - 1][x_pos_column + 1] as char, matrix[x_pos_row + 1][x_pos_column - 1] as char);
        return Ok(1);
    }

    if (matrix[x_pos_row - 1][x_pos_column - 1] == word[2] && matrix[x_pos_row + 1][x_pos_column + 1] == word[0]) &&
        (matrix[x_pos_row - 1][x_pos_column + 1] == word[0] && matrix[x_pos_row + 1][x_pos_column - 1] == word[2])
    {
        println!("{} {}", matrix[x_pos_row - 1][x_pos_column - 1] as char, matrix[x_pos_row + 1][x_pos_column + 1] as char);
        println!("{:>2}", matrix[x_pos_row][x_pos_column] as char);
        println!("{} {}", matrix[x_pos_row - 1][x_pos_column + 1] as char, matrix[x_pos_row + 1][x_pos_column - 1] as char);
        return Ok(1);
    }

    Ok(0)
}

fn explore_diagonal_left_up(
    matrix: &[Vec<u8>],
    x_pos_row: &usize,
    x_pos_column: &usize,
    word: &[u8],
) -> Result<usize> {
    if (*x_pos_column) < (word.len() - 1) || (*x_pos_row) < (word.len() - 1) {
        return Ok(0);
    }

    for i in 1..word.len() {
        let char_match = matrix[*x_pos_row - i][*x_pos_column - i] == word[i];
        if !char_match {
            return Ok(0);
        }
    }

    Ok(1)
}

fn explore_diagonal_left_down(
    matrix: &[Vec<u8>],
    x_pos_row: &usize,
    x_pos_column: &usize,
    word: &[u8],
) -> Result<usize> {
    if (*x_pos_column) < (word.len() - 1) || (matrix.len() - *x_pos_row) < word.len() {
        return Ok(0);
    }

    for i in 1..word.len() {
        let char_match = matrix[*x_pos_row + i][*x_pos_column - i] == word[i];
        if !char_match {
            return Ok(0);
        }
    }

    Ok(1)
}

fn explore_diagonal_right_up(
    matrix: &[Vec<u8>],
    x_pos_row: &usize,
    x_pos_column: &usize,
    word: &[u8],
) -> Result<usize> {
    if (matrix[*x_pos_row].len() - *x_pos_column) < word.len() || *x_pos_row < (word.len() - 1) {
        return Ok(0);
    }

    for i in 1..word.len() {
        let char_match = matrix[*x_pos_row - i][*x_pos_column + i] == word[i];
        if !char_match {
            return Ok(0);
        }
    }

    Ok(1)
}

fn explore_diagonal_right_down(
    matrix: &[Vec<u8>],
    x_pos_row: &usize,
    x_pos_column: &usize,
    word: &[u8],
) -> Result<usize> {
    if (matrix[*x_pos_row].len() - *x_pos_column) < word.len()
        || (matrix.len() - *x_pos_row) < word.len()
    {
        return Ok(0);
    }

    for i in 1..word.len() {
        let char_match = matrix[*x_pos_row + i][*x_pos_column + i] == word[i];
        if !char_match {
            return Ok(0);
        }
    }

    Ok(1)
}

fn explore_line(
    matrix: &Vec<Vec<u8>>,
    pos_row: &usize,
    pos_column: &usize,
    word: &Vec<u8>,
    index: &usize,
) -> Result<usize> {
    let current_value = matrix[*pos_row][*pos_column];
    if current_value != word[*index] {
        return Ok(0);
    }

    let total = if explore_line_left(matrix, pos_row, pos_column, word)? {
        1
    } else {
        0
    } + if explore_line_right(matrix, pos_row, pos_column, word)? {
        1
    } else {
        0
    };

    fn explore_line_left(
        matrix: &[Vec<u8>],
        x_pos_row: &usize,
        x_pos_column: &usize,
        word: &[u8],
    ) -> Result<bool> {
        if *x_pos_column < (word.len() - 1) {
            return Ok(false);
        }

        for i in 1..word.len() {
            let char_match = matrix[*x_pos_row][*x_pos_column - i] == word[i];
            if !char_match {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn explore_line_right(
        matrix: &[Vec<u8>],
        x_pos_row: &usize,
        x_pos_column: &usize,
        word: &[u8],
    ) -> Result<bool> {
        if matrix[*x_pos_row].len() - *x_pos_column < word.len() {
            return Ok(false);
        }

        for i in 1..word.len() {
            let char_match = matrix[*x_pos_row][*x_pos_column + i] == word[i];
            if !char_match {
                return Ok(false);
            }
        }

        Ok(true)
    }

    Ok(total)
}

fn explore_column(
    matrix: &Vec<Vec<u8>>,
    pos_row: &usize,
    pos_column: &usize,
    word: &Vec<u8>,
    index: &usize,
) -> Result<usize> {
    let current_value = matrix[*pos_row][*pos_column];
    if current_value != word[*index] {
        return Ok(0);
    }

    let total = if explore_column_up(matrix, pos_row, pos_column, word)? {
        1
    } else {
        0
    } + if explore_column_down(matrix, pos_row, pos_column, word)? {
        1
    } else {
        0
    };

    fn explore_column_up(
        matrix: &[Vec<u8>],
        x_pos_row: &usize,
        x_pos_column: &usize,
        word: &[u8],
    ) -> Result<bool> {
        if *x_pos_row < (word.len() - 1) {
            return Ok(false);
        }

        for i in 1..word.len() {
            let char_match = matrix[*x_pos_row - i][*x_pos_column] == word[i];
            if !char_match {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn explore_column_down(
        matrix: &[Vec<u8>],
        x_pos_row: &usize,
        x_pos_column: &usize,
        word: &[u8],
    ) -> Result<bool> {
        if matrix.len() - *x_pos_row < word.len() {
            return Ok(false);
        }

        for i in 1..word.len() {
            let char_match = matrix[*x_pos_row + i][*x_pos_column] == word[i];
            if !char_match {
                return Ok(false);
            }
        }

        Ok(true)
    }

    Ok(total)
}
