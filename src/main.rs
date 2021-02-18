mod graph;

use std::env;
use std::fs;
use std::io;
use graph::{Graph};

fn get_graph(filename: &String) -> Option<Graph> {
    if let Ok(file) = fs::File::open(filename) {
        let reader = io::BufReader::new(file);
        if let Ok(graph) = serde_json::from_reader(reader) as serde_json::Result<Graph> {
            return Some(graph);
        }
    }
    None
}

fn main() -> Result<(), i32> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing filename");
        return Err(-1);
    }

    let filename = &args[1];
    println!("Filename: {}", filename);

    if let Some(graph) = get_graph(filename) {
        println!("Parsed graph: {:#?}", graph);
        // if let Ok(path) = graph.solve_all() {
        //     println!("Path found");
        // } else {
        //     println!("Path not found");
        // }
    }

    Ok(())
}
