fn main() {
    let file = std::fs::read_to_string("./input.txt").unwrap();
    let lines = file.lines();
    let mut x = 0; // horizontal
    let mut y = 0; // depth
    let mut aim = 0;
    for line in lines {
        if line.starts_with("down") {
            let num = line.replace("down ", "").parse::<i32>().unwrap();
            aim += num;
        }
        if line.starts_with("up") {
            let num = line.replace("up ", "").parse::<i32>().unwrap();
            aim -= num;
        }
        if line.starts_with("forward") {
            let num = line.replace("forward ", "").parse::<i32>().unwrap();
            x += num;
            y += aim * num;
        }

        // println!("{}, {}, {}", line, x, y);
    }
    println!("{}, {}, {}", x, y, x * y);
}
