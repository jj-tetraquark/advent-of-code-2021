use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

fn get_file_contents(filename : &String) -> Vec<String> {
    let file = fs::File::open(filename).expect("cannot open file");
    io::BufReader::new(file).lines()
                            .filter_map(|line| line.ok()?.parse::<String>().ok())
                            .collect()
}

fn is_bingo(indices : &[usize]) -> bool {
    for row in 0..5 {
        if (row..row+5).all(|i| indices.contains(&i)) {
            return true;
        }
    }
    for col in 0..5 {
        if (col..col+21).step_by(5).all(|i| indices.contains(&i)) {
            return true;
        }
    }
    false
}

fn board_score(board : &Vec<u32>, marked_values : &Vec<usize>, final_number : &u32) -> u32 {
    let unmatched_sum :u32 = board.iter()
                         .enumerate()
                         .filter_map(|(idx, num)| 
                                     if marked_values.contains(&idx) {
                                         None
                                     }
                                     else {
                                         Some(num)
                                     }).sum();
    return unmatched_sum * final_number;
}

fn part1(numbers : &Vec<u32>, boards : &Vec<Vec<u32>>) -> u32 {
    let mut matches : Vec<Vec<usize>> = vec![vec![]; boards.len()];
    let mut numbers_it = numbers.iter();
    let (winning_board, final_number) = loop {
        if let Some(number) = numbers_it.next() {
            boards.iter()
                  .zip(&mut matches)
                  .for_each(|(board, indexes)| 
                            match board.iter().position(|x| x == number) {
                                Some(index) => indexes.push(index),
                                None => (),
                            });
            match matches.iter().position(|indices| is_bingo(indices)) {
                Some(board_index) => break (board_index, number),
                None => (),
            }
        }
    };
    println!("winning_board: {:?}, {}", winning_board, final_number);
    return board_score(&boards[winning_board], &matches[winning_board], final_number);
}

fn part2(numbers : &Vec<u32>, boards : &mut Vec<Vec<u32>>) -> u32 {
    let mut matches : Vec<Vec<usize>> = vec![vec![]; boards.len()];
    let mut numbers_it = numbers.iter().peekable();

    loop {
        if let Some(number) = numbers_it.next() {
            println!("Number: {}", number);
            boards.iter()
                  .zip(&mut matches)
                  .for_each(|(board, indexes)| 
                            match board.iter().position(|x| x == number) {
                                Some(index) => indexes.push(index),
                                None => (),
                            });

            while let Some(winner) = matches.iter().position(|indices| is_bingo(indices)) {
                if boards.len() > 1 && numbers_it.peek() != None {
                    println!("Removing {}", winner);
                    println!("match length: {}", matches[winner].len());
                    matches.remove(winner);
                    boards.remove(winner);
                    println!("{} boards remain", boards.len());
                }
                else {
                    println!("match length: {}", matches[winner].len());
                    println!("{} boards remain", boards.len());
                    return board_score(&boards[winner], &matches[winner], number);
                }
            }
        }
        else {
            panic!();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");
    
    let input = get_file_contents(&args[1]);
    let numbers: Vec<u32> = input[0].split(',').map(|x| x.parse::<u32>().unwrap()).collect();

    let boards: Vec<Vec<u32>> = input.iter()
                      .skip(1)
                      .map(|line| 
                           line.split_whitespace()
                               .filter_map(|x| x.parse::<u32>().ok()).collect())
                      .filter(|line : &Vec<u32>| line.len() == 5)
                      .collect::<Vec<Vec<u32>>>()
                      .windows(5)
                      .step_by(5)
                      .map(|row| row.concat())
                      .collect();

    println!("result: {}", part1(&numbers, &boards));
    println!("result: {}", part2(&numbers, &mut boards.clone()));
}
