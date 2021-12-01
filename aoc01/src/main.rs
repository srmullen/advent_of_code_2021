use std::fs;
use std::io::{self, BufRead};

fn get_readings() -> Vec<i32> {
    let input = "input/input.txt";
    let file = fs::File::open(input).unwrap();
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
    let mut readings = vec![];
    for line in lines {
        if let Ok(l) = line {
            let reading = l.parse::<i32>().unwrap();
            readings.push(reading);
        }
    }
    return readings;
}

fn part1() {
    let mut count = 0;
    let mut prev: Option<i32> = None;
    let readings = get_readings();
    for reading in readings {
        match prev {
            Some(n) => if reading > n {
                count += 1;
            },
            None => {}
        }
        prev = Some(reading);
    }
    println!("Count: {}", count);
}

fn part2() {
    let readings = get_readings();
    let mut prev: Option<i32> = None;
    let mut count = 0;
    for i in 0..readings.len()-2 {
        let sum = readings[i] + readings[i + 1] + readings[i + 2];
        match prev {
            Some(n) => if sum > n {
                count += 1;
            },
            None => {}
        };
        prev = Some(sum);
    }
    println!("Count: {}", count);
}

fn main() {
    part1();
    part2();
}