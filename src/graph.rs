use crate::LogicBlock;
use crate::Node;
use crate::StoringBlock;
use std::collections::HashMap;
use std::collections::VecDeque;

type NodeId = u32;

#[derive(Debug, Copy, Clone)]
pub enum NodeAction {
    InitNode,
    IncreaseValue,
    DecreaseValue,
}

#[derive(Debug)]
pub struct Graph {
    nodes: HashMap<u32, Node>,
    actions_queue: VecDeque<(NodeAction, NodeId)>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            actions_queue: VecDeque::new(),
        }
    }
    pub fn insert_nodes(&mut self, nodes: Vec<(Node, NodeId)>) {
        for (node, id) in nodes {
            assert!(!self.nodes.contains_key(&id));
            self.nodes.insert(id, node);
        }
    }

    /// do not work with button links for storing block
    /// if the second one is a storing block takes the first one as the source
    pub fn insert_links(&mut self, links: Vec<(NodeId, NodeId)>) {
        //TODO logical block must point toward storing block as the reversed is true
        for link in links {
            assert!(self.nodes.contains_key(&link.0));
            assert!(self.nodes.contains_key(&link.1));
            match self.nodes.get_mut(&link.1).unwrap() {
                Node::StoringBlock(node) => node.source = link.1,
                Node::LogicBlock(_) => match self.nodes.get_mut(&link.0).unwrap() {
                    Node::StoringBlock(node) => node.children.push(link.1),
                    Node::LogicBlock(node) => node.children.push(link.1),
                },
            }
        }
    }
    /// links must be compose of one logical block and one storing block
    pub fn insert_button_links(&mut self, links: Vec<(NodeId, NodeId)>) {
        for link in links {
            assert!(self.nodes.contains_key(&link.0));
            assert!(self.nodes.contains_key(&link.1));
            let is_first_storing_block =
                matches!(self.nodes.get(&link.0).unwrap(), Node::StoringBlock(_));
            let is_second_storing_block =
                matches!(self.nodes.get(&link.1).unwrap(), Node::StoringBlock(_));
            assert_ne!(is_first_storing_block, is_second_storing_block);

            let (storing_block_link, logical_block_link) = if is_first_storing_block {
                (link.0, link.1)
            } else {
                (link.1, link.0)
            };
            match self.nodes.get_mut(&storing_block_link).unwrap() {
                Node::StoringBlock(node) => node.button_node = logical_block_link,
                _ => unreachable!(),
            };
            match self.nodes.get_mut(&logical_block_link).unwrap() {
                Node::LogicBlock(node) => node.children.push(storing_block_link),
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
    pub fn get_logical_block(&self, key: u32) -> Option<&LogicBlock> {
        match self.nodes.get(&key)? {
            Node::LogicBlock(node) => Some(node),
            Node::StoringBlock(_) => {
                assert!(false);
                unreachable!();
            }
        }
    }

    pub fn get_storing_block(&self, key: u32) -> Option<&StoringBlock> {
        match self.nodes.get(&key)? {
            Node::LogicBlock(_) => {
                assert!(false);
                unreachable!();
            }
            Node::StoringBlock(node) => Some(node),
        }
    }

    /// only when initialising the graph
    /// add add increasevalue action to all children if the node is on and if it hasn't been
    /// explored during the initialisation (if the value is 0)
    fn init_node(&mut self, node: u32) {
        let node = self.get_mut_node(node).expect("node not found");
        match node {
            Node::LogicBlock(node) => {
                if !node.is_on() {
                    return;
                }
                for child in node.children.clone() {
                    self.actions_queue
                        .push_back((NodeAction::IncreaseValue, child));
                }
            }
            Node::StoringBlock(_) => todo!(),
        }
    }

    fn increase_value(&mut self, node: u32) {
        let node = self.get_mut_node(node).expect("node not found");
        match node {
            Node::LogicBlock(node) => {
                let was_on = node.is_on();
                node.set_value(node.get_value() + 1);
                let is_on = node.is_on();
                if is_on == was_on {
                    return;
                }
                let action = match is_on {
                    true => NodeAction::IncreaseValue,
                    false => NodeAction::DecreaseValue,
                };
                for child in node.children.clone() {
                    self.actions_queue.push_back((action, child));
                }
            }
            Node::StoringBlock(_) => todo!(),
        }
    }

    fn decrease_value(&mut self, node: u32) {
        let node = self.get_mut_node(node).expect("node not found");
        match node {
            Node::LogicBlock(node) => {
                let was_on = node.is_on();
                node.set_value(node.get_value() - 1);
                let is_on = node.is_on();
                if is_on == was_on {
                    return;
                }
                let action = match is_on {
                    true => NodeAction::IncreaseValue,
                    false => NodeAction::DecreaseValue,
                };
                for child in node.children.clone() {
                    self.actions_queue.push_back((action, child));
                }
            }
            Node::StoringBlock(_) => todo!(),
        }
    }

    pub fn do_action(&mut self) {
        let action = self.actions_queue.pop_front();
        if action.is_none() {
            return;
        }
        let action = action.unwrap();
        match action.0 {
            NodeAction::InitNode => self.init_node(action.1),
            NodeAction::IncreaseValue => self.increase_value(action.1),
            NodeAction::DecreaseValue => self.decrease_value(action.1),
        }
    }

    pub fn do_actions(&mut self) {
        while !self.actions_queue.is_empty() {
            self.do_action();
        }
    }

    /// init the value of the nodes in the graph
    /// to do only once and if and only if all the nodes have adden
    pub fn init_graph_state(&mut self) {
        for node_id in self.nodes.keys() {
            self.actions_queue
                .push_back((NodeAction::InitNode, *node_id));
        }
        self.do_actions();
    }
}

#[cfg(test)]
mod tests {
    use crate::LogicBlock;

    use super::*;

    #[test]
    fn binary_or() {
        /*
         * A > c
         * B > c
         * A -> B = lamp
         * C = OR
         * */
        let mut graph = Graph::new();
        let mut nodes = Vec::new();
        let input_a = Node::LogicBlock(LogicBlock::new(0b11111, vec![]));
        nodes.push((input_a, 1));
        let input_b = Node::LogicBlock(LogicBlock::new(0b11111, vec![]));
        nodes.push((input_b, 2));
        // output (binary or)
        let output = Node::LogicBlock(LogicBlock::new(0b00110, vec![]));
        nodes.push((output, 3));
        graph.insert_nodes(nodes);
        graph.insert_links(vec![(1, 3), (2, 3)]);
        assert!(graph.get_logical_block(1).unwrap().children == [3]);
        assert!(graph.get_logical_block(2).unwrap().children == [3]);
        assert!(graph.get_logical_block(3).unwrap().children == []);
        // add the init actions
        graph.init_graph_state();
        assert!(graph.get_logical_block(1).unwrap().get_value() == 0);
        assert!(graph.get_logical_block(2).unwrap().get_value() == 0);
        assert!(graph.get_logical_block(3).unwrap().get_value() == 2);
        assert!(graph.get_logical_block(1).unwrap().is_on());
        assert!(graph.get_logical_block(2).unwrap().is_on());
        assert!(graph.get_logical_block(3).unwrap().is_on());
    }
}
