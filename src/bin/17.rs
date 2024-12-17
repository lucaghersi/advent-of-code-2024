use adv_code_2024::start_day;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::ops::BitXor;

const DAY: &str = "17";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

const TEST2: &str = "\
Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    async fn part1(input: &str) -> Result<String> {
        let mut computer = parse(input);
        computer.execute();
        Ok(computer.get_output())
    }

    async fn part2(input: &str) -> Result<u64> {
        let computer = parse(input);
        let mut a: u64 = 0;
        let mut ind = computer.instructions.len() - 1;

        loop {
            let mut comp_try = Computer {
                register_a: a,
                register_b: 0,
                register_c: 0,
                instructions: computer.instructions.clone(),
                output: Vec::new(),
            };
            comp_try.execute();

            while comp_try.output.len() < comp_try.instructions.len() {
                comp_try.output.push(1000);
            }

            if comp_try.output[ind] == comp_try.instructions[ind] {
                if ind == 0 {
                    break;
                } else {
                    ind -= 1;
                }
            } else {
                a += 8u64.pow(ind as u32);
            }
        }

        Ok(a)
    }

    let result = part1(TEST).await?;
    println!("Test Result 1 = {}", result);
    assert_eq!("4,6,3,5,6,3,5,2,1,0", result);

    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);

    let result = time_snippet!(part1(&input).await?);
    println!("Result 1 = {}", result);
    assert_eq!("1,2,3,1,3,2,5,3,1", result);

    println!("=== Part 2 ===");

    let result = part2(TEST2).await?;
    println!("Test Result 2 = {}", result);
    assert_eq!(117440, result);

    let result = time_snippet!(part2(&input).await?);
    println!("Result 2 = {}", result);

    anyhow::Ok(())
}

#[derive(Debug)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instructions: Vec<u64>,
    output: Vec<u64>,
}

impl Computer {
    fn get_combo_operand(&self, op_pointer: usize) -> u64 {
        let operand = self.instructions[op_pointer + 1];

        if operand <= 3 {
            return operand;
        }
        if operand == 4 {
            return self.register_a;
        }
        if operand == 5 {
            return self.register_b;
        }
        if operand == 6 {
            return self.register_c;
        }

        panic!("Operand not allowed!");
    }

    fn get_literal_operand(&self, op_pointer: usize) -> u64 {
        self.instructions[op_pointer + 1]
    }
}

#[derive(Debug)]
enum Instruction {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl TryFrom<u64> for Instruction {
    type Error = ();

    fn try_from(v: u64) -> Result<Self, Self::Error> {
        match v {
            x if x == Instruction::Adv as u64 => Ok(Instruction::Adv),
            x if x == Instruction::Bxl as u64 => Ok(Instruction::Bxl),
            x if x == Instruction::Bst as u64 => Ok(Instruction::Bst),
            x if x == Instruction::Jnz as u64 => Ok(Instruction::Jnz),
            x if x == Instruction::Bxc as u64 => Ok(Instruction::Bxc),
            x if x == Instruction::Out as u64 => Ok(Instruction::Out),
            x if x == Instruction::Bdv as u64 => Ok(Instruction::Bdv),
            x if x == Instruction::Cdv as u64 => Ok(Instruction::Cdv),
            _ => Err(()),
        }
    }
}

trait CanExecute {
    fn execute(&mut self);
    fn get_output(&self) -> String;
}

impl CanExecute for Computer {
    fn execute(&mut self) {
        let mut op_pointer: usize = 0;
        loop {
            if op_pointer >= self.instructions.len() - 1 {
                break;
            }

            let op: Instruction = self.instructions[op_pointer].try_into().unwrap();

            match op {
                Instruction::Adv => {
                    let co = self.get_combo_operand(op_pointer);
                    let divisor = u64::pow(2, co as u32);
                    self.register_a = self.register_a.div_euclid(divisor);
                }
                Instruction::Bxl => {
                    let lo = self.get_literal_operand(op_pointer);
                    self.register_b = self.register_b.bitxor(lo);
                }
                Instruction::Bst => {
                    let co = self.get_combo_operand(op_pointer);
                    self.register_b = co % 8;
                }
                Instruction::Jnz => {
                    if self.register_a != 0 {
                        let lo = self.get_literal_operand(op_pointer);
                        op_pointer = lo as usize;
                        continue;
                    }
                }
                Instruction::Bxc => {
                    self.register_b = self.register_b.bitxor(self.register_c);
                }
                Instruction::Out => {
                    let co = self.get_combo_operand(op_pointer);
                    self.output.push(co % 8);
                }
                Instruction::Bdv => {
                    let co = self.get_combo_operand(op_pointer);
                    let divisor = u64::pow(2, co as u32);
                    self.register_b = self.register_a.div_euclid(divisor);
                }
                Instruction::Cdv => {
                    let co = self.get_combo_operand(op_pointer);
                    let divisor = u64::pow(2, co as u32);
                    self.register_c = self.register_a.div_euclid(divisor);
                }
            }

            op_pointer += 2;
        }
    }

    fn get_output(&self) -> String {
        self.output.iter().join(",")
    }
}

#[allow(unused_assignments)]
fn parse(input: &str) -> Computer {
    let regex = Regex::new("A:\\s(?<a>\\d*)\\n.*B:\\s(?<b>\\d*)\\n.*C:\\s(?<c>\\d*)\\n\\nProgram:\\s(?<program>(\\d,|\\d)*|)").unwrap();
    let captures = regex.captures(input).unwrap();

    let register_a = captures.name("a").unwrap().as_str().parse::<u64>().unwrap();
    let register_b = captures.name("b").unwrap().as_str().parse::<u64>().unwrap();
    let register_c = captures.name("c").unwrap().as_str().parse::<u64>().unwrap();
    let program = captures.name("program").unwrap().as_str();

    let instructions = program
        .split(',')
        .map(|f| f.parse::<u64>().unwrap())
        .collect_vec();

    Computer {
        register_a,
        register_b,
        register_c,
        instructions,
        output: Vec::new(),
    }
}
