#[derive(Copy, Clone, Debug)]
pub struct Frame {
    number: u32,
}

impl Frame {
    pub fn next(&self) -> Self {
        Self {
            number: self.number + 1
        }
    }

    pub fn number(&self) -> u32 {
        self.number
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            number: 0
        }
    }
}
