fn main() {
    let file = std::fs::read_to_string("./input.txt").unwrap();
    let mut prev_num = 100000;
    let mut larger = 0;
    let lines = file.lines();
    for line in lines {
        let num: i32 = line.parse::<i32>().unwrap();
        if num >= prev_num {
            larger += 1;
        }
        prev_num = num;

        println!("{}", num);
    }
    println!("{}", larger);
}
