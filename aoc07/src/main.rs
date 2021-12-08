use std::collections::HashMap;

fn part1() {
    let crabs: Vec<i32> = include_str!("../input/input.txt")
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let mut fuel: HashMap<i32, i32> = HashMap::new();
    for position in min..=max {
        fuel.insert(position, crabs.iter().map(|crab| (position - crab).abs()).sum());
    }
    
    let mut min = i32::MAX;
    let mut position = -1;
    for (k, v) in fuel.iter() {
        if v <= &min {
            position = *k;
            min = *v;
        }
    }
    println!("{}", position);
    dbg!(fuel.get(&337).unwrap());
}

fn crab_fuel(crab: i32, position: i32) -> i32 {
    let dist = (position - crab).abs();
    (dist.pow(2) + dist ) / 2
}

fn part2() {
    let crabs: Vec<i32> = include_str!("../input/input.txt")
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let mut fuel: HashMap<i32, i32> = HashMap::new();
    for position in min..=max {
        fuel.insert(position, crabs.iter().map(|crab| crab_fuel(*crab, position)).sum());
    }
    
    let mut min = i32::MAX;
    let mut position = -1;
    for (k, v) in fuel.iter() {
        if v <= &min {
            position = *k;
            min = *v;
        }
    }
    println!("{}", min);
}

fn main() {
    // part1();
    part2();
}

#[test]
fn test_crab_fuel() {
    assert_eq!(crab_fuel(5, 5), 0);
    assert_eq!(crab_fuel(5, 6), 1);
    assert_eq!(crab_fuel(5, 7), 3);
}
