use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::fmt;


type Coord = (usize, usize);

#[derive(Clone)]
struct Node {
    fscore: u32,
    gscore: u32,
    prev: Option<Coord>,
    visited: bool
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) ->  fmt::Result {
        write!(f, "{}", if self.visited {'v'} else {'-'})
    }
}

struct Grid<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>
}

impl<T> Grid<T> {
    fn at(&mut self, (x, y): &Coord) -> &mut T {
        &mut self.data[y * self.cols + x]
    }

    fn at_(&self, (x, y): &Coord) -> &T {
        &self.data[y * self.cols + x]
    }
     
    fn coord_of(&self, idx: usize) -> Coord {
        (idx % self.cols, idx / self.cols)
    }

    fn neighbours_of(&self, (x_, y_): &Coord) -> Vec<Coord> {
        let x = *x_ as i32;
        let y = *y_ as i32;
        [(x - 1, y),
         (x + 1, y),
         (x, y - 1),
         (x, y + 1)].iter()
                    .filter(|&(i, j)| (0..self.cols as i32).contains(i) && 
                                      (0..self.rows as i32).contains(j))
                    .map(|(i, j)| (*i as usize, *j as usize))
                    .collect()
    }
}

impl<T> fmt::Display for Grid<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        for y in 0..self.rows {
            for x in 0..self.cols {
                let _ = write!(f, "{}", self.at_(&(x,y))); 
            }
            let _ = writeln!(f, "");
        }
        write!(f,"")
    }
}

fn get_file_contents(filename : &String) -> Grid<u32> {
    let file = fs::File::open(filename).expect("cannot open file");
    let contents: Vec<Vec<_>> = io::BufReader::new(file).lines()
                                .filter_map(|line| line.ok()?.parse::<String>().ok())
                                .map(|line| line.chars()
                                                .filter_map(|i| i.to_digit(10)).collect())
                                .collect();
    Grid {
        rows: contents.len(),
        cols: contents[0].len(),
        data: contents.concat()
    }
}

fn distance(from: &Coord, to: &Coord) -> u32 {
    (to.0 - from.0) as u32 + (to.1 - from.1) as u32
}

fn dijkstra(risk_map: &Grid<u32>, source: &Coord, target: &Coord) -> u32 {  
    let mut score_map = Grid {
        rows: risk_map.rows,
        cols: risk_map.cols,
        data: vec![Node{gscore: u32::MAX, fscore: u32::MAX, prev: None, visited: false}; risk_map.data.len()]
    };

    score_map.at(source).fscore = 0;
    score_map.at(source).gscore = 0;

    while !score_map.at(target).visited {
        //println!("{}", score_map);
        let lowest_score_idx = score_map.data.iter()
                                             .enumerate()
                                             .filter(|(_, node)| !node.visited)
                                             .min_by_key(|(_, node)| node.fscore)
                                             .map(|(i,_)| i)
                                             .unwrap();

        let mut node = &mut score_map.data[lowest_score_idx];
        node.visited = true;
        let score = node.gscore;
        let coord = score_map.coord_of(lowest_score_idx);

        score_map.neighbours_of(&coord).iter().for_each(|neighbour| {
            let score_to_neighbour = score + risk_map.at_(neighbour);
            //println!("Score from {:?} -> {:?}: {}", node_coord, neighbour, score_to_neighbour);
            let mut neighbour_node = score_map.at(neighbour);
            if score_to_neighbour < neighbour_node.gscore {
                neighbour_node.gscore = score_to_neighbour;
                neighbour_node.fscore = score_to_neighbour + distance(&neighbour, &target);
                neighbour_node.prev = Some(coord);
            }
        });
    }

    //let mut node = target.clone();
    //let mut score = 0;
    //while &node != source {
    //    score += risk_map.at_(&node);
    //    node = score_map.at_(&node).prev.unwrap();
    //}
    //return score;
    return score_map.at_(target).gscore;
}

fn expand_map(risk_map: &Grid<u32>) -> Grid<u32> {
    let expanded_cols : Vec<u32> = risk_map.data
                            .chunks(risk_map.cols)
                            .map(|row| 
                                 (0..5).map(|inc| 
                                            row.iter()
                                                .map(|i| i + inc ).collect())
                                       .collect::<Vec<Vec<u32>>>().concat())
                            .collect::<Vec<Vec<u32>>>()
                            .concat();

    let expanded : Vec<u32> = (0..5).map(|inc| 
                                         expanded_cols.iter()
                                                      .map(|i| (i + inc - 1) % 9 + 1).collect())
                                    .collect::<Vec<Vec<u32>>>()
                                    .concat();

    Grid {
        rows: risk_map.rows * 5,
        cols: risk_map.cols * 5,
        data: expanded,
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    assert!(args.len() == 2, "Supply a file!");
    let risk_map = get_file_contents(&args[1]);

    println!("{}", risk_map);

    let distance_to_target = dijkstra(&risk_map, &(0,0),  &(risk_map.cols - 1, risk_map.rows -1));
    println!("distance_to_target: {}", distance_to_target);
    
    let expanded_map = expand_map(&risk_map);
    let distance_to_target = dijkstra(&expanded_map, &(0,0), &(expanded_map.cols - 1, expanded_map.rows - 1));
    println!("part2 distance_to_target: {}", distance_to_target);
}
