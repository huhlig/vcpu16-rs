//! VCPU16 System
pub mod bus;
pub mod cpu;
pub mod mem;
pub mod hardware;

type Word = u16;

/// Shared System
pub struct System {
    mem: Memory,
    cpu: VCPU16,
    clk: Clock,
    bus: Bus,
}

impl System {
    pub fn new() {
        System {
            mem: Memory::new(),
            cpu: VCPU16: new(),
            clk: Clock::new(),
            bus: Bus::new(),
        }
    }
    pub fn mem(&mut self) -> &mut Memory { &self.mem }
    pub fn cpu(&mut self) -> &mut VCPU16 { &self.cpu }
    pub fn clk(&mut self) -> &mut Clock { &self.clk }
    pub fn bus(&mut self) -> &mut Bus { &self.bus }
}
