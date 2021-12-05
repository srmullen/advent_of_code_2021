use std::cmp;

fn main() {
    let mut grid = vec![vec![0; 1000]; 1000];

    let lines: Vec<(Vec<usize>, Vec<usize>)> = include_str!("../input/input.txt")
        .lines()
        .map(|s| {
            let (c1, c2) = s.split_once(" -> ").unwrap();
            let p1 = c1.split(",").map(|n| n.parse::<usize>().unwrap()).collect();
            let p2 = c2.split(",").map(|n| n.parse::<usize>().unwrap()).collect();
            (p1, p2)
        }).collect();

    for line in lines {
        let (p1, p2) = line;
        let minx = cmp::min(p1[0], p2[0]);
        let maxx = cmp::max(p1[0], p2[0]);
        if p1[0] == p2[0] || p1[1] == p2[1] {
            for x in minx..=maxx {
                for y in cmp::min(p1[1], p2[1])..=cmp::max(p1[1], p2[1]) {
                    grid[x][y] = grid[x][y] + 1;
                }
            }
        } else {
            let steps = maxx - minx;
            for step in 0..=steps {
                let x = if p1[0] < p2[0] {
                    p1[0] + step
                } else {
                    p1[0] - step
                };
                let y = if p1[1] < p2[1] {
                    p1[1] + step
                } else {
                    p1[1] - step
                };
                grid[x][y] = grid[x][y] + 1;
            }
        }
    }

    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y] > 1 {
                count += 1;
            }
        }
    }

    println!("Count: {}", count);
}
