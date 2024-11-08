use crate::Graph;

#[derive(Debug)]
pub struct StoringBlock {
    pub is_on: bool,
    pub source: u32,      // node from which it gets its value
    pub button_node: u32, // if and only if button_node is on the storing block take the value of
    // source
    pub children: Vec<u32>,
}

impl StoringBlock {
    pub fn set_children(&mut self, children: &Vec<u32>) {
        for child in children {
            self.children.push(*child);
        }
    }
    pub fn update(&mut self, graph: Graph) {
        let button = graph.get_node(self.button_node).expect(&format!(
            "StoringBlock of source {} don't have any button node",
            self.source
        ));
        let is_button_on = match button {
            crate::Node::LogicBlock(node) => node.is_on(),
            crate::Node::StoringBlock(node) => node.is_on,
        };
        if is_button_on {
            let is_source_on = match graph.get_node(self.source).expect(&format!(
                "StoringBlock of source {} don't have any source",
                self.source
            )) {
                crate::Node::LogicBlock(node) => node.is_on(),
                crate::Node::StoringBlock(node) => node.is_on,
            };
            self.is_on = is_source_on;
        }
    }
}
