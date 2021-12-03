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

fn parse_instruction(instruction : &String) -> (i32, i32) {
    let parts : Vec<&str> = instruction.split_whitespace().collect();
    let mag = parts[1].parse::<i32>().unwrap();

    match parts[0] {
        "forward" => (mag, 0),
        "up" => (0, -mag),
        "down" => (0, mag),
        _ => panic!(),
    }
}

fn part1(instructions : &Vec<String>) {
    let destination = instructions.iter()
                                  .fold((0, 0), |coords, instruction| {
                                      let action = parse_instruction(instruction);
                                      (coords.0 + action.0, coords.1 + action.1)
                                  });

    println!("Part1");
    println!("solution: {}", (destination.0 * destination.1).abs());
}


fn part2(instructions : &Vec<String>) {
    let destination = instructions.iter()
                                  .fold((0, 0, 0), |coords, instruction| {
                                      let action = parse_instruction(instruction);
                                      (coords.0 + action.0,             // forward
                                       coords.1 + action.0 * coords.2,  // depth 
                                       coords.2 + action.1)             // aim
                                  });

    println!("Part2");
    println!("solution: {}", (destination.0 * destination.1).abs());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");
    
    let instructions = get_file_contents(&args[1]);

    part1(&instructions);

    part2(&instructions);
}
