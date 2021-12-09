fn get_readings() -> Vec<(Vec<&'static str>, Vec<&'static str>)> {
    include_str!("../input/input.txt")
        .lines()
        .map(|line| {
            let (input, output) = line.split_once(" | ").unwrap();
            (input.split(" ").collect(), output.split(" ").collect())
        }).collect()
}

fn part1() {
    let count: usize = get_readings()
        .iter()
        .map(|(_, output)| output.iter().filter(|r| {
            let c = r.len();
            c == 2 || c == 4 || c == 3 || c == 7
        }))
        .map(|out| {out.count()})
        .sum();

    println!("Count: {}", count);
}

fn part2() {
    use std::collections::{HashSet};

    fn to_digit(segments: &str) -> HashSet<char> {
        segments.chars().fold(HashSet::new(), |mut set, c| {
            set.insert(c);
            set
        })
    }

    let sum: u32 = get_readings()
        .iter()
        .map(|(input, output)| -> (Vec<HashSet<char>>, Vec<HashSet<char>>) {
            (
                input.iter().map(|segments| to_digit(segments)).collect(), 
                output.iter().map(|segments| to_digit(segments)).collect()
            )
        }).map(|(mut input, output)| {
            let one = input.remove(input.iter().position(|h| h.len() == 2).unwrap());
            let four = input.remove(input.iter().position(|h| h.len() == 4).unwrap());
            let seven = input.remove(input.iter().position(|h| h.len() == 3).unwrap());
            let eight = input.remove(input.iter().position(|h| h.len() == 7).unwrap());
            // 6 segment nums
            let six = input.remove(input.iter().position(|h| h.len() == 6 && h.intersection(&seven).count() == 2).unwrap());
            let nine = input.remove(input.iter().position(|h| h.len() == 6 && h.intersection(&four).count() == 4).unwrap());
            let zero = input.remove(input.iter().position(|h| h.len() == 6).unwrap());
            // 5 segment nums
            let three = input.remove(input.iter().position(|h| h.len() == 5 && h.intersection(&seven).count() == 3).unwrap());
            let five = input.remove(input.iter().position(|h| h.len() == 5 && h.intersection(&four).count() == 3).unwrap());
            let two = input.remove(input.iter().position(|h| h.len() == 5).unwrap());

            let mut out = String::new();
            for n in output {
                if n == zero { out.push('0') }
                if n == one { out.push('1') }
                if n == two { out.push('2') }
                if n == three { out.push('3') }
                if n == four { out.push('4') }
                if n == five { out.push('5') }
                if n == six { out.push('6') }
                if n == seven { out.push('7') }
                if n == eight { out.push('8') }
                if n == nine { out.push('9') }
            }
            out.parse::<u32>().unwrap()
        }).sum();

    println!("Sum: {}", sum);
}

fn main() {
    part1();
    part2();
}

