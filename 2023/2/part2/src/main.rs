/*
    It works but I don't recommend using this
*/
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone)]
struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}
fn main() {
    let file = read_to_string("../input.txt").unwrap();
    let lines = file.lines();
    let mut n = 0;
    for line in lines {
        let id = line.split(':').collect::<Vec<&str>>()[0]
            .replace("Game ", "")
            .parse::<i32>()
            .unwrap();
        println!("{}", id);
        let sets = line.split(':').collect::<Vec<&str>>()[1]
            .split(';')
            .collect::<Vec<&str>>();
        let mut v: Vec<Cubes> = Vec::new();
        for set in sets {
            let s = set.split(", ").collect::<Vec<&str>>();
            println!("{:?}", s);
            let mut cu: Cubes = Cubes {
                red: 0,
                green: 0,
                blue: 0,
            };
            for cube in s {
                if cube.contains("red") {
                    cu.red = cube
                        .trim()
                        .replace(" red", "")
                        .to_string()
                        .parse::<i32>()
                        .unwrap();
                } else if cube.contains("blue") {
                    cu.blue = cube
                        .trim()
                        .replace(" blue", "")
                        .to_string()
                        .parse::<i32>()
                        .unwrap();
                } else if cube.contains("green") {
                    cu.green = cube
                        .trim()
                        .replace(" green", "")
                        .to_string()
                        .parse::<i32>()
                        .unwrap();
                }
            }
            v.push(cu);
            // println!("{:?}", cu);
        }
        let mut prev = Cubes {
            red: 0,
            green: 0,
            blue: 0,
        };
        for c in v {
            if prev.red == 0 && prev.green == 0 && prev.blue == 0 {
                prev = c
            } else if prev.red < c.red {
                prev.red = c.red;
            }
            if prev.green < c.green {
                prev.green = c.green;
            }
            if prev.blue < c.blue {
                prev.blue = c.blue;
            }
        }
        println!("{:?}", prev);
        n += prev.red * prev.blue * prev.green;
    }
    println!("{:?}", n);
}
