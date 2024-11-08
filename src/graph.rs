use crate::Node;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct NodeAction {
    node: u32,
    is_on: bool,
}

#[derive(Debug)]
pub struct Graph {
    nodes: HashMap<u32, Node>,
    actions_queue: VecDeque<NodeAction>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            actions_queue: VecDeque::new(),
        }
    }
    pub fn insert_nodes(&mut self, nodes: Vec<(Node, u32)>) {
        for (node, id) in nodes {
            assert!(!self.nodes.contains_key(&id));
            self.nodes.insert(id, node);
        }
    }
    pub fn insert_links(&mut self, links: Vec<(u32, u32)>) {
        for link in links {
            assert!(self.nodes.contains_key(&link.0));
            assert!(self.nodes.contains_key(&link.1));
            self.nodes.get_mut(&link.0).unwrap().children.push(link.1);
        }
    }
}
