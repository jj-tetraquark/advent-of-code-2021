use std::env;
use std::fs;

fn get_file_contents(filename : &String) -> Vec<i32> {
   fs::read_to_string(filename).unwrap()
                               .trim()
                               .split(',')
                               .filter_map(|x| x.parse::<i32>().ok()).collect() 
}

fn fuel_cost(distance : i32) -> i32 { 
    distance * (1 + distance)/2
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");
    
    let crab_positions = get_file_contents(&args[1]);

    let min_pos = crab_positions.iter().min().unwrap();
    let max_pos = crab_positions.iter().max().unwrap();

    // Destination D, start position X, movement m
    // D will lie between Xmin and Xmax
    // Xi + mi = D
    // D - Xi = mi
    let alignment_costs : Vec<i32> = 
        (*min_pos..=*max_pos).map(|d : i32| 
                                  crab_positions.iter()
                                                .map(|x| (d - x).abs())
                                                .sum())
                             .collect();

    println!("Part1 Min movement: {}", alignment_costs.iter().min().unwrap());

    let alignment_costs2 : Vec<i32> = 
        (*min_pos..=*max_pos).map(|d : i32| 
                                   crab_positions.iter()
                                                 .map(|x| fuel_cost((d - x).abs()))
                                                 .sum())
                             .collect();

    println!("Part2 Min movement: {}", alignment_costs2.iter().min().unwrap());
}
