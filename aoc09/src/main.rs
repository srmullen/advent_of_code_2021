use std::collections::{HashSet, VecDeque};

fn get_adjacent(heights: &Vec<Vec<i32>>, x: usize, y: usize) -> Vec<i32> {
    let mut adjacent = vec!{};
    if x != 0 {
        adjacent.push(heights[x-1][y]);
    }
    if x != heights.len()-1 {
        adjacent.push(heights[x+1][y]);
    }
    if y != 0 {
        adjacent.push(heights[x][y-1]);
    }
    if y != heights[0].len()-1 {
        adjacent.push(heights[x][y+1]);
    }
    adjacent
}

fn get_adjacent2(heights: &Vec<Vec<i32>>, x: usize, y: usize) -> Vec<(i32, (usize, usize))> {
    let mut adjacent = vec!{};
    if x != 0 {
        adjacent.push((heights[x-1][y], (x-1, y)));
    }
    if x != heights.len()-1 {
        adjacent.push((heights[x+1][y], (x + 1, y)));
    }
    if y != 0 {
        adjacent.push((heights[x][y-1], (x, y-1)));
    }
    if y != heights[0].len()-1 {
        adjacent.push((heights[x][y+1], (x, y+1)));
    }
    adjacent
}

fn low_points(heights: &Vec<Vec<i32>>) -> Vec<(i32, (usize, usize))> {
    let mut lows = vec![];
    for x in 0..heights.len() {
        for y in 0..heights[x].len() {
            let depth = heights[x][y];
            let adjacent = get_adjacent(&heights, x, y);
            if adjacent.iter().all(|n| n > &depth) {
                lows.push((depth, (x, y)));
            }
        }
    }
    lows
}

// BFS
fn get_basin(heights: &Vec<Vec<i32>>, low: &(i32, (usize, usize))) -> HashSet<(usize, usize)> {
    let mut basin = HashSet::new();
    let mut stack = VecDeque::from([low.1]);
    while stack.len() > 0 {
        match stack.pop_front() {
            Some(pos) => {
                let val = heights[pos.0][pos.1];
                if !basin.contains(&pos) && val < 9 {
                    basin.insert(pos);
                    let adjacents = get_adjacent2(heights, pos.0, pos.1);
                    for adj in adjacents {
                        stack.push_back(adj.1);
                    }
                }
            },
            None => {}
        }
    }    
    basin
}

fn load_heigts() -> Vec<Vec<i32>> {
    include_str!("../input/input.txt")
        .lines()
        .map(|line| {
            line.split("")
                .filter(|c| c.len() == 1)
                .map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>()
        }).collect()
}

fn part1() {
    let heights = load_heigts();

    let risk = low_points(&heights).iter().map(|(h, _)| h + 1).sum::<i32>();
    println!("Risk: {}", risk);
}

fn part2() {
    let heights = load_heigts();

    let lows = low_points(&heights);
    let mut basins: Vec<usize> = lows.iter().map(|low| {
        get_basin(&heights, low).len()
    }).collect();

    basins.sort();
    basins.reverse();

    dbg!(basins[0] * basins[1] * basins[2]);
}

fn main() {
    // part1();
    part2();
}