use std::fs::read_to_string;
#[derive(Clone, Debug)]
struct ScratchCard {
    won: usize,
}
fn main() {
    println!("Day 4");
    part1();
    part2();
}

#[deny(clippy::never_loop)]
fn part1() {
    let file = read_to_string("input.txt").unwrap();
    let lines = file.lines();
    let mut sum = 0;

    for (ind, line) in lines.enumerate() {
        // remove "Card num:"
        let l = line.replace("Card ", " ");
        let s: Vec<&str> = l.split(": ").collect();
        let nums: Vec<&str> = s[1].trim_start().split(" |").collect();
        let your: Vec<&str> = nums[0].split_ascii_whitespace().collect();
        let winning: Vec<&str> = nums[1].split_ascii_whitespace().collect();
        // println!("{:?}, {:?}", winning, your);
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
        println!("{}", n);
        if n >= 1 {
            sum += 2_i32.pow(n - 1);
        };
    }
    println!("{}", sum)
}
fn part2() {
    let file = read_to_string("input.txt").unwrap();
    let lines = file.lines();
    let mut cards: Vec<ScratchCard> = vec![];
    for line in lines {
        let string = line.replace("Card ", " ");
        let (winning, your) = format(string.as_str());
        let matches = winning.iter().filter(|n| your.contains(n)).count();
        cards.push(ScratchCard { won: matches })
    }
    let mut l: Vec<i32> = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let len = l.len() - 1;
        let j = if card.won > len { len } else { card.won + i };
        for k in i + 1..j + 1 {
            l[k] += l[i];
        }
    }
    println!("{}", l.iter().sum::<i32>());
}
fn format(string: &str) -> (Vec<&str>, Vec<&str>) {
    let s: Vec<&str> = string.split(": ").collect();
    let nums: Vec<&str> = s[1].trim_start().split(" |").collect();
    let winning: Vec<&str> = nums[0].split_ascii_whitespace().collect();
    let your: Vec<&str> = nums[1].split_ascii_whitespace().collect();
    (winning, your)
}
