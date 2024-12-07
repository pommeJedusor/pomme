use std::{
    collections::{HashMap, VecDeque},
    fs::{self, File},
    io::Write,
};

enum LinkLineType {
    LinkDeclaration,
    Condition,
    Boucle,
}

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

fn is_line_link_declaration(line: &str) -> bool {
    true
}

fn get_link_line_type(line: &str) -> LinkLineType {
    if line.starts_with("for") {
        return LinkLineType::Boucle;
    }
    LinkLineType::LinkDeclaration
}

pub fn apply_variable(node: &str, variables: &mut HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut stack = String::new();
    for letter in node.chars() {
        assert!(!(letter == '$' && stack.len() > 0));
        if stack.len() != 0 || letter == '$' {
            stack.push(letter);
            if variables.contains_key(&stack) {
                result.push_str(variables.get(&stack).unwrap());
                stack = String::new();
            }
        } else {
            result.push(letter);
        }
    }
    assert!(stack.len() == 0, "node: {}, stack: {}", node, stack);
    result
}

pub fn analyse_links_part(
    lines: &Vec<&str>,
    variables: &mut HashMap<String, String>,
    nb_block_leading_spaces: u8,
) -> Vec<(String, String)> {
    let mut links: Vec<(String, String)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let nb_line_leading_spaces = line.chars().position(|x| x != ' ').unwrap() as u8;
        let line = line.trim();
        match nb_line_leading_spaces.cmp(&nb_block_leading_spaces) {
            std::cmp::Ordering::Less => return links,
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => continue,
        };

        match get_link_line_type(line) {
            LinkLineType::LinkDeclaration => {
                let result = line.split("->").collect::<Vec<&str>>();
                let sources = result[0]
                    .trim()
                    .split(",")
                    .map(|x| apply_variable(x.trim(), variables))
                    .collect::<Vec<String>>();
                let targets = result[1]
                    .trim()
                    .split(",")
                    .map(|x| apply_variable(x.trim(), variables))
                    .collect::<Vec<String>>();
                for source in sources.clone() {
                    for target in targets.clone() {
                        links.push((source.to_string(), target.to_string()));
                    }
                }
            }

            LinkLineType::Boucle => {
                let mut line = line.split(" ").filter(|x| x != &" ");
                let variable_name = line.nth(1).unwrap();
                let range = (line.nth(1).unwrap(), line.nth(1).unwrap());
                let nodes = get_nodes_of_range(range.0, range.1);
                assert!(variable_name.starts_with("$"));
                for node in nodes {
                    variables.insert(variable_name.to_string(), node);
                    let lines = lines.iter().map(|&x| x).skip(i + 1).collect::<Vec<&str>>();
                    let boucle_indent = lines
                        .iter()
                        .skip(i)
                        .next()
                        .unwrap()
                        .chars()
                        .position(|x| x != ' ')
                        .unwrap();
                    let boucle_links = analyse_links_part(&lines, variables, boucle_indent as u8);
                    for link in boucle_links {
                        links.push(link);
                    }
                }
            }
            _ => {}
        }
    }
    links
}

pub fn compile(path: &str) {
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
    //println!("{:#?}", sections);

    // read the sections to define nodes and their links
    let mut nodes_queue: Vec<String> = Vec::new();
    let mut nodes_hashmap: HashMap<String, Vec<String>> = HashMap::new();
    let mut nodes_requirements = HashMap::new();
    // TODO: imports
    // inputs
    let input_section = sections.get("inputs").expect("inputs field is missing");
    for line in input_section {
        let line: Vec<&str> = line.split("->").map(|x| x.trim()).collect();
        // if just one node
        if line.len() == 1 {
            nodes_queue.push(line[0].trim().to_string());
            nodes_hashmap.insert(line[0].trim().to_string(), vec![]);
            nodes_requirements.insert(line[0].trim().to_string(), 0b00000);
        }
        // if range of nodes
        else {
            let nodes = get_nodes_of_range(line[0].trim(), line[1].trim());
            for node in nodes {
                nodes_queue.push(node.clone().to_string());
                nodes_hashmap.insert(node.clone().to_string(), vec![]);
                nodes_requirements.insert(node.trim().to_string(), 0b00000);
            }
        }
    }

    // outputs
    let output_section = sections.get("outputs").expect("outputs field is missing");
    for line in output_section {
        let line: Vec<&str> = line.split("->").map(|x| x.trim()).collect();
        let requirements = line[1]
            .split_once(" ")
            .unwrap()
            .1
            .split(",")
            .map(|x| 1 << x.trim().parse::<u32>().unwrap())
            .fold(0, |a, b| a | b);
        // if just one node
        if line.len() == 1 {
            nodes_queue.push(line[0].trim().to_string());
            nodes_hashmap.insert(line[0].trim().to_string(), vec![]);
            nodes_requirements.insert(line[0].trim().to_string(), requirements);
        }
        // if range of nodes
        else {
            let nodes =
                get_nodes_of_range(line[0].trim(), line[1].trim().split(" ").nth(0).unwrap());
            for node in nodes {
                nodes_queue.push(node.clone().to_string());
                nodes_hashmap.insert(node.clone().to_string(), vec![]);
                nodes_requirements.insert(node.trim().to_string(), requirements);
            }
        }
    }

    // def
    let def_section = sections.get("def").expect("def field is missing");
    for line in def_section {
        let line: Vec<&str> = line.split("->").map(|x| x.trim()).collect();
        let requirements = line[1]
            .split_once(" ")
            .unwrap()
            .1
            .split(",")
            .map(|x| 1 << x.trim().parse::<u32>().unwrap())
            .fold(0, |a, b| a | b);
        // if just one node
        if line.len() == 1 {
            nodes_queue.push(line[0].trim().to_string());
            nodes_hashmap.insert(line[0].trim().to_string(), vec![]);
            nodes_requirements.insert(line[0].trim().to_string(), requirements);
        }
        // if range of nodes
        else {
            let nodes =
                get_nodes_of_range(line[0].trim(), line[1].trim().split(" ").nth(0).unwrap());
            for node in nodes {
                nodes_queue.push(node.clone().to_string());
                nodes_hashmap.insert(node.clone().to_string(), vec![]);
                nodes_requirements.insert(node.trim().to_string(), requirements);
            }
        }
    }
    // TODO: links
    let links_section = sections.get("links").expect("links field is missing");
    let links = analyse_links_part(links_section, &mut HashMap::new(), 0);
    for link in links {
        nodes_hashmap.entry(link.0).and_modify(|x| x.push(link.1));
    }

    //println!("{:#?}", nodes_queue);
    //println!("{:#?}", nodes_hashmap);
    //println!("{:#?}", nodes_requirements);
    // TODO: write the nodes expression in the new file
    let mut content = String::new();
    for (i, node) in nodes_queue.iter().enumerate() {
        let index = (i + 1).to_string();
        let requirements = nodes_requirements.get(node).unwrap();
        let requirements = (0..5)
            .map(|x| if requirements & 1 << x > 0 { "1" } else { "0" })
            .rev()
            .collect::<Vec<&str>>()
            .join("");
        let links = nodes_hashmap
            .get(node)
            .unwrap()
            .iter()
            .map(|x| (nodes_queue.iter().position(|el| el == x).unwrap() + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ");
        let line = format!("{} {} {}\n", index, requirements, links);
        content.push_str(&line);
    }
    let mut file = File::create(format!("{path}c")).unwrap();
    let _ = file.write(&content.into_bytes());
}

#[cfg(test)]
mod tests {
    use crate::init_map;

    use super::*;

    #[test]
    fn test() {
        compile("./components/test.bw");
        let mut map = init_map("./components/test.bwc");
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

    #[test]
    fn test2() {
        compile("./components/test2.bw");
        let mut map = init_map("./components/test2.bwc");
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
