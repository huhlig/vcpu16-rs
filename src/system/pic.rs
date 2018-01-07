use super::system::Word;

/// Programmable Interrupt Controller
pub struct PIC {
    /// Map of Queued Hardware
    interrupts: HashMap<u16, IRQ>,
    /// Is Interrupt Queueing Enabled
    enabled: bool,
    /// Current Queue Head
    head: u8,
    /// Current Queue Tail
    tail: u8,
}

pub struct IRQ {
    interrupts: [u16;256],
    enabled: bool,
    head: usize,
    tail: usize,
}

impl IRQ {
    pub fn new() -> IRQ {
        IRQ {
            interrupts: [[0; 256]; 65536
            enabled: false,
            head: 0,
            tail: 0,
        }
    }
    pub fn enqueue(&mut self, id: u16, message: Word) {
        if enabled {
            if
        }
    }
}