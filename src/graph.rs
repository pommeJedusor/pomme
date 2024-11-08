use crate::Node;
use crate::Node::LogicBlock;
use crate::Node::StoringBlock;
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

    /// do not work with button links for storing block
    /// if the second one is a storing block takes the first one as the source
    pub fn insert_links(&mut self, links: Vec<(u32, u32)>) {
        for link in links {
            assert!(self.nodes.contains_key(&link.0));
            assert!(self.nodes.contains_key(&link.1));
            match self.nodes.get_mut(&link.1).unwrap() {
                StoringBlock(node) => node.source = link.1,
                LogicBlock(_) => match self.nodes.get_mut(&link.1).unwrap() {
                    StoringBlock(node) => node.children.push(link.1),
                    LogicBlock(node) => node.children.push(link.1),
                },
            }
        }
    }
    /// links must be compose of one logical block and one storing block
    pub fn insert_button_links(&mut self, links: Vec<(u32, u32)>) {
        for link in links {
            assert!(self.nodes.contains_key(&link.0));
            assert!(self.nodes.contains_key(&link.1));
            let is_first_storing_block =
                matches!(self.nodes.get(&link.0).unwrap(), StoringBlock(_));
            let is_second_storing_block =
                matches!(self.nodes.get(&link.1).unwrap(), StoringBlock(_));
            assert_ne!(is_first_storing_block, is_second_storing_block);

            let storing_block_link = if is_first_storing_block {
                link.0
            } else {
                link.1
            };
            let logical_block_link = if is_first_storing_block {
                link.1
            } else {
                link.0
            };
            match self.nodes.get_mut(&storing_block_link).unwrap() {
                StoringBlock(node) => node.button_node = logical_block_link,
                _ => unreachable!(),
            };
        }
    }

    pub fn get_node(&self, key: u32) -> Option<&Node> {
        self.nodes.get(&key)
    }
    pub fn get_mut_node(&mut self, key: u32) -> Option<&mut Node> {
        self.nodes.get_mut(&key)
    }
}
