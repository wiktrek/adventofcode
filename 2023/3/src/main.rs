use std::fs::read_to_string;
struct Chars {
    position: (usize, usize),
}
fn main() {
    part1();
    part2();
}
fn part1() {
    let file = read_to_string("input.txt").unwrap();
    let lines = file.lines();
    let mut digits: Vec<Chars> = vec![];
    let mut symbols: Vec<Chars> = vec![];
    for (i, line) in lines.enumerate() {
        for (ind, c) in line.chars().enumerate() {
            if c != '.' {
                // println!("{}", c);
                if c.is_ascii_digit() {
                    // digits.push(Chars { position: (i, ind) });
                } else {
                    symbols.push(Chars { position: (i, ind) })
                }
            }
        }
    }
}
fn part2() {}
