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

    /* getters */
    pub fn get_node(&self, key: u32) -> Option<&Node> {
        self.nodes.get(&key)
    }

    fn get_mut_node(&mut self, key: u32) -> Option<&mut Node> {
        self.nodes.get_mut(&key)
    }

    pub fn get_logical_block(&self, key: u32) -> Option<&LogicBlock> {
        match self.nodes.get(&key)? {
            Node::LogicBlock(node) => Some(node),
            Node::StoringBlock(_) => None,
        }
    }

    fn get_mut_logical_block(&mut self, key: u32) -> Option<&mut LogicBlock> {
        match self.nodes.get_mut(&key)? {
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

    fn get_mut_storing_block(&mut self, key: u32) -> Option<&mut StoringBlock> {
        match self.nodes.get_mut(&key)? {
            Node::LogicBlock(_) => None,
            Node::StoringBlock(node) => Some(node),
        }
    }

    /* pub methods */
    pub fn insert_nodes(&mut self, nodes: Vec<(Node, NodeId)>) {
        for (node, id) in nodes {
            assert!(!self.nodes.contains_key(&id));
            self.nodes.insert(id, node);
        }
    }

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

    /// init the value of the nodes in the graph
    /// to do only once and if and only if all the nodes have adden
    pub fn init_graph_state(&mut self) {
        for node_id in self.nodes.keys() {
            self.actions_queue
                .push_back((NodeAction::InitNode, *node_id));
        }
        self.do_actions();
    }

    pub fn turn_on_lamp(&mut self, node_id: u32) {
        let node = self
            .get_mut_logical_block(node_id)
            .expect("can't turn on storing block");
        assert!(
            node.get_requirements() != 0b11111,
            "lamp already turned on ({})",
            node_id
        );
        assert!(
            node.get_requirements() == 0b00000,
            "it is not allowed to turn on a none rock block ({})",
            node_id
        );
        node.set_requirements(0b11111);
        for child in node.children.clone() {
            self.actions_queue
                .push_back((NodeAction::IncreaseValue, child));
        }
    }

    pub fn turn_off_lamp(&mut self, node_id: u32) {
        let node = self
            .get_mut_logical_block(node_id)
            .expect("can't turn on storing block");
        assert!(
            node.get_requirements() != 0b00000,
            "lamp already turned off ({})",
            node_id
        );
        assert!(
            node.get_requirements() == 0b11111,
            "it is not allowed to turn off a none lamp block ({})",
            node_id
        );
        node.set_requirements(0b00000);
        for child in node.children.clone() {
            self.actions_queue
                .push_back((NodeAction::DecreaseValue, child));
        }
    }

    pub fn apply_changes(&mut self) {
        self.do_actions();
    }

    /* privte methods*/
    /// must only be used when initialising the graph
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

    fn do_action(&mut self) {
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

    fn do_actions(&mut self) {
        while !self.actions_queue.is_empty() {
            self.do_action();
        }
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

        assert!(!graph.get_logical_block(1).unwrap().is_on());
        assert!(graph.get_logical_block(2).unwrap().is_on());
        assert!(!graph.get_logical_block(3).unwrap().is_on());
        assert!(graph.get_logical_block(4).unwrap().is_on());
        assert!(!graph.get_storing_block(5).unwrap().is_on);

        graph.turn_on_lamp(1);
        graph.do_actions();
        assert!(graph.get_storing_block(5).unwrap().is_on);
        graph.turn_off_lamp(1);
        graph.turn_off_lamp(4);
        graph.do_actions();
        assert!(graph.get_storing_block(5).unwrap().is_on);
        graph.turn_on_lamp(4);
        graph.do_actions();
        assert!(!graph.get_storing_block(5).unwrap().is_on);
    }

    #[test]
    fn test_adder() {
        /*
         * */
        let mut graph = Graph::new();
        let mut nodes = Vec::new();

        // input 1
        nodes.push((Node::LogicBlock(LogicBlock::new(0b00000, vec![])), 1));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b00000, vec![])), 2));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b00000, vec![])), 3));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b00000, vec![])), 4));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b00000, vec![])), 5));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b00000, vec![])), 6));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b00000, vec![])), 7));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b00000, vec![])), 8));

        // input 2
        nodes.push((Node::LogicBlock(LogicBlock::new(0b00000, vec![])), 9));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b00000, vec![])), 10));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b00000, vec![])), 11));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b00000, vec![])), 12));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b00000, vec![])), 13));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b00000, vec![])), 14));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b00000, vec![])), 15));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b00000, vec![])), 16));

        // ouput
        nodes.push((Node::LogicBlock(LogicBlock::new(0b01010, vec![])), 17));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b01010, vec![])), 18));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b01010, vec![])), 19));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b01010, vec![])), 20));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b01010, vec![])), 21));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b01010, vec![])), 22));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b01010, vec![])), 23));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b01010, vec![])), 24));

        // rest
        nodes.push((Node::LogicBlock(LogicBlock::new(0b01100, vec![])), 25));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b01100, vec![])), 26));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b01100, vec![])), 27));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b01100, vec![])), 28));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b01100, vec![])), 29));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b01100, vec![])), 30));
        nodes.push((Node::LogicBlock(LogicBlock::new(0b01100, vec![])), 31));

        graph.insert_nodes(nodes);

        /* from rightest bit to leftest */
        // bit 1
        graph.insert_links(vec![(1, 17), (9, 17), (1, 25), (9, 25)]);

        // bit 2
        graph.insert_links(vec![
            (2, 18),
            (10, 18),
            (2, 26),
            (10, 26),
            (25, 18),
            (25, 26),
        ]);

        // bit 3
        graph.insert_links(vec![
            (3, 19),
            (11, 19),
            (3, 27),
            (11, 27),
            (26, 19),
            (26, 27),
        ]);

        // bit 4
        graph.insert_links(vec![
            (4, 20),
            (12, 20),
            (4, 28),
            (12, 28),
            (27, 20),
            (27, 28),
        ]);

        // bit 5
        graph.insert_links(vec![
            (5, 21),
            (13, 21),
            (5, 29),
            (13, 29),
            (28, 21),
            (28, 29),
        ]);

        // bit 6
        graph.insert_links(vec![
            (6, 22),
            (14, 22),
            (6, 30),
            (14, 30),
            (29, 22),
            (29, 30),
        ]);

        // bit 7
        graph.insert_links(vec![
            (7, 23),
            (15, 23),
            (7, 31),
            (15, 31),
            (30, 23),
            (30, 31),
        ]);

        // bit 8
        graph.insert_links(vec![(8, 23), (16, 23), (31, 23)]);

        graph.init_graph_state();

        // set input 1 to 6
        graph.turn_on_lamp(2);
        graph.turn_on_lamp(3);

        // set input 2 to 33
        graph.turn_on_lamp(1);
        graph.turn_on_lamp(6);

        graph.apply_changes();

        // assert that the result is 39 (0b00100111)
        assert!(graph.get_node(17).unwrap().is_on());
        assert!(graph.get_node(18).unwrap().is_on());
        assert!(graph.get_node(19).unwrap().is_on());
        assert!(!graph.get_node(20).unwrap().is_on());
        assert!(!graph.get_node(21).unwrap().is_on());
        assert!(graph.get_node(22).unwrap().is_on());
        assert!(!graph.get_node(23).unwrap().is_on());
        assert!(!graph.get_node(24).unwrap().is_on());
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
