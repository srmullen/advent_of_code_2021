fn part1() {
    // one = 2
    // four = 4
    // seven = 3
    // eight = 7
    let readings: Vec<(Vec<&str>, Vec<&str>)> = include_str!("../input/input.txt")
        .lines()
        .map(|line| {
            let (input, output) = line.split_once(" | ").unwrap();
            (input.split(" ").collect(), output.split(" ").collect())
        }).collect();

        let count: usize = readings
            .iter()
            .map(|(_, output)| output.iter().filter(|r| {
                let c = r.len();
                c == 2 || c == 4 || c == 3 || c == 7
            }))
            .map(|out| {out.count()})
            .sum();

    println!("Count: {}", count);
}

fn main() {
    part1();
}

