/// OpCode Details
pub struct OpCode {
    pub name: &'static str,
    pub mask: u16,
    pub code: u16,
    pub time: u8,
    pub args: u8,
}

pub enum Operation {
    Nullary {
        name: &'static str,
        code: u16,
        time: u8
    },
    Unary {
        name: &'static str,
        code: u16,
        time: u8,
        arg0: Argument,
    },
    Binary {
        name: &'static str,
        code: u16,
        time: u8,
        arg0: Argument,
        arg1: Argument,
    }
}

enum Register {
    PC,
    SP,
    PS,
    A,
    B,
    C,
    X,
    Y,
    Z,
    I,
    J,
}

enum Argument {
    /// Next Word
    NextWord,
    /// Register
    Register(Register),
    /// Address in memory
    Memory(Argument),
    ///
    Offset(Register)
    /// Push Stack
    Push,
    /// Push Stack
    Pop,
    /// Peek Stack
    Peek,




}

pub struct Instruction {
    address: u16,
    opcode: OpCode,
    upper: Option<Argument>,
    lower: Option<Argument>,
}