use std::fs;
use std::iter::Iterator;

#[derive(Debug)]
struct Board {
    board: Vec<i32>
}

impl Board {
    fn new(board: Vec<i32>) -> Board {
        if board.len() != 25 {
            panic!("Boards not parsed correctly");
        }
        Board { board }
    }

    fn is_winner(&self, numbers: &Vec<i32>) -> bool {
        // check columns
        for x in 0..5 {
            let mut winner = true;
            for y in 0..5 {
                if !numbers.contains(&self.at(x, y)) {
                    winner = false;
                    break
                }
            }
            if winner {
                return winner;
            }
        }
        // check rows
        for y in 0..5 {
            let mut winner = true;
            for x in 0..5 {
                if !numbers.contains(&self.at(x, y)) {
                    winner = false;
                    break
                }
            }
            if winner {
                return winner;
            }
        }
        // check diagonals
        let winner = vec![
            &self.at(0, 0),
            &self.at(1, 1),
            &self.at(2, 2),
            &self.at(3, 3),
            &self.at(4, 4),
        ].iter().fold(true, |win, n| win && numbers.contains(n));
        if winner {
            return winner;
        }

        let winner = vec![
            &self.at(4, 0),
            &self.at(3, 1),
            &self.at(2, 2),
            &self.at(1, 3),
            &self.at(0, 4),
        ].iter().fold(true, |win, n| win && numbers.contains(n));
        if winner {
            return winner;
        }
        false
    }

    fn at(&self, x: i32, y: i32) -> i32 {
        self.board[(x + (y * 5)) as usize]
    }

    fn calc_score(&self, numbers: Vec<i32>) -> i32 {
        // dbg!(&self.board);
        // dbg!(&numbers);
        
        let mut uncalled = vec![];
        for n in &self.board  {
            if !numbers.contains(n) {
                uncalled.push(n);
            }
        }
        let mut sum = 0;
        for n in &uncalled {
            sum += *n;
        }

        sum * numbers.last().unwrap()
    }
}

fn create_boards(rows: Vec<&str>) -> Vec<Board> {
    let mut groups: Vec<Vec<i32>> = vec![];
    let mut group: Vec<Vec<i32>> = vec![];
    for row in rows {
        if row.len() > 0 {
            group.push(row.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect());
        } else {
            groups.push(group.concat());
            group = vec![];
        }
    }
    groups.iter().map(|numbers| Board::new(numbers.clone())).collect()
}

fn part1() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let mut input = contents.split("\n").collect::<Vec<&str>>();

    let mut numbers: Vec<i32> = input[0].split(",").map(|n| n.parse::<i32>().unwrap()).collect();
    numbers.reverse();
    let rows = input.split_off(2);

    let boards = create_boards(rows);

    let mut winning_board: Option<usize> = None;
    let mut selected = vec![
        numbers.pop().unwrap(),
        numbers.pop().unwrap(),
        numbers.pop().unwrap(),
        numbers.pop().unwrap(),
        numbers.pop().unwrap(),
    ];
    
    while let None = winning_board {
        for i in 0..boards.len() {
            let board = &boards[i];
            if board.is_winner(&selected) {
                winning_board = Some(i);
                break
            }
        }
        if let None = winning_board {
            selected.push(numbers.pop().unwrap());
        }
    }
    let winning_board = &boards[winning_board.unwrap()];
    let score = winning_board.calc_score(selected);
    println!("Score: {}", score);
}

fn part2() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let mut input = contents.split("\n").collect::<Vec<&str>>();

    let mut numbers: Vec<i32> = input[0].split(",").map(|n| n.parse::<i32>().unwrap()).collect();
    numbers.reverse();
    let rows = input.split_off(2);

    let boards = create_boards(rows);

    // let mut winning_board: Option<usize> = None;
    let mut selected = vec![
        numbers.pop().unwrap(),
        numbers.pop().unwrap(),
        numbers.pop().unwrap(),
        numbers.pop().unwrap(),
    ];

    let mut bingos: Vec<usize> = vec![0; boards.len()];
    
    let mut last_winner: Option<usize> = None;
    while bingos.iter().sum::<usize>() < bingos.len() {
        selected.push(numbers.pop().unwrap());
        for i in 0..boards.len() {
            let board = &boards[i];
            if bingos[i] == 0 && board.is_winner(&selected) {
                bingos[i] = 1;
                last_winner = Some(i);
            }
        }        
    }
    let loser = &boards[last_winner.unwrap()];
    dbg!(&selected);
    dbg!(loser);
    dbg!(loser.calc_score(selected));
}

fn main() {
    part1();
    part2();
}
