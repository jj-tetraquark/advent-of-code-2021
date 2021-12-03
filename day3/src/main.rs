use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

fn get_file_contents(filename : &String) -> Vec<Vec<u32>> {
    let file = fs::File::open(filename).expect("cannot open file");
    io::BufReader::new(file).lines()
                            .filter_map(|line| line.ok()?.parse::<String>().ok())
                            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
                            .collect()
}


fn get_most_common_value(input : &Vec<Vec<u32>>, index : usize) -> u32 {
    let total = input.iter().fold(0, |acc, x| acc + x[index]);
    if input.len() % 2 == 0 {
        if total >= (input.len() as u32)/2 { 
            return 1;
        } else { 
            return 0;
        }
    }
    if total > (input.len() as u32)/2 { 1 } else { 0 }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");
    
    let input = get_file_contents(&args[1]);
    let bin_size = input[0].len();
    let sums = input.iter()
                    .fold(vec![0; bin_size], |acc, x| acc.iter()
                                                         .zip(x.iter())
                                                         .map(|(x1, x2)| x1 + x2)
                                                         .collect());

    let input_length = input.len() as u32;
    let gamma = sums.iter().map(|&i| if i > input_length/2 {"1"} else {"0"}).collect::<String>();
    let epsilon = gamma.chars().map(|c| if c == '1' {"0"} else {"1"}).collect::<String>();

    let gamma_int = u32::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon_int = u32::from_str_radix(epsilon.as_str(), 2).unwrap();

    println!("{} * {} = {}", gamma_int, epsilon_int, gamma_int * epsilon_int);

    let o2_gen_value = {
        let mut o2_gen = input.clone();
        for index in 0..input_length as usize {
            let most_common_value = get_most_common_value(&o2_gen, index);
            o2_gen = o2_gen.into_iter().filter(|x| x[index] == most_common_value).collect();
            if o2_gen.len() == 1 {
                break;
            }
        }
        u32::from_str_radix(o2_gen[0].iter().map(u32::to_string).collect::<String>().as_str(), 2).unwrap()
    };

    let co2_scrub_value = {
        let mut co2_scrub = input.clone();
        for index in 0..input_length as usize {
            let least_common_value = (get_most_common_value(&co2_scrub, index) == 0) as u32;
            co2_scrub = co2_scrub.into_iter().filter(|x| x[index] == least_common_value).collect();
            if co2_scrub.len() == 1 {
                break;
            }
        }
        u32::from_str_radix(co2_scrub[0].iter().map(u32::to_string).collect::<String>().as_str(), 2).unwrap()
    };
    
    println!("o2_gen_value: {}", o2_gen_value);
    println!("co2_scrub_value: {}", co2_scrub_value);
    println!("life support: {}", o2_gen_value * co2_scrub_value);
}
