use std::io;
use std::env;
use std::fs;
use std::fmt;
use std::io::prelude::*;

#[derive(Clone)]
struct Image {
    rows: usize,
    cols: usize,
    data: Vec<u8>
}

impl Image {
    fn set(&mut self, x: usize, y: usize, value: u8) {
        self.data[y * self.cols + x] = value;
    }

    fn pad(&self, value: u8, pad: usize) -> Image {
        let mut padded = vec![value; (self.cols + pad * 2) * pad];
        self.data.chunks(self.cols)
                 .for_each(|row| {
                     padded.extend_from_slice(&vec![value; pad]);
                     padded.extend_from_slice(row);
                     padded.extend_from_slice(&vec![value; pad]);
                 });
        padded.extend_from_slice(&vec![value; (self.cols + pad * 2) * pad]);

        Image {
            rows: self.rows + pad * 2,
            cols: self.cols + pad * 2,
            data: padded
        }
    }

    fn crop(&mut self) {
        while self.data[..self.cols].iter().all(|&x| x == 0) {
            self.data.drain(..self.cols);
            self.rows -= 1
        }
        while self.data[(self.rows - 1)*self.cols..].iter().all(|&x| x == 0) {
            self.data.drain((self.rows - 1)*self.cols..);
            self.rows -= 1
        }
        loop {
            let left_col : Vec<usize> = 
                (0..self.rows).map(|r| r * self.cols).collect();
            if left_col.iter().all(|&i| self.data[i] == 0) {
                self.remove_non_consecutive(left_col);
                self.cols -= 1;
            }
            else {
                break;
            }
        }
        loop {
            let right_col : Vec<usize> = 
                (1..=self.rows).map(|r| r * self.cols - 1).collect();
            if right_col.iter().all(|&i| self.data[i] == 0) {
                self.remove_non_consecutive(right_col);
                self.cols -= 1;
            }
            else {
                break;
            }
        }
    }

    fn remove_non_consecutive(&mut self, mut to_remove: Vec<usize>) {
        to_remove.sort();
        to_remove.reverse();
        for r in to_remove {
            self.data.remove(r);
        }
    }

    fn kernel(&self, x: usize, y: usize, size: usize) -> Vec<u8> {
        assert!(size % 2 != 0);
        let start_y = y - size/2;
        let end_y = y + size/2;
        let start_x = x - size/2;

        (start_y..=end_y)
            .fold(Vec::new(), |mut kernel, cur_y| {
                let start = cur_y * self.cols + start_x;
                kernel.extend_from_slice(&self.data[start..start+size]);
                kernel
            })
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) ->  fmt::Result {
        let out :String = self.data
            .iter()
            .enumerate()
            .map(|(i, pixel)| {
                  let c = if *pixel == 0 { "." } else { "#" }.to_string();
                  c + if (i + 1) % self.cols == 0 { "\n" } else { "" }
            })
            .collect();
        write!(f, "{}", out)
    }
}

fn blank_image(rows: usize, cols: usize) -> Image {
    Image { rows: rows, cols: cols, data: vec![0; rows*cols] }
}

fn binary_to_dec(binary: &Vec<u8>) -> usize {
    binary.iter().fold(0, |dec, bit| dec << 1 ^ *bit as usize)
}

fn hashdot_to_int(c: char) -> Option<u8> {
    match c {
        '#' => Some(1),
        '.' => Some(0),
        _ => None
    }
}

fn get_file_contents(filename: &String) -> (Vec<u8>, Image) {
    let file = fs::File::open(filename).expect("cannot open file");
    let mut lines = io::BufReader::new(file).lines()
                    .filter_map(|line| line.ok()?.parse::<String>().ok());
    
    let enhancement_line = lines.next()
                                .unwrap()
                                .chars()
                                .filter_map(hashdot_to_int)
                                .collect();

    let data = lines.skip(1)
        .map(|line| line.chars().filter_map(hashdot_to_int).collect())
        .collect::<Vec<Vec<u8>>>();
    
    (enhancement_line, 
     Image{ 
         rows: data.len(),
         cols: data[0].len(),
         data: data.concat() 
     })
}

fn run(iterations: u32,
       mut image: Image,
       enhancement_line: &Vec<u8>) -> u64 {
    
    let mut pad_val = 0;
    for _ in 0..iterations {
        let padded = image.pad(pad_val, 3);
        image = blank_image(padded.rows, padded.cols);
        for x in 1..image.cols-1 {
            for y in 1..image.rows-1 {
                let idx = binary_to_dec(&padded.kernel(x, y, 3));
                let new_val = enhancement_line[idx];
                image.set(x,y,new_val)
            }
        }
        pad_val = enhancement_line[binary_to_dec(&vec![pad_val; 9])];
        image.crop();
    }

    image.data.iter().fold(0,|acc, &x| acc + x as u64)
}

fn main() {
    let args: Vec<_> = env::args().collect();
    assert!(args.len() == 2, "Supply the file!");
    let (enhancement_line, input_image) = get_file_contents(&args[1]);
    
    let part1 = run(2, input_image.clone(), &enhancement_line);
    println!("Sum of active pixels: {}", part1);
    let part2 = run(50, input_image.clone(), &enhancement_line);
    println!("Sum of active pixels: {}", part2);
}
