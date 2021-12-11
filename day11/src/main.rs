use std::fmt;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

type Point = (usize, usize);
type GridData = Vec<Vec<u32>>;
#[derive(Clone)]
struct Grid {
    data: GridData,
    rows: usize,
    cols: usize
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = self.data
                            .iter()
                            .map(|row| row
                                         .iter()
                                         .map(|i| i.to_string())
                                         .collect::<String>())
                            .collect::<Vec<String>>()
                            .join("\n");
        write!(f, "{}", formatted)
    }
}

fn get_file_contents(filename : &String) -> Grid {
    let file = fs::File::open(filename).expect("cannot open file");
    let data : GridData = io::BufReader::new(file).lines()
                            .filter_map(|line| line.ok()?.parse::<String>().ok())
                            .map(|line| line
                                    .chars()
                                    .filter_map(|c| c.to_digit(10)).collect())
                            .collect();

    Grid { rows: data.len() as usize, 
           cols: data[0].len(),
           data: data
    }
}

fn get_adjacents(x : i32, y : i32, rows : usize, cols : usize) -> Vec<Point> {
    [(x, y - 1),
     (x, y + 1),
     (x - 1, y),
     (x - 1, y - 1),
     (x - 1, y + 1),
     (x + 1, y),
     (x + 1, y - 1),
     (x + 1, y + 1)].iter()
                    .filter_map(|point| 
                                if (0..rows as i32).contains(&point.0) && 
                                   (0..cols as i32).contains(&point.1) {
                                       Some((point.0 as usize, point.1 as usize))
                                   }
                                   else {
                                       None
                                   })
                    .collect()

}

fn run_step(grid : &Grid) -> (Grid, u32) {
    let mut has_flashed : Vec<Point> = Vec::new();
    let mut update : GridData = grid.data
                                    .iter()
                                    .map(|row| row.iter().map(|x| x + 1).collect())
                                    .collect();

    loop {
        let mut at_peak_energy : Vec<Point> = 
            update
                .concat()
                .iter()
                .enumerate()
                .filter_map(|(i, energy)| { 
                            if energy > &9 { 
                                let coord = (i % grid.cols, i / grid.cols);
                                if !has_flashed.contains(&coord) {
                                    return Some(coord);
                                } 
                            }
                            return None
                            })
                .collect();

        if at_peak_energy.is_empty() {
            break;
        }

        at_peak_energy
            .iter()
            .for_each(|coord| 
                      get_adjacents(
                        coord.0 as i32, 
                        coord.1 as i32, 
                        grid.rows, 
                        grid.cols).iter()
                                  .for_each(|adj_coord| update[adj_coord.1][adj_coord.0] += 1));
        has_flashed.append(&mut at_peak_energy);
    } 

    has_flashed.iter().for_each(|point| update[point.1][point.0] = 0);

    (Grid{data: update, rows:grid.rows, cols:grid.cols},
     has_flashed.len() as u32)
}

fn main() {
    let args : Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");
    
    let grid = get_file_contents(&args[1]);

    println!("start:\n{:?}", grid);
    
    let mut update = grid.clone();
    let mut total_flashes = 0;
    for _ in 0..100 {
        let result = run_step(&update);
        update = result.0;
        let has_flashed = result.1;
        //println!("flashed: {}\nupdate:\n{:?}", has_flashed, update);
        total_flashes += has_flashed
    }

    println!("Total flashes: {}", total_flashes);

    let mut update = grid.clone();
    let mut step = 0;
    loop { 
        step += 1;
        let result = run_step(&update);
        update = result.0;
        let has_flashed = result.1;
        if has_flashed == (grid.rows * grid.cols) as u32 {
            break;
        }
    }

    println!("All flashed at step: {}", step);
}
