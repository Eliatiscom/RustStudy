use serde::{Deserialize, Serialize};
use std::cmp::{Ordering};

#[derive(Serialize, Deserialize)]
pub struct Graph {
    nodes: Vec<GraphNode>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphNode {
    id: u32,
    coords: Point,
    children: Vec<u32>
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Point {
    x: i32,
    y: i32
}

pub struct Path {
    nodes: Vec<u32>,
    length: f64
}

impl Graph {
    pub fn new() -> Graph {
        Graph { nodes: Vec::new() }
    }

    pub fn solve_all(&self) -> Result<Path, String> {
        match self.nodes.len() {
            0 => Err(String::from("Graph has no nodes")),
            1 => Err(String::from("Graph only has one node")),
            _ => self.bare_solve(self.nodes.iter().min().unwrap().id, self.nodes.iter().max().unwrap().id)
        }
    }

    pub fn solve_given(&self, start: u32, end: u32) -> Result<Path, String> {
        match self.nodes.len() {
            0 => Err(String::from("Graph has no nodes")),
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

impl GraphNode {
    pub fn new(id: u32, coords: &Point) -> GraphNode {
        GraphNode { id: id, coords: *coords, children: Vec::new() }
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
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

    pub fn add_node(&mut self, node: &GraphNode) {
        if self.nodes.len() > 0 {
            assert!(self.nodes.last().unwrap().children.iter().any(|child_id| *child_id == node.id));
            assert!(node.children.iter().any(|child_id| *child_id == node.id));
        }

        self.nodes.push(*node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_nodes() -> Vec<GraphNode> {
        let mut node1 = GraphNode::new(1, &Point::new(0, 1));
        node1.children.push(2);
        node1.children.push(3);
        let mut node2 = GraphNode::new(2, &Point::new(0, 1));
        node2.children.push(1);
        node2.children.push(4);
        let mut node3 = GraphNode::new(3, &Point::new(0, 1));
        node3.children.push(1);
        node3.children.push(4);
        let mut node4 = GraphNode::new(4, &Point::new(0, 1));
        node4.children.push(2);
        node4.children.push(3);

        //
        //    Connected nodes:
        //
        //     (1)---------(3)
        //      |           |
        //      |           |
        //      |           |
        //     (2)---------(4)
        //

        vec![node1, node2, node3, node4]
    }

    #[test]
    fn path_add_connected_nodes() {
        let nodes = create_nodes();
        let mut path = Path::new();

        path.add_node(&nodes[0]);
        path.add_node(&nodes[1]);
        path.add_node(&nodes[3]);
        path.add_node(&nodes[2]);

        assert_eq!(path.nodes[0], nodes[0]);
        assert_eq!(path.nodes[1], nodes[1]);
        assert_eq!(path.nodes[2], nodes[3]);
        assert_eq!(path.nodes[3], nodes[2]);
    }
}