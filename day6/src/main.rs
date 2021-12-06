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

fn final_population(days : u32, mut state : Vec<u64>) -> u64 {
    for _ in 0..days {
        let lanternfish_births = state[0];
        state.rotate_left(1);
        state[6] += lanternfish_births;
    }
    return state.iter().sum::<u64>();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");
    
    let input = get_file_contents(&args[1]);
    assert!(input.len() == 1);

    let init_state = input[0].split(',').map(|x| x.parse::<usize>().unwrap());
    let mut state = vec![0; 9]; // maximum of nine days
    init_state.for_each(|day| state[day] += 1);

    println!("Total lanternfish afer {} days : {}", 80, final_population(80, state.clone()));
    println!("Total lanternfish afer {} days : {}", 256, final_population(256, state.clone()));
}
