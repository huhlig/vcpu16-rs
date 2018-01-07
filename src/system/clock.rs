pub struct Clock {
    halted: bool,
    cycles: u64,
}

impl Clock {
    pub fn new() {
        Clock {
            halted: false,
            cycles: 0,
        }
    }
    pub fn halted(&self) -> bool { self.halted }
    pub fn cycles(&self) -> u64 { self.cycles }
    pub fn step(&mut self) { self.cycles += 1 }
}