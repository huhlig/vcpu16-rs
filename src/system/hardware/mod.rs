//! Hardware For the VCPU16

/// Hardware Interface
pub trait Hardware {
    /// Connect Hardware to the VCPU16
    fn connect(id: u16, system: System);
    /// Send an Interrupt to Hardware
    fn interrupt(message: u16);
    /// Time step
    fn step();
    /// Disconnect hardware from the VCPU16
    fn disconnect();
}