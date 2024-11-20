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
    pub fn new(is_on: bool, source: u32, button_node: u32, children: Vec<u32>) -> StoringBlock {
        StoringBlock {
            is_on,
            source,
            button_node,
            children,
        }
    }
}
