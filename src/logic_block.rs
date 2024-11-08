#[derive(Debug)]
pub struct LogicBlock {
    data: u8,
    pub children: Vec<u32>,
}

impl LogicBlock {
    pub fn new(requirements: u8, children: Vec<u32>) -> Self {
        assert!(requirements < 32);
        let data = requirements;
        Self { data, children }
    }

    pub fn get_value(&self) -> u8 {
        self.data >> 5
    }
    pub fn set_value(&mut self, value: u8) {
        assert!(value < 5);
        self.data = self.data & 0b00011111 | value << 5;
    }

    pub fn get_requirements(&self) -> u8 {
        self.data & 0b11111
    }
    pub fn set_requirements(&mut self, value: u8) {
        assert!(value < 32);
        self.data = self.data & 0b11100000 | value;
    }

    pub fn is_on(&self) -> bool {
        1 << self.get_value() & self.get_requirements() != 0
    }
    pub fn turn_to_lamp(&mut self) {
        self.set_requirements(0b11111);
    }
    pub fn turn_to_rock(&mut self) {
        self.set_requirements(0b00000);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_value_test() {
        let mut node = LogicBlock::new(0b11111, vec![]);
        assert!(node.get_value() == 0);
        node.set_value(4);
        assert!(node.get_value() == 4);
        node.set_value(2);
        assert!(node.get_value() == 2);
        node.set_value(1);
        assert!(node.get_value() == 1);
        node.set_value(1);
        assert!(node.get_value() == 1);
    }
    #[test]
    #[should_panic]
    fn set_value_test_panic() {
        let mut node = LogicBlock::new(0b0000, vec![]);
        node.set_value(5);
    }

    #[test]
    fn get_requirements_test() {
        let mut node = LogicBlock::new(0b11111, vec![]);
        assert!(node.get_requirements() == 31);
        node.set_requirements(0b00000);
        assert!(node.get_requirements() == 0);
        node.set_requirements(0b01001);
        assert!(node.get_requirements() == 9);
        node.set_requirements(0b01001);
        assert!(node.get_requirements() == 9);
    }
    #[test]
    #[should_panic]
    fn set_requirements_test_panic() {
        let mut node = LogicBlock::new(0b11111, vec![]);
        node.set_requirements(32);
    }

    #[test]
    fn is_on_test() {
        let mut node = LogicBlock::new(0b11111, vec![]);
        assert!(node.is_on() == true);
        node.set_requirements(0b00000);
        assert!(node.is_on() == false);
        node.set_requirements(0b00001);
        assert!(node.is_on() == true);
    }
    #[test]
    fn set_on_test() {
        let mut node = LogicBlock::new(0b11111, vec![]);
        assert!(node.is_on() == true);
        node.turn_to_rock();
        assert!(node.is_on() == false);
        node.turn_to_rock();
        assert!(node.is_on() == false);
        node.turn_to_lamp();
        assert!(node.is_on() == true);
        node.turn_to_lamp();
        assert!(node.is_on() == true);
    }
}
