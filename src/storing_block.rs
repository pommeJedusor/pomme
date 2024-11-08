use crate::Graph;

#[derive(Debug)]
pub struct StoringBlock {
    pub is_on: bool,
    pub source: u32,
    pub children: Vec<u32>,
}

impl StoringBlock {
    pub fn set_children(&mut self, children: &Vec<u32>) {
        for child in children {
            self.children.push(*child);
        }
    }
    pub fn update(&self, graph: Graph) {}
}
