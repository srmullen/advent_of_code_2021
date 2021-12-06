use std::collections::HashMap;

fn day(fishes: &mut Vec<u8>) {
    let mut caviar = vec![];
    for i in 0..fishes.len() {
        let fish = fishes[i];
        if fish == 0 {
            caviar.push(8);
            fishes[i] = 6;
        } else {
            fishes[i] = fish - 1;
        }
    }
    fishes.append(&mut caviar);
}

fn part1() {
    let mut fish: Vec<u8> = include_str!("../input/input.txt")
        .split(",")
        .map(|s| s.parse::<u8>().unwrap()).collect();
    for i in 0..80 {
        println!("Day: {}", i);
        day(&mut fish);
    }
    println!("Fishes: {}", fish.len());
}

fn part2() {
    let mut cache = HashMap::new();

    let fish = include_str!("../input/input.txt")
        .split(",")
        .map(|s| s.parse::<u8>().unwrap());
    
    let school: u64 = fish.map(|f| {
        memo_fish(&mut cache, f, 256)
    }).sum();

    println!("School size: {}", school);
}

fn memo_fish(cache: &mut HashMap<(u8, u64), u64>, fish: u8, days: u64) -> u64 {
    match cache.get_key_value(&(fish, days)) {
        Some((_, res)) => *res,
        None => {
            if days == 0 {
                let result = 1;
                cache.insert((fish, days), 1);
                return result;
            }
        
            if fish == 0 {
                let result = memo_fish(cache, 6, days - 1) + memo_fish(cache, 8, days - 1);
                cache.insert((fish, days), result);
                return result;
            }
        
            let result = memo_fish(cache, fish - 1, days - 1);
            cache.insert((fish, days), result);
            result
        }
    }
}

fn main() {
    part1();
    part2();
}