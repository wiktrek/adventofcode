use std::fs::read_to_string;

fn main() {
    part1();
    part2();
}
fn part1() {
    let file = read_to_string("../input.txt").unwrap();
    let lines = file.lines();
    for line in lines {
        println!("{}", line);
    }
}
fn part2() {}
