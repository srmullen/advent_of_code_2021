use std::fs;

#[derive(Debug)]
enum Movement {
    Up(i32),
    Down(i32),
    Forward(i32)
}

impl Movement {
    pub fn new(dir: &str, val: i32) -> Movement {
        match dir {
            "up" => Movement::Up(val),
            "down" => Movement::Down(val),
            "forward" => Movement::Forward(val),
            _ => panic!("Not a direction")
        }
    }
}

// FIXME: Would be better to return an iterator here, but I don't know how to type the return value.
fn get_movements(contents: String) -> Vec<Movement> {
    contents.lines().map(|line| {
        let direction = line.split(" ").collect::<Vec<&str>>();
        Movement::new(direction[0], direction[1].parse::<i32>().unwrap())
    }).collect()
}

fn part1() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let position = get_movements(contents).iter().fold((0, 0), |acc, movement| {
        match movement {
            Movement::Up(n) => (acc.0, acc.1 - n),
            Movement::Down(n) => (acc.0, acc.1 + n),
            Movement::Forward(n) => (acc.0 + n, acc.1),
        }
    });
    println!("Part 1 - Position: {}", position.0 * position.1);
}

fn part2() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let position = get_movements(contents).iter().fold((0, 0, 0), |acc, movement| {
        match movement {
            Movement::Up(n) => (acc.0, acc.1, acc.2 - n),
            Movement::Down(n) => (acc.0, acc.1, acc.2 + n),
            Movement::Forward(n) => (acc.0 + n, acc.1 + acc.2 * n, acc. 2)
        }
    });

    // dbg!(position);
    println!("Part 2 - Position: {}", position.0 * position.1);
}

fn main() {
    part1();
    part2();
}
