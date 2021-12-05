use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Hash, Debug)]
struct Point {
    x : u32,
    y : u32
}
impl PartialEq for Point {
    fn eq(&self, other : &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}


type Line = (Point, Point);

fn parse_point(point_string : &str) -> Point {
    let mut split = point_string.split(',');
    Point {
        x: split.next().unwrap().parse::<u32>().unwrap(),
        y: split.next().unwrap().parse::<u32>().unwrap()
    }
}

fn get_file_contents(filename : &String) -> Vec<Line> {
    let file = fs::File::open(filename).expect("cannot open file");
    io::BufReader::new(file).lines()
                            .filter_map(|line| line.ok()?.parse::<String>().ok())
                            .map(|line| {
                                let mut split = line.split(" -> ");
                                (parse_point(split.next().unwrap()), 
                                 parse_point(split.next().unwrap()))
                            })
                            .collect()
}

fn is_on_axis(line : &Line) -> bool {
    line.0.x == line.1.x || line.0.y == line.1.y
}

fn linear_range(start : u32, finish : u32) -> Vec<u32> {
    if finish >= start {
        (start..=finish).collect()
    }
    else {
        (finish..=start).rev().collect()
    }
}

fn interpolate(line: &Line) -> Vec<Point> {
    let xs = linear_range(line.0.x, line.1.x);
    let ys = linear_range(line.0.y, line.1.y);
    if xs.len() == 1 {
        return ys.iter().map(|y| Point { x: line.0.x, y: *y}).collect();
    }
    else if ys.len() == 1 {
        return xs.iter().map(|x| Point { x: *x, y: line.0.y}).collect();
    }
    assert!(xs.len() == ys.len());
    return xs.iter().zip(ys).map(|(x, y)| Point { x: *x, y: y }).collect();
}

fn get_intersections(line_occupancies : impl Iterator<Item=Point>) -> usize {
    line_occupancies.fold(HashMap::new(), |mut map, point| {
                                    let point_count = map.entry(point).or_insert(0);
                                    *point_count += 1;
                                    map
                                })
                    .iter()
                    .fold(0, |acc, (_, count)| acc + if count > &1 { 1 } else { 0 })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");

    let input_lines = get_file_contents(&args[1]);

    let on_axis_lines = input_lines.iter()
                                   .filter(|line| is_on_axis(line));
                                   
    
    let on_axis_occupancies = on_axis_lines.map(|line| interpolate(line))
                                                     .flatten();
    println!("on-axis intersections: {:?}", get_intersections(on_axis_occupancies));

    let occupancies = input_lines.iter().map(|line| interpolate(line)).flatten();
    println!("total intersections: {:?}", get_intersections(occupancies));
}

