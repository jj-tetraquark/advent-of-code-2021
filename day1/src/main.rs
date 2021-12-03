use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

fn get_file_contents(filename : &String) -> Vec<i32>
{
    let file = fs::File::open(filename).expect("cannot open file");
    io::BufReader::new(file).lines()
                            .filter_map(|number| number.ok()?.parse::<i32>().ok())
                            .collect()
}

fn part_1(values : &Vec<i32>) {
    let result = values.windows(2)
                       .fold(0, |acc , x| acc + if x[1] > x[0] { 1 } else { 0 });
    println!("{}", result);
}

fn part_2(values : &Vec<i32>) {
    let window_sums:Vec<i32> = values.windows(3)
                                     .map(|x| x.iter().sum()).collect();
    part_1(&window_sums);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");
    
    let values = get_file_contents(&args[1]);
    println!("Part 1");
    part_1(&values); 
    println!("Part 2");
    part_2(&values);
}
