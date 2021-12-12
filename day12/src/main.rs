use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

type Graph = HashMap<String, Vec<String>>;

fn build_graph_from_file(filename : &String) -> Graph {
    let file = fs::File::open(filename).expect("cannot open file");
    io::BufReader::new(file).lines()
                            .filter_map(|line| line.ok()?.parse::<String>().ok())
                            .fold(HashMap::new(), |mut graph, line| {
                                let mut parts = line.split('-');
                                let cave = parts.next().unwrap().to_string();
                                let conn = parts.next().unwrap().to_string();
                                graph.entry(cave.clone()).or_insert(Vec::new()).push(conn.clone());
                                graph.entry(conn).or_insert(Vec::new()).push(cave);
                                graph
                            })
}

fn is_small_cave(cave : &String) -> bool {
    cave.chars().any(char::is_lowercase)
}

fn traverse(
    graph : &Graph, 
    cave : &String, 
    mut path : Vec<String>,
    predicate : fn(&String, &Vec<String>) -> bool) -> Vec<Vec<String>> { 

    path.push(cave.clone());
    if cave == "end" {
        return vec![path];
    }

    graph[cave]
        .iter()
        .fold(Vec::new(), |mut paths, conn| { 
            if predicate(&conn, &path) {
                paths.append(&mut traverse(graph, conn, path.clone(), predicate))
            }
            paths
        })
}

fn find_all_paths(graph : &Graph, predicate : fn(&String, &Vec<String>) -> bool) -> Vec<Vec<String>> {
    traverse(graph, &"start".to_string(), Vec::new(), predicate)
}

fn part1_predicate(conn : &String, path : &Vec<String>) -> bool {
    !(is_small_cave(conn) && path.contains(conn))
}

fn part2_predicate(conn : &String, path : &Vec<String>) -> bool {
    if conn == "start" {
        return false;
    }
    else if conn == "end" {
        return true;
    }
    if is_small_cave(conn) {
        let mut hist : HashMap<&String, u32>  = HashMap::new();
        hist.insert(conn, 1);
        for cave in path.iter().filter(|&cave| is_small_cave(cave)) {
            *hist.entry(cave).or_insert(0) += 1;
        }
        return hist.into_values().filter(|&c| c > 1).sum::<u32>() <= 2;
    }
    return true;
}

fn main() {
    let args : Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");

    let graph = build_graph_from_file(&args[1]);

    println!("graph:\n {:?}", graph);

    let all_paths = find_all_paths(&graph, part1_predicate);
    println!("part1 {} paths", all_paths.len());

    let all_paths = find_all_paths(&graph, part2_predicate);
    println!("part2 {} paths", all_paths.len());
}
