use crate::LogicBlock;
use crate::StoringBlock;

#[derive(Debug)]
pub enum Node {
    LogicBlock(LogicBlock),
    StoringBlock(StoringBlock),
}
