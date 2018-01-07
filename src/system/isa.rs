use ::cpu::Register;

/// Register
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

/// Instruction Argument
enum Argument {
    /// Next Word [PC++]
    NextWord,
    /// Register
    Register(Register),
    /// Address in memory
    Memory(Argument),
    /// Register + NextWord
    Offset(Register),
    /// Push Stack
    PushStack,
    /// Push Stack
    PopStack,
    /// Literal Value
    Literal(u16),
}

/// Reference Operation
pub struct OpCode {
    name: &'static str,
    code: u16,
    time: u8,
}

pub enum Instruction {
    Label {
        address: u16,
        name: String,
    },
    Nullary {
        address: u16,
        time: u8,
        size: u8,
        op: &'static OpCode,
    },
    Unary {
        addr: u16,
        time: u8,
        size: u8,
        arg0: Argument,
        op: &'static OpCode,
    },
    Binary {
        addr: u16,
        time: u8,
        size: u8,
        arg0: Argument,
        arg1: Argument,
        op: &'static OpCode,
    },
}

//--------------------------------------------------------------------------------------------------
// Nullary Instructions
//--------------------------------------------------------------------------------------------------

/// Mask: 0xFFFF, Value: 0x00, Time: 1, Name: NOP, Type: Nullary
/// Description: No Operation
pub const NOP: OpCode = OpCode { name: "NOP", code: 0x00, time: 1 };

/// Mask: 0xFFFF, Value: 0x01, Time: 1, Name: CLK, Type: Nullary
/// Description: Sets I, J, PS from a 32 bit unsigned monotonically increasing cycle clock
/// I is set to ((0xFFFF0000 & T) >> 16)
/// J is set to ((0x0000FFFF & T) >> 0)
/// PS is set to 0x0001 if clock overflows, 0x0000 otherwise
pub const CLK: OpCode = OpCode { name: "CLK", code: 0x01, time: 1 };

/// Mask: 0xFFFF, Value: 0x1F, Time: 1, Name: ERR, Type: Nullary
/// Description: Decoding Error
pub const ERR: OpCode = OpCode { name: "ERR", code: 0x3F, time: 1 };

//--------------------------------------------------------------------------------------------------
// Unary Instructions
//--------------------------------------------------------------------------------------------------
/// Mask: 0x03FF, Value: 0x01, Time: 3, Name: JSR, Type: Unary
/// Description: Pushes the address of the next instruction to the stack, then sets PC to u
pub const JSR: OpCode = OpCode { name: "JSR", code: 0x01, time: 3 };

/// Mask: 0x03FF, Value: 0x02, Time: 1, Name: NOT, Type: Unary
/// Description: Sets u to ~u
pub const NOT: OpCode = OpCode { name: "NOT", code: 0x02, time: 3 };

/// Mask: 0x03FF, Value: 0x08, Time: 4, Name: INT, Type: Unary
/// Description: Triggers a software interrupt with message u
pub const INT: OpCode = OpCode { name: "INT", code: 0x08, time: 4 };

/// Mask: 0x03FF, Value: 0x09, Time: 1, Name: IAG, Type: Unary
/// Description: Sets u to IA
pub const IAG: OpCode = OpCode { name: "IAG", code: 0x09, time: 1 };

/// Mask: 0x03FF, Value: 0x0A, Time: 1, Name: IAS, Type: Unary
/// Description: Sets IA to u
pub const IAS: OpCode = OpCode { name: "IAS", code: 0x0A, time: 1 };

/// Mask: 0x03FF, Value: 0x0B, Time: 3, Name: RFI, Type: Unary
/// Description: disables interrupt queueing, pops A from the stack, then pops PC from the stack
pub const RFI: OpCode = OpCode { name: "RFI", code: 0x0B, time: 3 };

/// Mask: 0x03FF, Value: 0x0C, Time: 2, Name: IAQ, Type: Unary
/// Description: if u is nonzero, interrupts will be added to the queue instead of triggered. if u
/// is zero, interrupts will be triggered as normal again
pub const IAQ: OpCode = OpCode { name: "RFI", code: 0x0C, time: 2 };

/// Mask: 0x03FF, Value: 0x10, Time: 2, Name: HWN, Type: Unary
/// Description: Sets u to number of connected hardware devices
pub const HWN: OpCode = OpCode { name: "HWN", code: 0x10, time: 2 };

/// Mask: 0xFFFF, Value: 0x10, Time: 4, Name: HWQ, Type: Unary
/// Description: Sets X, Y, Z registers to information about hardware at port u
///  * X is a 16 bit word identifying the manufacturer id
///  * Y is a 16 bit word identifying the hardware id
///  * Z is a 16 bit word identifying the hardware version
pub const HWQ: OpCode = OpCode { name: "HWQ", code: 0x11, time: 4 };

/// Mask: 0x03FF, Value: 0x12, Time: 4, Name: HWI, Type: Unary
/// Description: Sends an interrupt to hardware at port u
pub const HWI: OpCode = OpCode { name: "HWN", code: 0x12, time: 4 };

//--------------------------------------------------------------------------------------------------
// Binary Instructions
//--------------------------------------------------------------------------------------------------
/// Mask: 0x001F, Value: 0x01, Time: 1, Name: SET, Type: Binary
/// Description: Sets m to u
pub const SET: OpCode = OpCode { name: "SET", code: 0x01, time: 1 };

/// Mask: 0x001F, Value: 0x02, Time: 2, Name: ADD, Type: Binary
/// Description: Sets m to m + u. Sets PS to 0x0001 if there's an overflow, 0x0000 otherwise
pub const ADD: OpCode = OpCode { name: "ADD", code: 0x02, time: 2 };

/// Mask: 0x001F, Value: 0x03, Time: 2, Name: SUB, Type: Binary
/// Description: Sets m to m - u. Sets PS to 0xFFFF if there's an underflow, 0x0000 otherwise
pub const SUB: OpCode = OpCode { name: "SUB", code: 0x03, time: 2 };

/// Mask: 0x001F, Value: 0x04, Time: 2, Name: MUL, Type: Binary
/// Description: Sets m to (m * u), sets PS to ((m*u)>>16) & 0xFFFF) (treats m & u as unsigned)
pub const MUL: OpCode = OpCode { name: "MUL", code: 0x04, time: 2 };

/// Mask: 0x001F, Value: 0x05, Time: 2, Name: MUL, Type: Binary
/// Description: Sets m to (m * u), sets PS to ((m*u)>>16) & 0xFFFF) (treats m & u as signed)
pub const MLI: OpCode = OpCode { name: "MLI", code: 0x05, time: 2 };

/// Mask: 0x001F, Value: 0x06, Time: 3, Name: DIV, Type: Binary
/// Description: Sets m to m / u, Sets PS to ((m<<16)/u) & 0xFFFF. if u==0, sets m and PS to 0
/// instead. (treats m & u as unsigned)
pub const DIV: OpCode = OpCode { name: "DIV", code: 0x06, time: 3 };

/// Mask: 0x001F, Value: 0x07, Time: 3, Name: DVI, Type: Binary
/// Description: Sets m to m / u, Sets PS to ((m<<16)/u) & 0xFFFF. if u==0, sets m and PS to 0
/// instead. Rounds towards 0. (treats m & u as signed)
pub const DVI: OpCode = OpCode { name: "DIV", code: 0x07, time: 3 };

/// Mask: 0x001F, Value: 0x08, Time: 3, Name: MOD, Type: Binary
/// Description: Sets m to m % u. If u == 0, Sets m to 0 instead. (treats m & u as unsigned)
pub const MOD: OpCode = OpCode { name: "MOD", code: 0x08, time: 3 };

/// Mask: 0x001F, Value: 0x09, Time: 3, Name: MDI, Type: Binary
/// Description: Sets m to m % u. If u == 0, Sets m to 0 instead. (treats m & u as signed)
pub const MDI: OpCode = OpCode { name: "MOD", code: 0x09, time: 3 };

/// Mask: 0x001F, Value: 0x0A, Time: 1, Name: AND, Type: Binary
/// Description: Sets m to m & u
pub const AND: OpCode = OpCode { name: "AND", code: 0x0A, time: 1 };

/// Mask: 0x001F, Value: 0x0B, Time: 1, Name: BOR, Type: Binary
/// Description: Sets m to m | u
pub const BOR: OpCode = OpCode { name: "BOR", code: 0x0B, time: 1 };

/// Mask: 0x001F, Value: 0x0C, Time: 1, Name: XOR, Type: Binary
/// Description: Sets m to m ^ u
pub const XOR: OpCode = OpCode { name: "XOR", code: 0x0C, time: 1 };

/// Mask: 0x001F, Value: 0x0D, Time: 1, Name: LLS, Type: Binary
/// Description: Sets m to m << u, Sets PS to ((m<<u) >> 16) & 0xFFFF (logical shift)
pub const LLS: OpCode = OpCode { name: "LLS", code: 0x0D, time: 1 };

/// Mask: 0x001F, Value: 0x0E, Time: 1, Name: LRS, Type: Binary
/// Description: Sets m to m >>> u, Sets PS to ((m<<16)>>u) & 0xFFFF. (logical shift)
pub const LRS: OpCode = OpCode { name: "LRS", code: 0x0E, time: 1 };

/// Mask: 0x001F, Value: 0x0F, Time: 1, Name: ARS, Type: Binary
/// Description:  Sets m to m>>u, sets PS to ((m<<16)>>>u)&0xFFFF (arithmetic shift) (treats m as signed)
pub const ARS: OpCode = OpCode { name: "ARS", code: 0x0F, time: 1 };

/// Mask: 0x001F, Value: 0x10, Time: 2, Name: IFB, Type: Binary
/// Description: Performs next instruction only if (m & u) != 0
pub const IFB: OpCode = OpCode { name: "IFB", code: 0x10, time: 2 };

/// Mask: 0x001F, Value: 0x11, Time: 2, Name: IFC, Type: Binary
/// Description: Performs next instruction only if (m & u) == 0
pub const IFC: OpCode = OpCode { name: "IFB", code: 0x11, time: 2 };

/// Mask: 0x001F, Value: 0x12, Time: 2, Name: IFE, Type: Binary
/// Description: Performs next instruction only if m == u
pub const IFE: OpCode = OpCode { name: "IFE", code: 0x12, time: 2 };

/// Mask: 0x001F, Value: 0x13, Time: 2, Name: IFN, Type: Binary
/// Description: Performs next instruction only if m != u
pub const IFN: OpCode = OpCode { name: "IFN", code: 0x13, time: 2 };

/// Mask: 0x001F, Value: 0x14, Time: 2, Name: IFG, Type: Binary
/// Description: Performs next instruction only if m > u (unsigned)
pub const IFG: OpCode = OpCode { name: "IFG", code: 0x14, time: 2 };

/// Mask: 0x001F, Value: 0x15, Time: 2, Name: IFA, Type: Binary
/// Description: Performs next instruction only if m > u (signed)
pub const IFA: OpCode = OpCode { name: "IFA", code: 0x15, time: 2 };

/// Mask: 0x001F, Value: 0x16, Time: 2, Name: IFL, Type: Binary
/// Description: Performs next instruction only if m < u (unsigned)
pub const IFL: OpCode = OpCode { name: "IFL", code: 0x16, time: 2 };

/// Mask: 0x001F, Value: 0x17, Time: 2, Name: IFU, Type: Binary
/// Description: Performs next instruction only if m < u (signed)
pub const IFU: OpCode = OpCode { name: "IFU", code: 0x17, time: 2 };

/// Mask: 0x001F, Value: 0x1A, Time: 3, Name: ADX, Type: Binary
/// Description: Sets m to m + u + PS, sets PS to 0x0001 if there is an overflow, 0x0000 otherwise
pub const ADX: OpCode = OpCode { name: "ADX", code: 0x1A, time: 3 };

/// Mask: 0x001F, Value: 0x1B, Time: 3, Name: SBX, Type: Binary
/// Description: Sets m to m - u + PS, sets PS to 0xFFFF if there is an overflow, 0x0000 otherwise
pub const SBX: OpCode = OpCode { name: "SBX", code: 0x1B, time: 3 };

/// Mask: 0x001F, Value: 0x1E, Time: 3, Name: STI, Type: Binary
/// Description: Sets m to u, then increases I and J by 1
pub const STI: OpCode = OpCode { name: "STI", code: 0x1E, time: 2 };

/// Mask: 0x001F, Value: 0x1F, Time: 3, Name: STD, Type: Binary
/// Description: Sets m to u, then decreases I and J by 1
pub const STD: OpCode = OpCode { name: "STD", code: 0x1F, time: 2 };
