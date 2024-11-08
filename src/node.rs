pub struct Node {
    data: u8,
    pub children: Vec<u32>,
}

impl Node {
    pub fn get_value(&self) -> u8 {
        self.data >> 6
    }
    pub fn set_value(&mut self, value: u8) {
        assert!(value < 4);
        self.data = self.data & 0b00111111 | value << 6;
    }

    pub fn get_requirements(&self) -> u8 {
        self.data >> 2 & 0b1111
    }
    pub fn set_requirements(&mut self, value: u8) {
        assert!(value < 16);
        self.data = self.data & 0b11000011 | value << 2;
    }

    pub fn is_storage(&self) -> bool {
        self.data >> 1 & 1 != 0
    }
    pub fn set_storage(&mut self, value: bool) {
        let value = match value {
            true => 1,
            false => 0,
        };
        self.data = self.data & 0b11111101 | value << 1;
    }

    pub fn is_on(&self) -> bool {
        self.data & 1 != 0
    }
    pub fn set_on(&mut self, value: bool) {
        let value = match value {
            true => 1,
            false => 0,
        };
        self.data = self.data & 0b11111110 | value;
    }
}

impl From<u8> for Node {
    fn from(data: u8) -> Self {
        let children = Vec::new();
        Node { data, children }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_value_test() {
        let node = Node::from(0b00111101);
        assert!(node.get_value() == 0);
        let node = Node::from(0b11000010);
        assert!(node.get_value() == 3);
        let node = Node::from(0b10100101);
        assert!(node.get_value() == 2);
    }
    #[test]
    fn set_value_test() {
        let mut node = Node::from(0b00111101);
        assert!(node.get_value() == 0);
        node.set_value(2);
        assert!(node.get_value() == 2);
        let mut node = Node::from(0b11000010);
        assert!(node.get_value() == 3);
        node.set_value(1);
        assert!(node.get_value() == 1);
        let mut node = Node::from(0b11100101);
        assert!(node.get_value() == 3);
        node.set_value(0);
        assert!(node.get_value() == 0);
    }
    #[test]
    #[should_panic]
    fn set_value_test_panic() {
        let mut node = Node::from(0b00111101);
        node.set_value(4);
        let mut node = Node::from(0b00000000);
        node.set_value(5);
    }

    #[test]
    fn get_requirements_test() {
        let node = Node::from(0b00111101);
        assert!(node.get_requirements() == 15);
        let node = Node::from(0b11000010);
        assert!(node.get_requirements() == 0);
        let node = Node::from(0b10100101);
        assert!(node.get_requirements() == 9);
    }
    #[test]
    fn set_requirements_test() {
        let mut node = Node::from(0b00111101);
        assert!(node.get_requirements() == 15);
        node.set_requirements(2);
        assert!(node.get_requirements() == 2);
        let mut node = Node::from(0b11000010);
        assert!(node.get_requirements() == 0);
        node.set_requirements(7);
        assert!(node.get_requirements() == 7);
        let mut node = Node::from(0b11100101);
        assert!(node.get_requirements() == 9);
        node.set_requirements(0);
        assert!(node.get_requirements() == 0);
    }
    #[test]
    #[should_panic]
    fn set_requirements_test_panic() {
        let mut node = Node::from(0b00111101);
        node.set_requirements(16);
        let mut node = Node::from(0b00000000);
        node.set_value(17);
    }

    #[test]
    fn is_storage_test() {
        let node = Node::from(0b00111101);
        assert!(node.is_storage() == false);
        let node = Node::from(0b11000010);
        assert!(node.is_storage() == true);
        let node = Node::from(0b10100101);
        assert!(node.is_storage() == false);
    }
    #[test]
    fn set_storage_test() {
        let mut node = Node::from(0b00111101);
        assert!(node.is_storage() == false);
        node.set_storage(true);
        assert!(node.is_storage() == true);
        let mut node = Node::from(0b11000010);
        assert!(node.is_storage() == true);
        node.set_storage(false);
        assert!(node.is_storage() == false);
        let mut node = Node::from(0b11100101);
        assert!(node.is_storage() == false);
        node.set_storage(false);
        assert!(node.is_storage() == false);
    }

    #[test]
    fn is_on_test() {
        let node = Node::from(0b00111101);
        assert!(node.is_on() == true);
        let node = Node::from(0b11000010);
        assert!(node.is_on() == false);
        let node = Node::from(0b10100101);
        assert!(node.is_on() == true);
    }
    #[test]
    fn set_on_test() {
        let mut node = Node::from(0b00111101);
        assert!(node.is_on() == true);
        node.set_on(false);
        assert!(node.is_on() == false);
        let mut node = Node::from(0b11000010);
        assert!(node.is_on() == false);
        node.set_on(true);
        assert!(node.is_on() == true);
        let mut node = Node::from(0b11100101);
        assert!(node.is_on() == true);
        node.set_on(true);
        assert!(node.is_on() == true);
    }
}
