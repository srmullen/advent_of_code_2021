use std::fs;

fn get_bit(num: i32, idx: usize) -> bool {
    num & (1 << idx) != 0
}

fn bitstr_to_int(bits: &str) -> i32 {
    bits.chars().enumerate().fold(0, |acc, (i ,c)| {
        if c == '1' {
            acc | (1 << (bits.len() - 1) - i)
        } else {
            acc
        }
    })
}

fn get_readings() -> Vec<i32> {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    contents.lines().map(|line| {
        bitstr_to_int(line)
    }).collect()
}

fn gamma_rate(readings: &Vec<i32>) -> i32 {
    let mut rate = 0;
    for shift in 0..12 {
        let mut count = 0;
        for reading in readings {
            if get_bit(*reading, shift) {
                count += 1;
            }
        }
        
        if count > (readings.len()) / 2 {
            rate = rate | (1 << shift);
        } 
    }
    rate
}

fn epsilon_rate(gamma: i32) -> i32 {
    let mut epsilon = 0;
    for shift in 0..12 {
        if !get_bit(gamma, shift) {
            epsilon = epsilon | (1 << shift);
        }
    }
    epsilon
}

fn part1() {
    let readings = get_readings();
    let gamma = gamma_rate(&readings);
    let epsilon = epsilon_rate(gamma);
    println!("Power consumption: {}", gamma * epsilon);
}

fn most_common_bit(readings: &Vec<i32>, shift: usize) -> (i32, i32) {
    let mut ones = 0;
    let mut zeros = 0;

    for reading in readings {
        if get_bit(*reading, shift) {
            ones += 1;
        } else {
            zeros += 1;
        }
    }
    (zeros, ones)
}

fn oxygen_rating(readings: &Vec<i32>) -> i32 {
    let mut shift = 11;
    let mut filtered = readings.clone();
    while filtered.len() > 1 {
        let mut next = vec![];
        let bit_counts = most_common_bit(&filtered, shift);
        let bit = if bit_counts.0 > bit_counts.1 { 0 } else { 1 };
        for reading in &filtered {
            if (get_bit(*reading, shift) && bit == 1) || (!get_bit(*reading, shift) && bit == 0) {
                next.push(*reading);
            }
        }
        filtered = next;
        shift -= 1;
    }
    filtered[0]
}

fn scrubber_rating(readings: &Vec<i32>) -> i32 {
    let mut shift = 11;
    let mut filtered = readings.clone();
    while filtered.len() > 1 {
        let mut next = vec![];
        let bit_counts = most_common_bit(&filtered, shift);
        let bit = if bit_counts.0 <= bit_counts.1 { 0 } else { 1 };
        for reading in &filtered {
            if (get_bit(*reading, shift) && bit == 1) || (!get_bit(*reading, shift) && bit == 0) {
                next.push(*reading);
            }
        }
        filtered = next;
        shift -= 1;
    }
    filtered[0]
}

fn part2() {
    let readings = get_readings();

    let og_rating = oxygen_rating(&readings);
    let c02_rating = scrubber_rating(&readings);
    println!("Life support rating: {}", og_rating * c02_rating);
}

fn main() {   
    part1();
    part2();
}