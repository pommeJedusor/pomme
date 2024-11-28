use crate::LogicBlock;
use crate::StoringBlock;

#[derive(Debug)]
pub enum Node {
    LogicBlock(LogicBlock),
    StoringBlock(StoringBlock),
}

impl Node {
    pub fn is_on(&self) -> bool {
        match self {
            Node::LogicBlock(node) => node.is_on(),
            Node::StoringBlock(node) => node.is_on,
        }
    }
    pub fn get_children(&self) -> &Vec<u32> {
        match self {
            Node::LogicBlock(node) => &node.children,
            Node::StoringBlock(node) => &node.children,
        }
    }
}
