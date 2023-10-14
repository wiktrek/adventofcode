fn main() {
    let file = std::fs::read_to_string("../input.txt").unwrap();
    let lines = file.lines();
    let mut a = false;
    let mut nums: Vec<i32> = vec![];
    for line in lines {
        if a {
            // println!("{} aaaaa", line);
        } else {
            a = true;
            println!("{}", line);
            let n: Vec<&str> = line.split(',').collect();

            n.iter()
                .for_each(|i| nums.push((**i).parse::<i32>().unwrap()));
            println!("{:?}", nums);
        }
    }
}
