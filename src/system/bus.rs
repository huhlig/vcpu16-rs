use super::Word;
use super::hardware::Hardware;

pub struct Bus {
    hardware: HashMap<Word, Hardware>
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            hardware: HashMap<Word, Hardware>,
        }
    }
}