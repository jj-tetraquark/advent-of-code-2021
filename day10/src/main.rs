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

fn find_error_position(line : &String) -> (i32, Vec<char>) {
    let mut closing_symbols = Vec::new();
    for (i, c) in line.chars().enumerate() {
        let idx = i as i32;
        match c {
            '(' => closing_symbols.push(')'),
            '{' => closing_symbols.push('}'),
            '[' => closing_symbols.push(']'),
            '<' => closing_symbols.push('>'),
            ')' | '}' | ']' | '>' => if closing_symbols.pop() != Some(c) { 
                                        return (idx, Vec::new()); 
                                    },
            _ => panic!("unrecognised symbol {}", c)
        }
    }
    closing_symbols.reverse();
    return (-(closing_symbols.len() as i32), closing_symbols);
}

fn get_error_score(line : &String, idx : i32) -> u32 {
    if idx <= 0 {
        return 0;
    }

    let error_char = line.as_bytes()[idx as usize] as char;
    match error_char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!()
    }
}

fn get_autocomplete_score(completing_symbols : &Vec<char>) -> u64 {
    completing_symbols.iter()
                      .fold(0, |score, symbol|
                                score * 5 + match symbol {
                                    ')' => 1,
                                    ']' => 2,
                                    '}' => 3,
                                    '>' => 4,
                                    _ => panic!()
                                })
}

fn main() {
    let args : Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");
    
    let input = get_file_contents(&args[1]);
    
    let error_score = input.iter()
                            .fold(0, |score, line| {
                                  let (idx,_) = find_error_position(&line);
                                  score + get_error_score(&line, idx)
                            });

    println!("part1 score: {}", error_score);

    let mut autocomplete_scores : Vec<u64> = 
        input.iter()
             .filter_map(|line| {
                 let (idx, complete_symbols) = find_error_position(&line);
                 if idx >= 0 {
                     None
                 }
                 else {
                     Some(get_autocomplete_score(&complete_symbols))
                  }
             })
             .collect();

    autocomplete_scores.sort();
    println!("part2: {}", autocomplete_scores[autocomplete_scores.len()/2]);
}
