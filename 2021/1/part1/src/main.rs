fn main() {
    let file = std::fs::read_to_string("../input.txt").unwrap();
    let mut data: Vec<i32> = vec![];
    let lines = file.lines();
    for line in lines {
        let num: i32 = line.parse::<i32>().unwrap();
        data.push(num);
    }
    println!(
        "{}",
        data.iter()
            .zip(data.iter().skip(1))
            .filter(|(x, y)| x < y)
            .count()
    );
}
