struct Column {
    one: String,
    two: String,
    three: String,
    four: String,
    five: String,
}
fn main() {
    let file = std::fs::read_to_string("../input.txt").unwrap();
    let lines = file.lines();
    let mut a = false;
    let mut nums: Vec<&str> = vec![];
    let cols: Vec<Column> = vec![];
    let mut j = 0;
    let mut k = 0;
    for line in lines {
        if a {
            // println!("{} aaaaa", line);
            if k == 5 {
                k = 0;
            }
            j = 0;
            for n in nums.iter() {
                if line.contains(format!(" {} ", n).as_str()) {
                    // println!("{} {}", line, n);
                    j += 1;
                    // println!("{}", j);
                    if j == 5 {
                        println!("n: {} line: {}", n, line)
                    }
                };
            }
        } else {
            a = true;
            println!("{}", line);
            nums = line.split(',').collect();

            println!("{:?}", nums);
        }
    }
}
