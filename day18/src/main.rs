use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::fmt;
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Type {
    Raw(u32),
    Nested(Box<SFNum>),
}

impl Type {
    fn magnitude(&self) -> u64 {
        match self {
            Type::Raw(value) => *value as u64,
            Type::Nested(ref nested) => nested.magnitude()
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) ->  fmt::Result {
        match self {
            Type::Raw(value) => write!(f, "{}", value),
            Type::Nested(nested) => write!(f, "{}", nested)
        }
    }
}

#[derive(Debug, Clone)]
struct SFNum {
    left: Type,
    right: Type,
}

type Explosion = (Option<u32>, Option<u32>);

impl SFNum {
    fn reduce(&mut self) {
        loop {
            if self.try_explode(0).is_some() {
                continue;
            }
            if self.try_split() {
                continue;
            }
            break;
        }
    }

    fn try_explode(&mut self, level: u32) -> Option<Explosion> {
        // explode first
        if level >= 4 {
            //println!("{} explodes", self);
            if let Type::Raw(left) = self.left {
                if let Type::Raw(right) = self.right {
                    return Some((Some(left), Some(right)));
                }
            }
        }

        // explode left and propagate explosion
        if let Type::Nested(ref mut left) = self.left {
            if let Some((left_exp, right_exp)) = left.try_explode(level + 1) {
                //println!("propagate explosion {:?} left {}", (left_exp, right_exp), self);
                if let Some(right_exp_val) = right_exp { 
                    if left_exp.is_some() {
                        //println!("set {} to 0", self.left);
                        self.left = Type::Raw(0);
                    }
                    match self.right {
                        Type::Raw(ref right_val) => {
                            //println!("set {} to {}", self.right, right_val + right_exp_val);
                            self.right = Type::Raw(right_val + right_exp_val);
                        },
                        Type::Nested(ref mut right_nested) => right_nested.add_left(right_exp_val),
                    }
                    //println!("value now: {}", self);
                }
                return Some((left_exp, None)); 
            }
        }

        // explode right and propagate explosion
        if let Type::Nested(ref mut right) = self.right {
            if let Some((left_exp, right_exp)) = right.try_explode(level + 1) {
                //println!("propagate explosion {:?} right {}", (left_exp, right_exp), self);
                if let Some(left_exp_val) = left_exp { 
                    if right_exp.is_some() {
                        //println!("set {} to 0", self.right);
                        self.right = Type::Raw(0);
                    }
                    match self.left {
                        Type::Raw(ref left_val) => {
                            //println!("set {} to {}", self.left, left_val + left_exp_val);
                            self.left = Type::Raw(left_val + left_exp_val)
                        },
                        Type::Nested(ref mut left_nested) => left_nested.add_right(left_exp_val),
                    }
                    //println!("value now: {}", self);
                }
                return Some((None, right_exp)); 
            }
        }
        None
    }

    fn try_split(&mut self) -> bool {
        for node in [&mut self.left, &mut self.right] {
            match node {
                Type::Raw(val) => {
                    if *val > 9 {
                        let split = *val as f32 / 2.0;
                        *node = Type::Nested(
                            Box::new(SFNum {
                                left: Type::Raw(split.floor() as u32),
                                right: Type::Raw(split.ceil() as u32),
                                }));
                       return true;
                    }
                },
                Type::Nested(nested) => { 
                    if nested.try_split() { 
                        return true; 
                    } 
                } 
            }
        }
        return false;
    }

    fn add_right(&mut self, value: u32) {
        //println!("propagate {} down to {}", value, self.right);
        match &mut self.right {
            Type::Raw(raw) => self.right = Type::Raw(*raw + value),
            Type::Nested(nested) => nested.add_right(value)
        }
    }

    fn add_left(&mut self, value: u32) {
        //println!("propagate {} down to {}", value, self.left);
        match &mut self.left {
            Type::Raw(raw) => self.left = Type::Raw(*raw + value),
            Type::Nested(nested) => nested.add_left(value)
        }
    }

    fn magnitude(&self) -> u64 {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }

}

impl fmt::Display for SFNum {
    fn fmt(&self, f: &mut fmt::Formatter) ->  fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl std::ops::Add<SFNum> for SFNum {
    type Output = SFNum;
    fn add(self, rhs: SFNum) -> SFNum {
        let mut result = SFNum {
            left: Type::Nested(Box::new(self)),
            right: Type::Nested(Box::new(rhs)),
        };
        result.reduce();
        return result;
    }
}

fn get_file_contents(filename: &String) -> Vec<SFNum> {
    let file = fs::File::open(filename).expect("cannot open file");
    io::BufReader::new(file).lines()
                            .filter_map(|line| line.ok()?.parse::<String>().ok())
                            .map(|line| 
                                 parse_to_sfnum(&line.chars().collect::<Vec<_>>()[..], 0).0)
                            .collect()
}

fn parse_to_sfnum(line: &[char], level: u32) -> (SFNum, usize) {
    let mut branches = Vec::new();
    let mut i = 1; // skip opening [
    while i < line.len() {
        match line[i] {
            '[' => {
                let (val, inc) = parse_to_sfnum(&line[i..], level + 1);
                branches.push(Type::Nested(Box::new(val)));
                i += inc
            },
            ',' => assert!(branches.len() == 1),
            ']' => break,
            n => branches.push(Type::Raw(n.to_digit(10).unwrap()))
        }
        i += 1;
    }

    let ret = SFNum {
        left: branches.remove(0),
        right: branches.remove(0),
    };
    return (ret, i);
}


fn main() {
    let args : Vec<_> = env::args().collect();
    let input = get_file_contents(&args[1]);

    let start = input[0].clone();
    let sum = input.iter().skip(1).fold(start, |acc, x| acc + x.clone());
    println!("sum: {}", sum);
    println!("magnitude: {}", sum.magnitude());

    let max_magnitude = input.iter()
                            .tuple_combinations()
                            .map(|(a,b)| std::cmp::max((a.clone()+b.clone()).magnitude(), 
                                                       (b.clone()+a.clone()).magnitude()))
                            .max()
                            .unwrap();
    println!("Max magnitude: {}", max_magnitude); 
}
