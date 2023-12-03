fn main() {
    let file = std::fs::read_to_string("../input.txt").unwrap();
    let lines = file.lines();
    let mut num = 0;
    for line in lines {
        let mut a: String = String::new();
        for c in line.chars() {
            if c.is_ascii_digit() {
                a = format!("{}{}", a, c.to_string().to_owned())
            };
        }

        if a.len() > 2 {
            a = format!(
                "{}{}",
                a.chars().next().unwrap(),
                a.chars().nth(a.len() - 1).unwrap()
            );
            println!("{}", a);
        } else if a.len() == 1 {
            a = format!("{}{}", a, a);
            println!("{}", a);
        }
        num += a.parse::<i32>().unwrap();
    }
    println!("{}", num)
}
