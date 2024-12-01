use std::{collections::HashMap, fs};

pub fn compile(path: &str) {
    const INDENTATION_LENGTH: u32 = 4;

    let contents = fs::read_to_string(path).expect(&format!("Failed to read the file: {}", path));
    let mut sections: HashMap<&str, Vec<&str>> = HashMap::new();
    for section in contents.split("\n\n\n") {
        let lines = section
            .split("\n")
            .filter(|x| x.len() != 0 && &x.trim()[0..=0] != "#")
            .collect::<Vec<&str>>();
        let section_name = &lines[0].trim();
        let section_name = &section_name[..section_name.len() - 1];
        let section = lines
            .iter()
            .skip(1)
            .filter(|x| x.len() != 0 && &x.trim()[0..=0] != "#")
            .map(|x| *x)
            .collect();
        sections.insert(section_name, section);
    }
    println!("{:#?}", sections);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        compile("./components/adder.bw");
        assert!(true);
    }
}
