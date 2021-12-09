use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

type Entry = (Vec<String>, Vec<String>);

fn get_file_contents(filename : &String) -> Vec<Entry> {
    let file = fs::File::open(filename).expect("cannot open file");
    io::BufReader::new(file).lines()
                            .filter_map(|line| line.ok()?.parse::<String>().ok())
                            .map(|line| {
                                let mut parts = line.split('|');
                                (parts.next().unwrap().split_whitespace().map(str::to_string).collect(), 
                                 parts.next().unwrap().split_whitespace().map(str::to_string).collect())
                            })
                            .collect()
}

fn part_1(entries : &Vec<Entry>) -> u32 {
    entries.iter().fold(0, |acc, entry| 
                     {
                        acc + entry.1.iter().fold(0, |acc, digit_str|
                                                  {
                                                      let len = digit_str.len();
                                                      acc + if len <= 4 || len == 7 { 1 } else {0}
                                                  })
                     })
}

// 1, 4, 7, 8 we get for free
// convert letters to binary
// in the example:
//    a b c d e f g
// 0: 1 1 1 0 1 1 1 
// 1: 0 0 1 0 0 1 0 
// 2: 1 0 1 1 1 0 1 
// 3: 1 0 1 1 0 1 1
// 4: 0 1 1 1 0 1 0
// 5: 1 1 0 1 0 1 1
// 6: 1 1 0 1 1 1 1
// 7: 1 0 1 0 0 1 0
// 8: 1 1 1 1 1 1 1
// 9: 1 1 1 1 0 1 1
// T: 8 6 8 7 4 9 7

// Find the most common bit (9), the only one with the bit missing is 2
// Find the least common bit (4), 8 AND NOT this bit is 9
// Find the bit with count 6, 9 AND NOT this bit is 3
// 0, 5, 6 remain - 5 is the one with 5 active bits
// 5 OR most common bit is 6
// 0 remains


fn convert_to_binary_rep(input: &String) -> u8 {
    input.chars().fold(0,|acc, x| match x
                       {
                         'a' => acc | 0b01000000,
                         'b' => acc | 0b00100000,
                         'c' => acc | 0b00010000,
                         'd' => acc | 0b00001000,
                         'e' => acc | 0b00000100,
                         'f' => acc | 0b00000010,
                         'g' => acc | 0b00000001,
                         _ => panic!()
                       })
}

fn get_bit_counts(input: &Vec<u8>, bit_letters : &Vec<u8>) -> Vec<usize> {
    bit_letters.iter()
               .map(|letter| input.iter().filter(|comp| letter & *comp > 0).count() )
               .collect()
}

fn count_set_bits(mut number : u8) -> i32 {
    let mut count: i32 = 0;
    while number > 0 { 
        count += (number & 1) as i32;
        number >>= 1;
    }
    count
}

fn determine_mapping(sequence: &Vec<String>) -> HashMap<u8, i32> {
    let mut sequence_as_binary : Vec<u8> = sequence.iter()
                                               .map(|input| convert_to_binary_rep(input))
                                               .collect();
    let mut map = HashMap::new();
    sequence_as_binary.iter()
                      .for_each(|input| match count_set_bits(*input) {
                        2 => { map.insert(*input, 1);},
                        4 => { map.insert(*input, 4);},
                        3 => { map.insert(*input, 7);},
                        7 => { map.insert(*input, 8);},
                        _ => ()
            });

    let bit_letters : Vec<u8> = "abcdefg".chars()
                                         .map(|c| convert_to_binary_rep(&c.to_string()))
                                         .collect();

    let bit_counts = get_bit_counts(&sequence_as_binary, &bit_letters);

    // Find the most common bit (9), the only one with the bit missing is 2
    let most_common_bit = bit_letters[bit_counts.iter().position(|x| *x == 9).unwrap()];
    map.insert(*sequence_as_binary.iter().find(|&number| number & most_common_bit == 0).unwrap(), 2);
                      
    // Find the least common bit (4), 8 AND NOT this bit is 9
    let least_common_bit = bit_letters[bit_counts.iter().position(|x| *x == 4).unwrap()];
    let eight : &u8 = map.iter()
                         .find_map(|(key, &value)| if value == 8 { Some(key) } else { None }).unwrap();

    let nine = eight & !least_common_bit;
    map.insert(nine, 9);
    
    // Find the bit with count 6, 9 not this bit is 3
    let middle_bit = bit_letters[bit_counts.iter().position(|x| *x == 6).unwrap()];
    let three = nine & !middle_bit;
    map.insert(three, 3);

    // 0, 5, 6 remain - 5 is the one with 5 active bits
    sequence_as_binary.retain(|x| !map.contains_key(x));
    assert!(sequence_as_binary.len() == 3);

    let five = sequence_as_binary.iter().find(|&x| count_set_bits(*x) == 5).unwrap();
    map.insert(*five, 5);

    // 5 OR least common bit is 6
    let six = five | least_common_bit;
    map.insert(six, 6);

    // 0 remains
    sequence_as_binary.retain(|x| !map.contains_key(x));
    assert!(sequence_as_binary.len() == 1);

    map.insert(sequence_as_binary[0], 0);

    return map;
}

fn main() {
    let args : Vec<String> = env::args().collect();

    let entries = get_file_contents(&args[1]);

    println!("part1: {}", part_1(&entries));

    let part2 = entries.iter()
           .fold(0, |total, (input, output)|
                 {
                    let mapping = determine_mapping(input);
                    total + output.iter().fold(0, |value, digit| 
                                               value * 10 + mapping[&convert_to_binary_rep(digit)])
                });

    println!("part2: {:?}", part2);

}
