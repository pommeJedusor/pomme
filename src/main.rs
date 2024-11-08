use binary_world::Graph;
use binary_world::Node;

fn main() {
    let mut graph = Graph::new();
    graph.insert_nodes(vec![(Node::new(), 0)]);
}
