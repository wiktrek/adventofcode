use std::{fs::read_to_string, str::Lines};
#[derive(Debug, Clone, PartialEq)]
struct Position(i32, i32);
fn main() {
    part1();
    part2();
}
fn part1() {
    let file = read_to_string("input.txt").unwrap();
    let lines = file.lines();
    let symbols = symbols(lines.clone());
    let nums = numbers(lines.clone());
    let mut total = 0;
    // println!("{:?} \n\n\n {:?}", nums, symbols);
    let rows = lines.count() as i32;

    let m = file.lines().next().unwrap().chars().count() as i32;
    for (n, p) in nums {
        if check_if_near_symbol(p.clone(), symbols.clone(), rows, m) {
            // println!("{}", n);
            total += n;
        }
    }
    println!("{}", total)
}
fn numbers(lines: Lines<'_>) -> Vec<(usize, Vec<Position>)> {
    let mut nums: Vec<(usize, Vec<Position>)> = vec![];
    for (row, line) in lines.enumerate() {
        let mut a: (String, Vec<Position>) = (String::new(), vec![]);
        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                a.0 = format!("{}{}", a.0, ch.to_digit(10).unwrap());
                a.1.push(Position(row as i32, col as i32));
                // println!("{ch}   {}", ch.to_digit(10).unwrap() as usize);
            } else if !a.1.is_empty() && !a.0.is_empty() {
                nums.push((a.0.parse::<usize>().unwrap(), a.1.clone()));

                a.0 = String::new();
                a.1 = vec![]
            }
        }
        if !a.1.is_empty() && !a.0.is_empty() {
            nums.push((a.0.parse::<usize>().unwrap(), a.1.clone()));

            a.0 = String::new();
            a.1 = vec![]
        }
    }
    nums
}
fn check_if_near_symbol(position: Vec<Position>, symbols: Vec<Position>, n: i32, m: i32) -> bool {
    for s in symbols {
        let neighbors = [
            Position(s.0 - 1, s.1 - 1),
            Position(s.0 - 1, s.1),
            Position(s.0 - 1, s.1 + 1),
            Position(s.0, s.1 - 1),
            Position(s.0, s.1 + 1),
            Position(s.0 + 1, s.1 - 1),
            Position(s.0 + 1, s.1),
            Position(s.0 + 1, s.1 + 1),
        ]
        .into_iter()
        .filter(|c| c.0 >= 0 && c.1 >= 0 && c.0 < n && c.1 < m)
        .collect::<Vec<Position>>();
        for p in neighbors.iter() {
            if position.contains(p) {
                return true;
            }
        }
    }
    false
}
fn symbols(lines: Lines<'_>) -> Vec<Position> {
    let mut symbols: Vec<Position> = vec![];
    for (row, line) in lines.enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if !ch.is_ascii_digit() && ch != '.' {
                symbols.push(Position(row as i32, col as i32));
            }
        }
    }
    symbols
}
fn part2() {}
// use regex::Regex;
// use std::env;
// use std::fs;
// use std::process;
// #[derive(Debug)]
// struct Position(i32, i32);

// fn usage() {
//     let progname = env::args().next().unwrap();
//     println!("usage: {progname} <file>");
//     process::exit(1);
// }

// fn build_numbers(contents: &str) -> Vec<(&str, Position)> {
//     let mut numbers: Vec<(&str, Position)> = vec![];
//     let num_re = Regex::new(r"\d+").unwrap();
//     for (row, line) in contents.lines().enumerate() {
//         for num_match in num_re.find_iter(line) {
//             numbers.push((
//                 num_match.as_str(),
//                 Position(row as i32, num_match.start() as i32),
//             ));
//         }
//     }
//     numbers
// }

// fn build_symbols(contents: &str) -> Vec<(char, Position)> {
//     let mut symbols: Vec<(char, Position)> = vec![];
//     for (row, line) in contents.lines().enumerate() {
//         for (col, ch) in line.chars().enumerate() {
//             if !ch.is_ascii_digit() && ch != '.' {
//                 symbols.push((ch, Position(row as i32, col as i32)));
//             }
//         }
//     }
//     symbols
// }

// fn build_part_numbers(
//     num_rows: u32,
//     num_cols: u32,
//     numbers: &[(&str, Position)],
//     symbols: &[(char, Position)],
// ) -> Vec<u32> {
//     let mut part_numbers: Vec<u32> = vec![];
//     for (num_str, num_start_pos) in numbers.iter() {
//         let num_end_pos = Position(num_start_pos.0, num_start_pos.1 + num_str.len() as i32 - 1);
//         for (_, symbol_pos) in symbols.iter() {
//             let neighbors = [
//                 Position(symbol_pos.0 - 1, symbol_pos.1 - 1),
//                 Position(symbol_pos.0 - 1, symbol_pos.1),
//                 Position(symbol_pos.0 - 1, symbol_pos.1 + 1),
//                 Position(symbol_pos.0, symbol_pos.1 - 1),
//                 Position(symbol_pos.0, symbol_pos.1 + 1),
//                 Position(symbol_pos.0 + 1, symbol_pos.1 - 1),
//                 Position(symbol_pos.0 + 1, symbol_pos.1),
//                 Position(symbol_pos.0 + 1, symbol_pos.1 + 1),
//             ]
//             .into_iter()
//             .filter(|p| s.0 >= 0 && s.1 >= 0 && p.0 < num_rows as i32 && p.1 < num_cols as i32)
//             .collect::<Vec<Position>>();
//             for neighbor in neighbors.iter() {
//                 if neighbor.0 == num_start_pos.0
//                     && neighbor.1 >= num_start_pos.1
//                     && neighbor.1 <= num_end_pos.1
//                 {
//                     part_numbers.push(num_str.parse::<u32>().unwrap());
//                     break;
//                 }
//             }
//         }
//     }
//     part_numbers
// }

// fn process(contents: &str) -> u32 {
//     let num_rows = contents.lines().count() as u32;
//     let num_cols = contents.lines().next().unwrap().chars().count() as u32;
//     let numbers = build_numbers(contents);
//     let symbols = build_symbols(contents);
//     let part_numbers = build_part_numbers(num_rows, num_cols, &numbers, &symbols);
//     for n in part_numbers.clone() {
//         println!("{:?}", n);
//     }
//     part_numbers.iter().sum()
// }

// fn main() {
//     if env::args().count() < 2 {
//         usage();
//     }
//     let filename = env::args().nth(1).unwrap();
//     let contents = fs::read_to_string(filename).expect("read of input file failed");
//     let result = process(&contents);
//     println!("result = {result}");
// }
