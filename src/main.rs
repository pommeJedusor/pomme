use binary_world::Graph;
use binary_world::LogicBlock;
use binary_world::Node;

fn main() {
    let mut graph = Graph::new();
    let block = LogicBlock::new(0, vec![]);
    graph.insert_nodes(vec![(Node::LogicBlock(block), 0)]);
}
