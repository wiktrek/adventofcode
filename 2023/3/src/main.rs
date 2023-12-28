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
fn part2() {
    let file = read_to_string("input.txt").unwrap();
    let lines = file.lines();
    let nums = numbers(lines.clone());
    let symbols = star_symbols(lines.clone());
    let rows = lines.count() as i32;
    let m = file.lines().next().unwrap().chars().count() as i32;
    let mut total = 0;

    for s in symbols {
        total += check_if_near_numbers(nums.clone(), s, rows, m);
    }
    println!("{}", total)
}
fn check_if_near_numbers(
    mut nums: Vec<(usize, Vec<Position>)>,
    s: Position,
    n: i32,
    m: i32,
) -> i32 {
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
    let mut a: Vec<usize> = vec![];

    for n in neighbors.iter() {
        for (idx, (num, p)) in nums.clone().iter().enumerate() {
            if p.contains(n) {
                a.push(*num);
                nums.remove(idx);
                if a.len() == 2 {
                    println!("{:?}", a);
                    return a.iter().product::<usize>() as i32;
                }
            }
        }
    }
    0
}
fn star_symbols(lines: Lines<'_>) -> Vec<Position> {
    let mut symbols: Vec<Position> = vec![];
    for (row, line) in lines.enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '*' {
                symbols.push(Position(row as i32, col as i32));
            }
        }
    }
    symbols
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
