use crate::LogicBlock;
use crate::Node;
use crate::StoringBlock;
use std::collections::HashMap;
use std::collections::VecDeque;

type NodeId = u32;

#[derive(Debug, Copy, Clone)]
pub enum ChangeValue {
    IncreaseValue,
    DecreaseValue,
}

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
        for link in links {
            assert!(self.nodes.contains_key(&link.0));
            assert!(self.nodes.contains_key(&link.1));

            let is_first_storing_block =
                matches!(self.nodes.get(&link.0).unwrap(), Node::StoringBlock(_));
            let is_second_storing_block =
                matches!(self.nodes.get(&link.1).unwrap(), Node::StoringBlock(_));
            assert!(!is_first_storing_block || !is_second_storing_block);

            if is_first_storing_block {
                let first_node = self.get_mut_storing_block(link.0).unwrap();
                first_node.children.push(link.1);
            } else {
                let first_node = self.get_mut_logical_block(link.0).unwrap();
                first_node.children.push(link.1);
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

            let storing_block = self.get_mut_storing_block(storing_block_link).unwrap();
            storing_block.button_node = logical_block_link;
            let logical_block = self.get_mut_logical_block(logical_block_link).unwrap();
            logical_block.children.push(storing_block_link);
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
            Node::StoringBlock(_) => None,
        }
    }
    pub fn get_storing_block(&self, key: u32) -> Option<&StoringBlock> {
        match self.nodes.get(&key)? {
            Node::LogicBlock(_) => None,
            Node::StoringBlock(node) => Some(node),
        }
    }
    pub fn get_mut_logical_block(&mut self, key: u32) -> Option<&mut LogicBlock> {
        match self.nodes.get_mut(&key)? {
            Node::LogicBlock(node) => Some(node),
            Node::StoringBlock(_) => None,
        }
    }
    pub fn get_mut_storing_block(&mut self, key: u32) -> Option<&mut StoringBlock> {
        match self.nodes.get_mut(&key)? {
            Node::LogicBlock(_) => None,
            Node::StoringBlock(node) => Some(node),
        }
    }

    /// only when initialising the graph
    /// add add increasevalue action to all children if the node is on and if it hasn't been
    /// explored during the initialisation (if the value is 0)
    fn init_node(&mut self, node_id: u32) {
        let node = self.get_mut_node(node_id).expect("node not found");
        if !node.is_on() {
            return;
        }
        for child in node.get_children().clone() {
            self.actions_queue
                .push_back((NodeAction::IncreaseValue, child));
        }
    }

    fn update_storing_node_value(&mut self, node_id: u32) -> bool {
        let node = self.get_storing_block(node_id).unwrap();
        let is_source_on = self.get_logical_block(node.source).unwrap().is_on();
        let is_button_node_on = self.get_logical_block(node.button_node).unwrap().is_on();

        let node = self.get_mut_storing_block(node_id).unwrap();
        if is_button_node_on {
            node.is_on = is_source_on;
        }
        node.is_on
    }

    fn update_logic_node_value(&mut self, node_id: u32, change_value: ChangeValue) {
        let node = self.get_mut_logical_block(node_id).unwrap();
        let was_on = node.is_on();
        let new_value = match change_value {
            ChangeValue::IncreaseValue => node.get_value() + 1,
            ChangeValue::DecreaseValue => node.get_value() - 1,
        };
        node.set_value(new_value);
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

    fn update_value(&mut self, node_id: u32, change_value: ChangeValue) {
        let node = self.get_mut_node(node_id).expect("node not found");
        match node {
            Node::LogicBlock(_) => {
                self.update_logic_node_value(node_id, change_value);
            }
            Node::StoringBlock(_) => {
                self.update_storing_node_value(node_id);
            }
        }
    }

    pub fn do_action(&mut self) {
        let action = self.actions_queue.pop_front();
        if action.is_none() {
            return;
        }
        let (action, node) = action.unwrap();
        match action {
            NodeAction::InitNode => self.init_node(node),
            NodeAction::IncreaseValue => self.update_value(node, ChangeValue::IncreaseValue),
            NodeAction::DecreaseValue => self.update_value(node, ChangeValue::DecreaseValue),
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

    #[test]
    fn binary_and() {
        /*
         * A > c
         * B > c
         * A -> B = lamp
         * C = AND
         * */
        let mut graph = Graph::new();
        let mut nodes = Vec::new();
        let input_a = Node::LogicBlock(LogicBlock::new(0b11111, vec![]));
        nodes.push((input_a, 1));
        let input_b = Node::LogicBlock(LogicBlock::new(0b11111, vec![]));
        nodes.push((input_b, 2));
        // output (binary or)
        let output = Node::LogicBlock(LogicBlock::new(0b00100, vec![]));
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

    #[test]
    fn binary_long() {
        /*
         * A > e
         * D > e
         * B > f
         * C > f
         * E > g
         * F > g
         * E = OR
         * F = AND
         * G = AND
         * A -> C = lamp
         * */
        let mut graph = Graph::new();
        let mut nodes = Vec::new();

        let input_a = Node::LogicBlock(LogicBlock::new(0b11111, vec![]));
        nodes.push((input_a, 1));
        let input_b = Node::LogicBlock(LogicBlock::new(0b11111, vec![]));
        nodes.push((input_b, 2));
        let input_c = Node::LogicBlock(LogicBlock::new(0b11111, vec![]));
        nodes.push((input_c, 3));
        let input_d = Node::LogicBlock(LogicBlock::new(0b00000, vec![]));
        nodes.push((input_d, 4));

        let input_e = Node::LogicBlock(LogicBlock::new(0b00110, vec![]));
        nodes.push((input_e, 5));
        let input_f = Node::LogicBlock(LogicBlock::new(0b00100, vec![]));
        nodes.push((input_f, 6));
        // output
        let input_g = Node::LogicBlock(LogicBlock::new(0b00100, vec![]));
        nodes.push((input_g, 7));

        // output (binary or)
        graph.insert_nodes(nodes);
        graph.insert_links(vec![(1, 5), (4, 5)]);
        graph.insert_links(vec![(2, 6), (3, 6)]);
        graph.insert_links(vec![(5, 7), (6, 7)]);
        assert!(graph.get_logical_block(1).unwrap().children == [5]);
        assert!(graph.get_logical_block(2).unwrap().children == [6]);
        assert!(graph.get_logical_block(3).unwrap().children == [6]);
        assert!(graph.get_logical_block(4).unwrap().children == [5]);
        assert!(graph.get_logical_block(5).unwrap().children == [7]);
        assert!(graph.get_logical_block(6).unwrap().children == [7]);
        assert!(graph.get_logical_block(7).unwrap().children == []);
        // add the init actions
        graph.init_graph_state();
        assert!(graph.get_logical_block(1).unwrap().get_value() == 0);
        assert!(graph.get_logical_block(2).unwrap().get_value() == 0);
        assert!(graph.get_logical_block(3).unwrap().get_value() == 0);
        assert!(graph.get_logical_block(4).unwrap().get_value() == 0);
        assert!(graph.get_logical_block(5).unwrap().get_value() == 1);
        assert!(graph.get_logical_block(6).unwrap().get_value() == 2);
        assert!(graph.get_logical_block(7).unwrap().get_value() == 2);
        assert!(graph.get_logical_block(1).unwrap().is_on());
        assert!(graph.get_logical_block(2).unwrap().is_on());
        assert!(graph.get_logical_block(3).unwrap().is_on());
        assert!(!graph.get_logical_block(4).unwrap().is_on());
        assert!(graph.get_logical_block(5).unwrap().is_on());
        assert!(graph.get_logical_block(6).unwrap().is_on());
        assert!(graph.get_logical_block(7).unwrap().is_on());
    }

    #[test]
    fn test_storing_block1() {
        /*
         * A > c
         * B > c
         * C > ^e
         * D - ^e
         * A -> B = lamp
         * C = AND
         * D = lamp / stone
         * */
        let mut graph = Graph::new();
        let mut nodes = Vec::new();

        let input_a = Node::LogicBlock(LogicBlock::new(0b11111, vec![]));
        nodes.push((input_a, 1));
        let input_b = Node::LogicBlock(LogicBlock::new(0b11111, vec![]));
        nodes.push((input_b, 2));
        let input_c = Node::LogicBlock(LogicBlock::new(0b00100, vec![]));
        nodes.push((input_c, 3));
        let input_d = Node::LogicBlock(LogicBlock::new(0b00000, vec![]));
        nodes.push((input_d, 4));
        let input_e = Node::StoringBlock(StoringBlock::new(false, 3, 4, vec![]));
        nodes.push((input_e, 5));

        graph.insert_nodes(nodes);

        graph.insert_links(vec![(1, 3), (2, 3)]);
        graph.insert_links(vec![(3, 5), (4, 5)]);

        graph.init_graph_state();

        assert!(graph.get_logical_block(1).unwrap().get_value() == 0);
        assert!(graph.get_logical_block(2).unwrap().get_value() == 0);
        assert!(graph.get_logical_block(3).unwrap().get_value() == 2);
        assert!(graph.get_logical_block(4).unwrap().get_value() == 0);

        assert!(graph.get_logical_block(1).unwrap().is_on() == true);
        assert!(graph.get_logical_block(2).unwrap().is_on() == true);
        assert!(graph.get_logical_block(3).unwrap().is_on() == true);
        assert!(graph.get_logical_block(4).unwrap().is_on() == false);
        assert!(graph.get_storing_block(5).unwrap().is_on == false);
    }

    #[test]
    fn test_storing_block2() {
        /*
         * A > c
         * B > c
         * C > ^e
         * D - ^e
         * A -> B = lamp
         * C = AND
         * D = lamp / stone
         * */
        let mut graph = Graph::new();
        let mut nodes = Vec::new();

        let input_a = Node::LogicBlock(LogicBlock::new(0b11111, vec![]));
        nodes.push((input_a, 1));
        let input_b = Node::LogicBlock(LogicBlock::new(0b11111, vec![]));
        nodes.push((input_b, 2));
        let input_c = Node::LogicBlock(LogicBlock::new(0b00100, vec![]));
        nodes.push((input_c, 3));
        let input_d = Node::LogicBlock(LogicBlock::new(0b00000, vec![]));
        nodes.push((input_d, 4));
        let input_e = Node::StoringBlock(StoringBlock::new(true, 3, 4, vec![]));
        nodes.push((input_e, 5));

        graph.insert_nodes(nodes);

        graph.insert_links(vec![(1, 3), (2, 3)]);
        graph.insert_links(vec![(3, 5), (4, 5)]);

        graph.init_graph_state();

        assert!(graph.get_logical_block(1).unwrap().get_value() == 0);
        assert!(graph.get_logical_block(2).unwrap().get_value() == 0);
        assert!(graph.get_logical_block(3).unwrap().get_value() == 2);
        assert!(graph.get_logical_block(4).unwrap().get_value() == 0);

        assert!(graph.get_logical_block(1).unwrap().is_on() == true);
        assert!(graph.get_logical_block(2).unwrap().is_on() == true);
        assert!(graph.get_logical_block(3).unwrap().is_on() == true);
        assert!(graph.get_logical_block(4).unwrap().is_on() == false);
        assert!(graph.get_storing_block(5).unwrap().is_on == true);
    }

    #[test]
    fn test_storing_block3() {
        /*
         * A > c
         * B > c
         * C > ^e
         * D - ^e
         * A -> B = lamp
         * C = AND
         * D = lamp / stone
         * */
        let mut graph = Graph::new();
        let mut nodes = Vec::new();

        let input_a = Node::LogicBlock(LogicBlock::new(0b11111, vec![]));
        nodes.push((input_a, 1));
        let input_b = Node::LogicBlock(LogicBlock::new(0b11111, vec![]));
        nodes.push((input_b, 2));
        let input_c = Node::LogicBlock(LogicBlock::new(0b00100, vec![]));
        nodes.push((input_c, 3));
        let input_d = Node::LogicBlock(LogicBlock::new(0b11111, vec![]));
        nodes.push((input_d, 4));
        let input_e = Node::StoringBlock(StoringBlock::new(false, 3, 4, vec![]));
        nodes.push((input_e, 5));

        graph.insert_nodes(nodes);

        graph.insert_links(vec![(1, 3), (2, 3)]);
        graph.insert_links(vec![(3, 5), (4, 5)]);

        graph.init_graph_state();

        assert!(graph.get_logical_block(1).unwrap().get_value() == 0);
        assert!(graph.get_logical_block(2).unwrap().get_value() == 0);
        assert!(graph.get_logical_block(3).unwrap().get_value() == 2);
        assert!(graph.get_logical_block(4).unwrap().get_value() == 0);

        assert!(graph.get_logical_block(1).unwrap().is_on() == true);
        assert!(graph.get_logical_block(2).unwrap().is_on() == true);
        assert!(graph.get_logical_block(3).unwrap().is_on() == true);
        assert!(graph.get_logical_block(4).unwrap().is_on() == true);
        assert!(graph.get_storing_block(5).unwrap().is_on == true);
    }

    #[test]
    fn test_storing_block4() {
        /*
         * A > c
         * B > c
         * C > ^e
         * D - ^e
         * A -> B = lamp
         * C = AND
         * D = lamp / stone
         * */
        let mut graph = Graph::new();
        let mut nodes = Vec::new();

        let input_a = Node::LogicBlock(LogicBlock::new(0b00000, vec![]));
        nodes.push((input_a, 1));
        let input_b = Node::LogicBlock(LogicBlock::new(0b11111, vec![]));
        nodes.push((input_b, 2));
        let input_c = Node::LogicBlock(LogicBlock::new(0b00100, vec![]));
        nodes.push((input_c, 3));
        let input_d = Node::LogicBlock(LogicBlock::new(0b11111, vec![]));
        nodes.push((input_d, 4));
        let input_e = Node::StoringBlock(StoringBlock::new(true, 3, 4, vec![]));
        nodes.push((input_e, 5));

        graph.insert_nodes(nodes);

        graph.insert_links(vec![(1, 3), (2, 3)]);
        graph.insert_links(vec![(3, 5), (4, 5)]);

        graph.init_graph_state();

        assert!(graph.get_logical_block(1).unwrap().get_value() == 0);
        assert!(graph.get_logical_block(2).unwrap().get_value() == 0);
        assert!(graph.get_logical_block(3).unwrap().get_value() == 1);
        assert!(graph.get_logical_block(4).unwrap().get_value() == 0);

        assert!(graph.get_logical_block(1).unwrap().is_on() == false);
        assert!(graph.get_logical_block(2).unwrap().is_on() == true);
        assert!(graph.get_logical_block(3).unwrap().is_on() == false);
        assert!(graph.get_logical_block(4).unwrap().is_on() == true);
        assert!(graph.get_storing_block(5).unwrap().is_on == false);
    }
    //#[test]
    //fn boucle() {
    //    /*
    //     * A > b
    //     * B > c
    //     * C > a
    //     * A strictly needs 0
    //     * B strictly needs 1
    //     * C strictly needs 1
    //     * */
    //    let mut graph = Graph::new();
    //    let mut nodes = Vec::new();
    //
    //    let input_a = Node::LogicBlock(LogicBlock::new(0b00001, vec![]));
    //    nodes.push((input_a, 1));
    //    let input_b = Node::LogicBlock(LogicBlock::new(0b00010, vec![]));
    //    nodes.push((input_b, 2));
    //    let input_c = Node::LogicBlock(LogicBlock::new(0b00010, vec![]));
    //    nodes.push((input_c, 3));
    //
    //    graph.insert_nodes(nodes);
    //
    //    graph.insert_links(vec![(1, 2), (2, 3), (3, 1)]);
    //
    //    graph.init_graph_state();
    //}
}
