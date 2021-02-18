use serde::{Deserialize, Serialize};
use std::cmp::{Ordering};

#[derive(Serialize, Deserialize, Debug)]
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

pub struct Path<'a> {
    graph: Graph,
    nodes: Vec<&'a GraphNode>
}

pub enum SolveStatus<'a> {
    Solved(Path<'a>),
    Unsolved(Graph, &'static str)
}

impl Graph {
    pub fn with_nodes(nodes: Vec<GraphNode>) -> Graph {
        Graph { nodes }
    }

    pub fn get_node(&self, id: u32) -> Option<&GraphNode> {
        assert_ne!(id, 0);
        self.nodes.iter().find(|node| node.id == id)
    }

    pub fn solve_all<'a>(self) -> SolveStatus<'a> {
        match self.nodes.len() {
            0 => SolveStatus::Unsolved(self, "Graph has no nodes"),
            1 => SolveStatus::Unsolved(self, "Graph only has one node"),
            _ => {
                let first_node = self.nodes.iter().min().unwrap().id;
                let last_node = self.nodes.iter().max().unwrap().id;
                self.bare_solve(first_node, last_node)
            }
        }
    }

    pub fn solve_given<'a>(self, start: u32, end: u32) -> SolveStatus<'a> {
        if start == end {
            return SolveStatus::Unsolved(self, "Start node cannot be the same as end node");
        }

        match self.nodes.len() {
            0 => SolveStatus::Unsolved(self, "Graph has no nodes"),
            1 => SolveStatus::Unsolved(self, "Graph only has one node"),
            _ => {
                if self.nodes.iter().any(|el| el.id == start) && self.nodes.iter().any(|el| el.id == end) {
                    self.bare_solve(start, end)
                } else {
                    SolveStatus::Unsolved(self, "Could not find start/end node")
                }
            }
        }
    }

    fn bare_solve<'a>(self, _start: u32, _end: u32) -> SolveStatus<'a> {
        SolveStatus::Solved(Path::new(self))
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

impl<'a> Path<'a> {
    pub fn new(graph: Graph) -> Self {
        Self {
            graph,
            nodes: Vec::new()
        }
    }

    pub fn push_node(&mut self, id: u32) {
        let child_check = |&child_id| child_id == id;

        // check if the last node contains 'id' as a child
        if let Some(last) = self.nodes.last_mut() {
            assert!(last.children.iter().any(&child_check));
        }

        // get the node with given 'id' from graph
        if let Some(new_node) = self.graph.get_node(id) {
            // if found, check if node contains the last node in the path as a child
            assert!(new_node.children.iter().any(&child_check));
            self.nodes.push(new_node);
        } else {
            panic!("Given id does not correspond to a node in the contained graph");
        }
    }

    pub fn reclaim_graph(self) -> Graph {
        self.graph
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
    fn test_path_adding_nodes() {
        let graph = Graph::with_nodes(create_nodes());
        let mut path = Path::new(graph);

        path.push_node(1);
        path.push_node(2);
        path.push_node(4);
        path.push_node(3);

        let mut path_iter = path.nodes.iter();
        assert_eq!(Some(&1), path_iter.next());
        assert_eq!(Some(&2), path_iter.next());
        assert_eq!(Some(&4), path_iter.next());
        assert_eq!(Some(&3), path_iter.next());
        assert_eq!(None, path_iter.next());
    }
}