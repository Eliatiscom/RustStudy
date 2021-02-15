use serde::{Deserialize, Serialize};
use std::cmp::{Ordering};

#[derive(Serialize, Deserialize)]
pub struct Graph {
    nodes: Vec<GraphNode>
}

#[derive(Serialize, Deserialize)]
pub struct GraphNode {
    id: u32,
    coords: Point,
    children: Vec<u32>
}

#[derive(Serialize, Deserialize)]
pub struct Point {
    x: i32,
    y: i32
}

pub struct Path {
    nodes: Vec<GraphNode>,
    length: f64
}

impl Graph {
    pub fn new() -> Graph {
        Graph { nodes: Vec::new() }
    }

    pub fn solve_all(&self) -> Result<Path, String> {
        match self.nodes.len() {
            0 => Err(String::from("Graph had no nodes")),
            1 => Err(String::from("Graph only has one node")),
            _ => self.bare_solve(self.nodes.iter().min().unwrap().id, self.nodes.iter().max().unwrap().id)
        }
    }

    pub fn solve_given(&self, start: u32, end: u32) -> Result<Path, String> {
        match self.nodes.len() {
            0 => Err(String::from("Graph had no nodes")),
            1 => Err(String::from("Graph only has one node")),
            _ => {
                if self.nodes.iter().any(|el| el.id == start) && self.nodes.iter().any(|el| el.id == end) {
                    self.bare_solve(start, end)
                } else {
                    Err(String::from("Could not find start/end node"))
                }
            }
        }
    }

    fn bare_solve(&self, start: u32, end: u32) -> Result<Path, String> {
        Ok(Path::new())
    }
}

impl Eq for Point {}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.cmp(&other.y),
            neq => neq
        }
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for GraphNode {}
impl PartialEq for GraphNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for GraphNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialOrd for GraphNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Path {
    pub fn new() -> Path {
        Path { 
            nodes: Vec::new(), 
            length: 0.0
        }
    }
}