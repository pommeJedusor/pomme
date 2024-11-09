use binary_world::Graph;
use binary_world::LogicBlock;
use binary_world::Node;

fn main() {
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
    println!("{graph:#?}");
}
