fn main() {
    let file = std::fs::read_to_string("../input.txt").unwrap();
    let lines = file.lines();
    let mut data: Vec<i32> = vec![];
    for a in lines {
        let b = a.parse::<i32>().unwrap();
        data.push(b)
    }
    let c = data.windows(3);

    let answer = c
        .zip(data.windows(3).skip(1))
        .filter(|(x, y)| x.iter().sum::<i32>() < y.iter().sum())
        .count();
    println!("{:?}", answer);
}
