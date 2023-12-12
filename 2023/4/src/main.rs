use std::fs::read_to_string;

fn main() {
    println!("Day 4");
    part1();
    part2();
}
fn part1() {
    let file = read_to_string("input.txt").unwrap();
    let lines = file.lines();
    let mut sum = 0;
    for line in lines {
        // remove "Card num:"
        let l = line.replace("Card ", " ");
        let s: Vec<&str> = l.split(": ").collect();
        let nums: Vec<&str> = s[1].trim_start().split(" |").collect();
        let winning: Vec<&str> = nums[0].split_ascii_whitespace().collect();
        let your: Vec<&str> = nums[1].split_ascii_whitespace().collect();
        println!("{:?}, {:?}", winning, your);
        let mut n = 0;
        for w in winning {
            for i in &your {
                // println!("{w}, {i}");
                if w == *i {
                    // println!("{w}, {i} SAME {n}");
                    n += 1
                }
            }
        }
        if n >= 1 {
            sum += 2_i32.pow(n - 1);
        }
    }
    println!("{}", sum)
}
fn part2() {}
