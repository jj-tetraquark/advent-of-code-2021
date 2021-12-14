use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use itertools::Itertools;
       
type Pair = (char, char);
type SeqMap = HashMap<Pair, char>;
type CreationMap = HashMap<Pair, (Pair, Pair)>;

fn get_file_contents(filename : &String) -> (Vec<char>, SeqMap) {
    let file = fs::File::open(filename).expect("cannot open file");
    let contents: Vec<_> = io::BufReader::new(file).lines()
                            .filter_map(|line| line.ok()?.parse::<String>().ok())
                            .collect();
    
    let start : Vec<_> = contents[0].chars().collect();

    let map = contents.iter().skip(2).fold(HashMap::new(), |mut map, line| {
        let mapping : Vec<&str> = line.split(" -> ").collect();
            map.insert(mapping[0].chars().next_tuple().unwrap(), 
                       mapping[1].chars().next().unwrap());
        map
    });
    
    return (start, map);
}

fn make_creation_map(seq_map: &SeqMap) -> CreationMap {
    seq_map.iter().map(|(pair, letter)| (*pair, ((pair.0, *letter), (*letter, pair.1)))).collect()
}

fn step(sequence : &Vec<char>, map : &SeqMap) -> Vec<char> {
    let mut res = sequence
        .windows(2)
        .fold(Vec::new(), |mut seq, pair| {
            seq.push(pair[0]);
            seq.push(map[&(pair[0], pair[1])]);
            seq
        });
    res.push(*sequence.last().unwrap());
    res
}

fn naiive_approach(start: &Vec<char>, map: &SeqMap, count :u32) -> u32 {
    let mut seq = start.clone();
    for i in 0..count {
        println!("{}", i);
        seq = step(&seq, &map);
    }

    let hist = seq.iter().fold(HashMap::new(), |mut hist, el| {
        *hist.entry(el).or_insert(0) += 1;
        hist
    });

    println!("{:?}", hist);

    let max = hist.values().max().unwrap();
    let min = hist.values().min().unwrap();

    return max - min;
}

fn get_counts(seq: &Vec<char>, map: &SeqMap, hist: &mut HashMap<char, u32>, count: u32) {
    if count == 0 {
        return;
    }

    seq.windows(2).for_each(|section| {
        let new_el = map[&(section[0], section[1])];
        *hist.entry(new_el).or_insert(0) += 1;
        get_counts(&vec![section[0], new_el, section[1]], map, hist, count - 1)
    })
}

fn get_solution_recursive(seq: &Vec<char>, map: &SeqMap, count: u32) -> u32 {
    let mut hist = seq.iter().fold(HashMap::new(), |mut hist, c| {
        *hist.entry(*c).or_insert(0) += 1;
        hist
    });
    get_counts(&seq, &map, &mut hist, count);
    let min = hist.values().min().unwrap();
    let max = hist.values().max().unwrap();
    return max - min;
}

fn get_solution(seq: &Vec<char>, map: &SeqMap, count: u32) -> u64 {
    let creation_map = make_creation_map(&map);
    let mut letter_hist = seq.iter().fold(HashMap::new(), |mut hist, c| {
        *hist.entry(*c).or_insert(0 as u64) += 1;
        hist
    });

    let mut pair_hist = seq.windows(2).fold(HashMap::new(), |mut hist, pair| {
        *hist.entry((pair[0], pair[1])).or_insert(0 as u64) += 1;
        hist
    });

    for _ in 0..count {
        pair_hist = pair_hist.iter().fold(HashMap::new(), |mut hist, (pair, count)| {
            let created_pair = creation_map[pair];
            *letter_hist.entry(map[pair]).or_insert(0) += count;
            *hist.entry(created_pair.0).or_insert(0) += count;
            *hist.entry(created_pair.1).or_insert(0) += count;
            hist
        });
    }
    let min = letter_hist.values().min().unwrap();
    let max = letter_hist.values().max().unwrap();
    return max - min;
}

fn main() {
    let args : Vec<_> = env::args().collect();
    assert!(args.len() == 2, "Specify a file!");
    let (start, map) = get_file_contents(&args[1]);

    println!("start: {:?}, mapping:\n{:?}", start, map);

    println!("part1: {}", get_solution_recursive(&start, &map, 10));
    println!("part 2: {}", get_solution(&start, &map, 40));

}
