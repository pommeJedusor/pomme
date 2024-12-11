use pomme::Graph;
use pomme::LogicBlock;
use pomme::Node;
use pomme::StoringBlock;

fn main() {
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
