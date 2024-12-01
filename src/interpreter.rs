use crate::{Graph, LogicBlock, Node};
use std::fs;

pub fn init_map(path: &str) -> Graph {
    let contents = fs::read_to_string(path).expect(&format!("Failed to read the file: {}", path));
    let lines = contents.split("\n").filter(|x| x != &"");
    let mut nodes = Vec::new();
    for (i, line) in lines.enumerate() {
        let parameters = line.split_whitespace().collect::<Vec<&str>>();
        assert!(parameters.len() >= 2, "line {i} is not valid:\n{line}");
        let node_id = parameters[0].parse::<u32>().unwrap();
        let requirements = u8::from_str_radix(parameters[1], 2).unwrap();
        let children = parameters
            .iter()
            .skip(2)
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let node = Node::LogicBlock(LogicBlock::new(requirements, children));
        nodes.push((node, node_id));
    }
    let mut graph = Graph::new();
    graph.insert_nodes(nodes);
    graph.init_graph_state();
    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut map = init_map("./components/adder.bwc");
        // init input 1 to 96 (0b01100000)
        map.turn_on_lamp(6);
        map.turn_on_lamp(7);
        // init input 2 to 37 (0b00100101)
        map.turn_on_lamp(9);
        map.turn_on_lamp(11);
        map.turn_on_lamp(14);
        // check output is 133 (0b10000101)
        map.apply_changes();
        assert!(map.get_node(17).unwrap().is_on());
        assert!(!map.get_node(18).unwrap().is_on());
        assert!(map.get_node(19).unwrap().is_on());
        assert!(!map.get_node(20).unwrap().is_on());
        assert!(!map.get_node(21).unwrap().is_on());
        assert!(!map.get_node(22).unwrap().is_on());
        assert!(!map.get_node(23).unwrap().is_on());
        assert!(map.get_node(24).unwrap().is_on());
    }
}
