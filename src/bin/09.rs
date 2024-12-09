use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "2333133121414131402";

#[tokio::main]
async fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    async fn part1(input: &str) -> Result<usize> {
        let disk_map = parse(input).await;
        let disk_map = collapse_free_space(disk_map);
        let checksum = calculate_checksum(&disk_map);
        Ok(checksum)
    }

    async fn part2(input: &str) -> Result<usize> {
        let disk_map = parse(input).await;
        let disk_map = defragment_files(disk_map);
        let checksum = calculate_checksum(&disk_map);
        Ok(checksum)
    }

    let result = part1(TEST).await?;
    println!("Test Result 1 = {}", result);
    assert_eq!(1928, result);

    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);

    let result = time_snippet!(part1(&input).await?);
    println!("Result 1 = {}", result);
    assert_eq!(6353658451014, result);

    println!("=== Part 2 ===");

    let result = part2(TEST).await?;
    println!("Test Result 2 = {}", result);
    assert_eq!(2858, result);

    let result = time_snippet!(part2(&input).await?);
    println!("Result 2 = {}", result);

    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct DiskSegment {
    id: Option<usize>,
    length: u32,
    is_file: bool,
}

impl Display for DiskSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_file {
            write!(f, "File {} (length: {})", self.id.unwrap(), self.length)
        } else {
            write!(f, "Empty space (length: {})", self.length)
        }
    }
}

async fn parse(input: &str) -> Vec<DiskSegment> {
    let mut is_file = true;
    let mut file_id = 0;

    let mut disk_map: Vec<DiskSegment> = Vec::new();

    for c in input.chars() {
        if let Some(length) = c.to_digit(10) {
            let segment = DiskSegment {
                id: if is_file { Some(file_id) } else { None },
                length,
                is_file,
            };
            disk_map.push(segment);

            if is_file {
                is_file = false;
            } else {
                is_file = true;
                file_id += 1;
            }
        } else {
            break;
        }
    }

    //debug(&disk_map);

    disk_map
}

fn defragment_files(mut map: Vec<DiskSegment>) -> Vec<DiskSegment> {
    let mut collapsed_disk_map: Vec<DiskSegment> = Vec::with_capacity(map.len());

    loop {
        if map.is_empty() {
            break;
        }

        let first = map.remove(0);
        if first.is_file {
            collapsed_disk_map.push(first);
            continue;
        }

        let mut space_left = first.length;
        loop {
            let maybe_fit = map
                .iter()
                .rposition(|&p| p.is_file && p.length <= space_left);
            if maybe_fit.is_none() {
                break;
            }

            let fitting_file_position = maybe_fit.unwrap();
            let fitting_file = map[fitting_file_position];
            collapsed_disk_map.push(fitting_file);
            map[fitting_file_position] = DiskSegment {
                is_file: false,
                id: None,
                length: fitting_file.length,
            };

            space_left = space_left.abs_diff(fitting_file.length);
            if space_left == 0 {
                break;
            }
        }

        if space_left > 0 {
            collapsed_disk_map.push(DiskSegment {
                is_file: false,
                id: None,
                length: space_left,
            });
        }
    }

    //debug(&collapsed_disk_map);

    collapsed_disk_map
}

fn collapse_free_space(mut map: Vec<DiskSegment>) -> Vec<DiskSegment> {
    let mut collapsed_disk_map: Vec<DiskSegment> = Vec::with_capacity(map.len());

    loop {
        if map.is_empty() {
            break;
        }

        let first = map.remove(0);
        if first.is_file {
            collapsed_disk_map.push(first);
            continue;
        }

        let mut space_left = first.length;
        loop {
            let last_item_result = map.pop();
            if last_item_result.is_none() {
                break;
            }
            let mut last_item = last_item_result.unwrap();

            if !last_item.is_file {
                continue;
            }

            if space_left > last_item.length {
                space_left -= last_item.length;
                collapsed_disk_map.push(last_item);
            } else {
                collapsed_disk_map.push(DiskSegment {
                    is_file: true,
                    id: last_item.id,
                    length: space_left,
                });
                last_item.length -= space_left;
                map.push(last_item);
                break;
            }
        }
    }

    //debug(&collapsed_disk_map);

    collapsed_disk_map
}

fn calculate_checksum(map: &[DiskSegment]) -> usize {
    let mut checksum = 0;
    let mut position = 0;

    for f in map.iter() {
        for _ in 0..f.length {
            if f.is_file {
                checksum += position * f.id.unwrap();
            }
            position += 1;
        }
    }
    checksum
}

fn debug(map: &[DiskSegment]) {
    for segment in map {
        if segment.is_file {
            print!(
                "{}",
                segment
                    .id
                    .unwrap()
                    .to_string()
                    .repeat(segment.length as usize)
            );
        } else {
            print!("{}", ".".repeat(segment.length as usize));
        }
    }

    println!();
}
