use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

type Mask = Vec<Vec<bool>>;
type Coord = (usize, usize);
type ICoord = (i32, i32);

fn get_file_contents(filename : &String) -> Vec<Vec<u32>> {
    let file = fs::File::open(filename).expect("cannot open file");
    io::BufReader::new(file).lines()
                            .filter_map(|line| line.ok()?.parse::<String>().ok())
                            .map(|line| line.chars().filter_map(|x| x.to_digit(10)).collect())
                            .collect()
}

fn get_unmarked_adjacents(coord : &ICoord, mask : &Mask) -> Vec<Coord> {
    let rows = mask.len() as i32;
    let cols = mask[0].len() as i32;
    vec![(coord.0 - 1, coord.1), 
         (coord.0 + 1, coord.1),
         (coord.0, coord.1 + 1),
         (coord.0, coord.1 -1)].iter()
                               .filter_map(|&(x,y)| 
                                           if (0..rows).contains(&y) && (0..cols).contains(&x) && !mask[y as usize][x as usize] { 
                                               Some((x as usize, y as usize)) 
                                           } 
                                           else { 
                                               None 
                                           })
                                .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");
    
    let input = get_file_contents(&args[1]);

    let cols = input[0].len();
    let rows = input.len();

    let mut low_points = vec![vec![false; cols]; rows];

    for j in 0..rows {
        for i in 0..cols {
            let up = if j == 0 { 9 } else { input[j-1][i] };
            let down = if j == rows - 1 { 9 } else { input[j+1][i] };
            let left = if i == 0 { 9 } else { input[j][i-1] };
            let right = if i == cols - 1 { 9 } else { input[j][i+1] };

            low_points[j][i] = [up, down, left, right].iter().all(|&adj| input[j][i] < adj);
        }
    }
    
    let risk_level_sum : u32  = input.concat()
                                      .iter()
                                      .zip(low_points.concat())
                                      .filter_map(|(value, is_low)| if is_low { Some(1 + *value) } else { None })
                                      .sum();

    println!("part1: {:?}", risk_level_sum);

    let mut masks : Vec<Mask> = low_points.concat()
                                          .iter()
                                          .enumerate()
                                          .filter_map(|(idx, value)| 
                                                      if *value { 
                                                          let mut mask = vec![vec![false; cols]; rows];
                                                          mask[idx / cols][idx % cols] = true;
                                                          Some(mask)
                                                      }
                                                      else {
                                                          None
                                                      })
                                            .collect();

    for mask in &mut masks {
        let minima = mask.concat().iter().position(|&x| x).unwrap();
        let mut cur_x = minima % cols;
        let mut cur_y = minima / cols;
        let mut coords_to_try = get_unmarked_adjacents(&(cur_x as i32, cur_y as i32), &mask);
        coords_to_try.retain(|&(x, y)| input[y][x] != 9 && input[y][x] > input[cur_y][cur_x]);

        while !coords_to_try.is_empty() {
            coords_to_try.iter().for_each(|&(x, y)|  mask[y][x] = true );
            if let Some(new_coord) = coords_to_try.pop() {
                cur_x = new_coord.0;
                cur_y = new_coord.1;
                let mut adjacents = get_unmarked_adjacents(&(cur_x as i32, cur_y as i32), &mask);
                adjacents.retain(|&(x, y)| input[y][x] != 9 && input[y][x] > input[cur_y][cur_x]);
                coords_to_try.append(&mut adjacents);
                coords_to_try.sort();
                coords_to_try.dedup();
            }
        }
    }

    let mut basin_sizes : Vec<usize> = masks.iter().map(|mask| mask.concat().iter().filter(|&x| *x).count()).collect();
    basin_sizes.sort();
    basin_sizes.reverse();
    println!("basin sizes: {:?}", basin_sizes);
    println!("part2: {}", basin_sizes.iter().take(3).product::<usize>());

}
