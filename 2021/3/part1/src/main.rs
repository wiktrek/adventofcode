/*
    It works!

    Sorry.
*/
#[derive(Debug)]
struct A {
    zero: i32,
    one: i32,
}
fn main() {
    let file = std::fs::read_to_string("../input.txt").unwrap();
    let lines = file.lines();
    let mut gamma: Vec<A> = vec![
        A { zero: 0, one: 0 },
        A { zero: 0, one: 0 },
        A { zero: 0, one: 0 },
        A { zero: 0, one: 0 },
        A { zero: 0, one: 0 },
        A { zero: 0, one: 0 },
        A { zero: 0, one: 0 },
        A { zero: 0, one: 0 },
        A { zero: 0, one: 0 },
        A { zero: 0, one: 0 },
        A { zero: 0, one: 0 },
        A { zero: 0, one: 0 },
    ];
    for line in lines {
        let bits: Vec<u32> = line
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();
        for (i, n) in bits.iter().enumerate() {
            match *n {
                1 => gamma[i].one += 1,
                0 => gamma[i].zero += 1,
                _ => return,
            }
        }
        println!("{:?}, {:?}", bits, gamma);
    }
    let mut str: String = "".to_string();
    for n in gamma.iter() {
        if n.one > n.zero {
            str += "1"
        } else {
            str += "0"
        }
    }
    let i = i32::from_str_radix(str.as_str(), 2).expect("Not a binary number!");
    let i2 = i32::from_str_radix(
        str.replace('1', "a")
            .replace('0', "b")
            .replace('b', "1")
            .replace('a', "0")
            .as_str(),
        2,
    )
    .expect("Err");
    println!("Str: {} {} {} answer: {}", str, i, i2, i * i2)
}
