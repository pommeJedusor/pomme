use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn get_nodes_of_range<'a>(start: &'a str, end: &'a str) -> Vec<String> {
    let mut nodes = Vec::new();
    let mut current = start.to_string();
    let last = end.to_string();

    while current != last {
        nodes.push(current.clone());
        current = increment_node_name(&current);
    }
    nodes.push(last);

    nodes
}

fn increment_node_name(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    let mut must_carry = true;

    for i in (0..chars.len()).rev() {
        if must_carry {
            if chars[i] == 'Z' {
                chars[i] = 'A';
            } else {
                chars[i] = (chars[i] as u8 + 1) as char;
                must_carry = false;
            }
        }
    }

    if must_carry {
        chars.insert(0, 'A');
    }

    chars.iter().collect()
}

pub fn compile(path: &str) {
    const INDENTATION_LENGTH: u32 = 4;
    let allowed_sections = ["inputs", "outputs", "def", "links", "imports"];

    // divide the content in sections and the sections in lines also remove the comments and empty lines
    let contents = fs::read_to_string(path).expect(&format!("Failed to read the file: {}", path));
    let mut sections: HashMap<&str, Vec<&str>> = HashMap::new();
    for section in contents.split("\n\n\n") {
        let lines = section
            .split("\n")
            .filter(|x| x.len() != 0 && &x.trim()[0..=0] != "#")
            .collect::<Vec<&str>>();
        let section_name = &lines[0].trim();
        let section_name = &section_name[..section_name.len() - 1];

        assert!(
            allowed_sections.contains(&section_name),
            "section's name: {} is not allowed",
            &section_name
        );
        assert!(!sections.contains_key(&section_name));

        let section = lines
            .iter()
            .skip(1)
            .filter(|x| x.len() != 0 && &x.trim()[0..=0] != "#")
            .map(|x| *x)
            .collect();
        sections.insert(section_name, section);
    }
    println!("{:#?}", sections);

    // read the sections to define nodes and their links
    let mut nodes_queue: VecDeque<String> = VecDeque::new();
    let mut nodes_hashmap: HashMap<String, Vec<&str>> = HashMap::new();
    let mut nodes_requirements = HashMap::new();
    // inputs
    let input_section = sections.get("inputs").expect("inputs field is missing");
    for line in input_section {
        let line: Vec<&str> = line.split("->").collect();
        // if just one node
        if line.len() == 1 {
            nodes_queue.push_back(line[0].trim().to_string());
            nodes_hashmap.insert(line[0].trim().to_string(), vec![]);
            nodes_requirements.insert(line[0].trim().to_string(), 0b00000);
        }
        // if range of nodes
        else {
            let nodes = get_nodes_of_range(line[0].trim(), line[1].trim());
            for node in nodes {
                nodes_queue.push_back(node.clone().to_string());
                nodes_hashmap.insert(node.clone().to_string(), vec![]);
                nodes_requirements.insert(line[0].trim().to_string(), 0b00000);
            }
        }
    }

    // outputs
    let output_section = sections.get("outputs").expect("outputs field is missing");
    for line in output_section {
        let line: Vec<&str> = line.split("->").collect();
        // if just one node
        if line.len() == 1 {
            nodes_queue.push_back(line[0].trim().to_string());
            nodes_hashmap.insert(line[0].trim().to_string(), vec![]);
            // TODO: get requirements accordingly
            nodes_requirements.insert(line[0].trim().to_string(), 0b00000);
        }
        // if range of nodes
        else {
            let nodes =
                get_nodes_of_range(line[0].trim(), line[1].trim().split(" ").nth(0).unwrap());
            for node in nodes {
                nodes_queue.push_back(node.clone().to_string());
                nodes_hashmap.insert(node.clone().to_string(), vec![]);
                // TODO: get requirements accordingly
                nodes_requirements.insert(line[0].trim().to_string(), 0b00000);
            }
        }
    }
    println!("{:#?}", nodes_queue);
    println!("{:#?}", nodes_hashmap);
    // TODO: write the nodes expression in the new file
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        compile("./components/adder.bw");
        assert!(false);
    }
}
