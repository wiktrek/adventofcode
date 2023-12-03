fn main() {
    let file = std::fs::read_to_string("../input.txt").unwrap();
    let lines = file.lines();
    let mut num = 0;
    for l in lines {
        let line = l
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3ree")
            .replace("four", "f4ur")
            .replace("five", "f5ve")
            .replace("six", "s6x")
            .replace("seven", "s7ven")
            .replace("eight", "e8ght")
            .replace("nine", "n9ne");
        // .replace("zero", "0");

        println!("{} {}", l, line);
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
