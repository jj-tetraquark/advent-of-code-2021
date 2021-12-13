use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::collections::HashSet;
use regex::Regex;

type Dot = (u32, u32);
type Line = (char, u32);

fn get_file_contents(filename : &String) -> (HashSet<Dot>, Vec<Line>) {
    let mut dots = HashSet::new();
    let mut lines = Vec::new();
    let line_regex = Regex::new(r"fold along ([xy])=([0-9]+)").unwrap();

    let file = fs::File::open(filename).expect("cannot open file");
    io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok()?.parse::<String>().ok())
        .for_each(|line| {
            let dot_parts = line.split(',').collect::<Vec<_>>();
            if dot_parts.len() == 2 {
                dots.insert((dot_parts[0].parse::<u32>().unwrap(), 
                             dot_parts[1].parse::<u32>().unwrap()));
            }
            else if let Some(cap) = line_regex.captures(line.as_str()) {
                lines.push((cap[1].chars().next().unwrap(), 
                            cap[2].parse::<u32>().unwrap()));
            }
        });

    return (dots, lines)
}

fn do_fold(line : &Line, dots : &HashSet<Dot>) -> HashSet<Dot> {
    let fold_x = |dot : &Dot| {
            if dot.0 > line.1 {
                let new_x = line.1 - (dot.0 - line.1);
                return (new_x, dot.1);
            }
            return (dot.0, dot.1);
        };

    let fold_y = |dot : &Dot| {
            if dot.1 > line.1 {
                let new_y = line.1 - (dot.1 - line.1);
                return (dot.0, new_y);
            }
            return (dot.0, dot.1);
        };

    let fold : Box<dyn Fn(&Dot) -> Dot> = if line.0 == 'x' { 
        Box::new(fold_x) 
    } 
    else { 
        Box::new(fold_y) 
    };

    dots.iter().map(|dot| fold(dot)).collect()
}

fn render(dots : &HashSet<Dot>) {
    let max_x = dots.iter().max_by_key(|dot| dot.0).unwrap().0;
    let max_y = dots.iter().max_by_key(|dot| dot.1).unwrap().1;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if dots.contains(&(x,y)) {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() {
    let args : Vec<_> = env::args().collect();
    assert!(args.len() == 2, "Specify the input!!");
    
    let (dots, lines) = get_file_contents(&args[1]);

    println!("dots visible after first fold: {:?}", do_fold(&lines[0], &dots).len());
    let folded = lines.iter().fold(dots, |dots, line| do_fold(line, &dots));
    render(&folded);
}
