use ::cpu::Register;

/// OpCode Details
pub struct OpCode {
    pub name: &'static str,
    pub mask: u16,
    pub code: u16,
    pub time: u8,
    pub args: u8,
}

/// ValueCode Details
pub struct ValueCode {
    pub name: &'static str,
    pub code: u16,
    pub time: u8,
    pub next: bool,
}

//--------------------------------------------------------------------------------------------------
// Nullary Instructions: (word & 0xFFFF) == VALUE << 10
//--------------------------------------------------------------------------------------------------

/// Mask: 0xFFFF, Value: 0x00, Time: 1, Name: NOP, Type: Nullary
/// Description: No Operation
pub const NOP: OpCode = OpCode { name: "NOP", mask: 0xFFFF, code: 0x00 << 10, time: 1, args: 0 };

/// Mask: 0xFFFF, Value: 0x01, Time: 1, Name: CLK, Type: Nullary
/// Description: Sets I, J, PS from a 32 bit unsigned monotonically increasing cycle clock
/// I is set to ((0xFFFF0000 & T) >> 16)
/// J is set to ((0x0000FFFF & T) >> 0)
/// PS is set to 0x0001 if clock overflows, 0x0000 otherwise
pub const CLK: OpCode = OpCode { name: "CLK", mask: 0xFFFF, code: 0x01 << 10, time: 1, args: 0 };

/// Mask: 0xFFFF, Value: 0x1F, Time: 1, Name: ERR, Type: Nullary
/// Description: Decoding Error
pub const ERR: OpCode = OpCode { name: "ERR", mask: 0xFFFF, code: 0x3F << 10, time: 1, args: 0 };

//--------------------------------------------------------------------------------------------------
// Unary Instructions: (word & 0x03FF) == VALUE << 5
// Upper Value:        (word & 0xFC00) == VALUE << 10
//--------------------------------------------------------------------------------------------------
/// Mask: 0x03FF, Value: 0x01, Time: 3, Name: JSR, Type: Unary
/// Description: Pushes the address of the next instruction to the stack, then sets PC to u
pub const JSR: OpCode = OpCode { name: "JSR", mask: 0x03FF, code: 0x01 << 5, time: 3, args: 1 };

/// Mask: 0x03FF, Value: 0x02, Time: 1, Name: NOT, Type: Unary
/// Description: Sets u to ~u
pub const NOT: OpCode = OpCode { name: "NOT", mask: 0x03FF, code: 0x02 << 5, time: 3, args: 1 };

/// Mask: 0x03FF, Value: 0x08, Time: 4, Name: INT, Type: Unary
/// Description: Triggers a software interrupt with message u
pub const INT: OpCode = OpCode { name: "INT", mask: 0x03FF, code: 0x08 << 5, time: 4, args: 1 };

/// Mask: 0x03FF, Value: 0x09, Time: 1, Name: IAG, Type: Unary
/// Description: Sets u to IA
pub const IAG: OpCode = OpCode { name: "IAG", mask: 0x03FF, code: 0x09 << 5, time: 1, args: 1 };

/// Mask: 0x03FF, Value: 0x0A, Time: 1, Name: IAS, Type: Unary
/// Description: Sets IA to u
pub const IAS: OpCode = OpCode { name: "IAS", mask: 0x03FF, code: 0x0A << 5, time: 1, args: 1 };

/// Mask: 0x03FF, Value: 0x0B, Time: 3, Name: RFI, Type: Unary
/// Description: disables interrupt queueing, pops A from the stack, then pops PC from the stack
pub const RFI: OpCode = OpCode { name: "RFI", mask: 0x03FF, code: 0x0B << 5, time: 3, args: 1 };

/// Mask: 0x03FF, Value: 0x0C, Time: 2, Name: IAQ, Type: Unary
/// Description: if u is nonzero, interrupts will be added to the queue instead of triggered. if u
/// is zero, interrupts will be triggered as normal again
pub const IAQ: OpCode = OpCode { name: "RFI", mask: 0x03FF, code: 0x0C << 5, time: 2, args: 1 };

/// Mask: 0x03FF, Value: 0x10, Time: 2, Name: HWN, Type: Unary
/// Description: Sets u to number of connected hardware devices
pub const HWN: OpCode = OpCode { name: "HWN", mask: 0x03FF, code: 0x10 << 5, time: 2, args: 1 };

/// Mask: 0xFFFF, Value: 0x10, Time: 4, Name: HWQ, Type: Unary
/// Description: Sets X, Y, Z registers to information about hardware at port u
///  * X is a 16 bit word identifying the manufacturer id
///  * Y is a 16 bit word identifying the hardware id
///  * Z is a 16 bit word identifying the hardware version
pub const HWQ: OpCode = OpCode { name: "HWQ", mask: 0x03FF, code: 0x11 << 5, time: 4, args: 1 };

/// Mask: 0x03FF, Value: 0x12, Time: 4, Name: HWI, Type: Unary
/// Description: Sends an interrupt to hardware at port u
pub const HWI: OpCode = OpCode { name: "HWN", mask: 0x03FF, code: 0x12 << 5, time: 4, args: 1 };

//--------------------------------------------------------------------------------------------------
// Binary Instructions: (word & 0x001F) == VALUE << 0
// Upper Value:         (word & 0xFC00) == VALUE << 10
// Middle Value:        (word & 0x03E0) == VALUE << 5
//--------------------------------------------------------------------------------------------------
/// Mask: 0x001F, Value: 0x01, Time: 1, Name: SET, Type: Binary
/// Description: Sets m to u
pub const SET: OpCode = OpCode { name: "SET", mask: 0x001F, code: 0x01 << 0, time: 1, args: 2 };

/// Mask: 0x001F, Value: 0x02, Time: 2, Name: ADD, Type: Binary
/// Description: Sets m to m + u. Sets PS to 0x0001 if there's an overflow, 0x0000 otherwise
pub const ADD: OpCode = OpCode { name: "ADD", mask: 0x001F, code: 0x02 << 0, time: 2, args: 2 };

/// Mask: 0x001F, Value: 0x03, Time: 2, Name: SUB, Type: Binary
/// Description: Sets m to m - u. Sets PS to 0xFFFF if there's an underflow, 0x0000 otherwise
pub const SUB: OpCode = OpCode { name: "SUB", mask: 0x001F, code: 0x03 << 0, time: 2, args: 2 };

/// Mask: 0x001F, Value: 0x04, Time: 2, Name: MUL, Type: Binary
/// Description: Sets m to (m * u), sets PS to ((m*u)>>16) & 0xFFFF) (treats m & u as unsigned)
pub const MUL: OpCode = OpCode { name: "MUL", mask: 0x001F, code: 0x04 << 0, time: 2, args: 2 };

/// Mask: 0x001F, Value: 0x05, Time: 2, Name: MUL, Type: Binary
/// Description: Sets m to (m * u), sets PS to ((m*u)>>16) & 0xFFFF) (treats m & u as signed)
pub const MLI: OpCode = OpCode { name: "MLI", mask: 0x001F, code: 0x05 << 0, time: 2, args: 2 };

/// Mask: 0x001F, Value: 0x06, Time: 3, Name: DIV, Type: Binary
/// Description: Sets m to m / u, Sets PS to ((m<<16)/u) & 0xFFFF. if u==0, sets m and PS to 0
/// instead. (treats m & u as unsigned)
pub const DIV: OpCode = OpCode { name: "DIV", mask: 0x001F, code: 0x06 << 0, time: 3, args: 2 };

/// Mask: 0x001F, Value: 0x07, Time: 3, Name: DVI, Type: Binary
/// Description: Sets m to m / u, Sets PS to ((m<<16)/u) & 0xFFFF. if u==0, sets m and PS to 0
/// instead. Rounds towards 0. (treats m & u as signed)
pub const DVI: OpCode = OpCode { name: "DIV", mask: 0x001F, code: 0x07 << 0, time: 3, args: 2 };

/// Mask: 0x001F, Value: 0x08, Time: 3, Name: MOD, Type: Binary
/// Description: Sets m to m % u. If u == 0, Sets m to 0 instead. (treats m & u as unsigned)
pub const MOD: OpCode = OpCode { name: "MOD", mask: 0x001F, code: 0x08 << 0, time: 3, args: 2 };

/// Mask: 0x001F, Value: 0x09, Time: 3, Name: MDI, Type: Binary
/// Description: Sets m to m % u. If u == 0, Sets m to 0 instead. (treats m & u as signed)
pub const MDI: OpCode = OpCode { name: "MOD", mask: 0x001F, code: 0x09 << 0, time: 3, args: 2 };

/// Mask: 0x001F, Value: 0x0A, Time: 1, Name: AND, Type: Binary
/// Description: Sets m to m & u
pub const AND: OpCode = OpCode { name: "AND", mask: 0x001F, code: 0x0A << 0, time: 1, args: 2 };

/// Mask: 0x001F, Value: 0x0B, Time: 1, Name: BOR, Type: Binary
/// Description: Sets m to m | u
pub const BOR: OpCode = OpCode { name: "BOR", mask: 0x001F, code: 0x0B << 0, time: 1, args: 2 };

/// Mask: 0x001F, Value: 0x0C, Time: 1, Name: XOR, Type: Binary
/// Description: Sets m to m ^ u
pub const XOR: OpCode = OpCode { name: "XOR", mask: 0x001F, code: 0x0C << 0, time: 1, args: 2 };

/// Mask: 0x001F, Value: 0x0D, Time: 1, Name: LLS, Type: Binary
/// Description: Sets m to m << u, Sets PS to ((m<<u) >> 16) & 0xFFFF (logical shift)
pub const LLS: OpCode = OpCode { name: "LLS", mask: 0x001F, code: 0x0D << 0, time: 1, args: 2 };

/// Mask: 0x001F, Value: 0x0E, Time: 1, Name: LRS, Type: Binary
/// Description: Sets m to m >>> u, Sets PS to ((m<<16)>>u) & 0xFFFF. (logical shift)
pub const LRS: OpCode = OpCode { name: "LRS", mask: 0x001F, code: 0x0E << 0, time: 1, args: 2 };

/// Mask: 0x001F, Value: 0x0F, Time: 1, Name: ARS, Type: Binary
/// Description:  Sets m to m>>u, sets PS to ((m<<16)>>>u)&0xFFFF (arithmetic shift) (treats m as signed)
pub const ARS: OpCode = OpCode { name: "ARS", mask: 0x001F, code: 0x0F << 0, time: 1, args: 2 };

/// Mask: 0x001F, Value: 0x10, Time: 2, Name: IFB, Type: Binary
/// Description: Performs next instruction only if (m & u) != 0
pub const IFB: OpCode = OpCode { name: "IFB", mask: 0x001F, code: 0x10 << 0, time: 2, args: 2 };

/// Mask: 0x001F, Value: 0x11, Time: 2, Name: IFC, Type: Binary
/// Description: Performs next instruction only if (m & u) == 0
pub const IFC: OpCode = OpCode { name: "IFB", mask: 0x001F, code: 0x11 << 0, time: 2, args: 2 };

/// Mask: 0x001F, Value: 0x12, Time: 2, Name: IFE, Type: Binary
/// Description: Performs next instruction only if m == u
pub const IFE: OpCode = OpCode { name: "IFE", mask: 0x001F, code: 0x12 << 0, time: 2, args: 2 };

/// Mask: 0x001F, Value: 0x13, Time: 2, Name: IFN, Type: Binary
/// Description: Performs next instruction only if m != u
pub const IFN: OpCode = OpCode { name: "IFN", mask: 0x001F, code: 0x13 << 0, time: 2, args: 2 };

/// Mask: 0x001F, Value: 0x14, Time: 2, Name: IFG, Type: Binary
/// Description: Performs next instruction only if m > u (unsigned)
pub const IFG: OpCode = OpCode { name: "IFG", mask: 0x001F, code: 0x14 << 0, time: 2, args: 2 };

/// Mask: 0x001F, Value: 0x15, Time: 2, Name: IFA, Type: Binary
/// Description: Performs next instruction only if m > u (signed)
pub const IFA: OpCode = OpCode { name: "IFA", mask: 0x001F, code: 0x15 << 0, time: 2, args: 2 };

/// Mask: 0x001F, Value: 0x16, Time: 2, Name: IFL, Type: Binary
/// Description: Performs next instruction only if m < u (unsigned)
pub const IFL: OpCode = OpCode { name: "IFL", mask: 0x001F, code: 0x16 << 0, time: 2, args: 2 };

/// Mask: 0x001F, Value: 0x17, Time: 2, Name: IFU, Type: Binary
/// Description: Performs next instruction only if m < u (signed)
pub const IFU: OpCode = OpCode { name: "IFU", mask: 0x001F, code: 0x17 << 0, time: 2, args: 2 };

/// Mask: 0x001F, Value: 0x1A, Time: 3, Name: ADX, Type: Binary
/// Description: Sets m to m + u + PS, sets PS to 0x0001 if there is an overflow, 0x0000 otherwise
pub const ADX: OpCode = OpCode { name: "ADX", mask: 0x001F, code: 0x1A << 0, time: 3, args: 2 };

/// Mask: 0x001F, Value: 0x1B, Time: 3, Name: SBX, Type: Binary
/// Description: Sets m to m - u + PS, sets PS to 0xFFFF if there is an overflow, 0x0000 otherwise
pub const SBX: OpCode = OpCode { name: "SBX", mask: 0x001F, code: 0x1B << 0, time: 3, args: 2 };

/// Mask: 0x001F, Value: 0x1E, Time: 3, Name: STI, Type: Binary
/// Description: Sets m to u, then increases I and J by 1
pub const STI: OpCode = OpCode { name: "STI", mask: 0x001F, code: 0x1E << 0, time: 2, args: 2 };

/// Mask: 0x001F, Value: 0x1F, Time: 3, Name: STD, Type: Binary
/// Description: Sets m to u, then decreases I and J by 1
pub const STD: OpCode = OpCode { name: "STD", mask: 0x001F, code: 0x1F << 0, time: 2, args: 2 };

/// Value: 0x00, Time: 0, Name: A
/// Description: Value of Register A
pub const REGA: ValueCode = ValueCode { name: "A", code: 0x00, time: 0, next: false };

/// Value: 0x01, Time: 0, Name: B
/// Description: Value of Register B
pub const REGB: ValueCode = ValueCode { name: "B", code: 0x01, time: 0, next: false };

/// Value: 0x02, Time: 0, Name: C
/// Description: Value of Register C
pub const REGC: ValueCode = ValueCode { name: "C", code: 0x02, time: 0, next: false };

/// Value: 0x03, Time: 0, Name: X
/// Description: Value of Register X
pub const REGX: ValueCode = ValueCode { name: "X", code: 0x03, time: 0, next: false };

/// Value: 0x04, Time: 0, Name: Y
/// Description: Value of Register Y
pub const REGY: ValueCode = ValueCode { name: "Y", code: 0x04, time: 0, next: false };

/// Value: 0x05, Time: 0, Name: Z
/// Description: Value of Register Z
pub const REGZ: ValueCode = ValueCode { name: "Z", code: 0x05, time: 0, next: false };

/// Value: 0x06, Time: 0, Name: I
/// Description: Value of Register I
pub const REGI: ValueCode = ValueCode { name: "I", code: 0x06, time: 0, next: false };

/// Value: 0x07, Time: 0, Name: J
/// Description: Value of Register J
pub const REGJ: ValueCode = ValueCode { name: "J", code: 0x07, time: 0, next: false };

/// Value: 0x08, Time: 0, Name: [A]
/// Description: Value of Memory at Address from Register A
pub const MEMA: ValueCode = ValueCode { name: "[A]", code: 0x08, time: 0, next: false };

/// Value: 0x09, Time: 0, Name: [B]
/// Description: Value of Memory at Address from Register B
pub const MEMB: ValueCode = ValueCode { name: "[B]", code: 0x09, time: 0, next: false };

/// Value: 0x0A, Time: 0, Name: [C]
/// Description: Value of Memory at Address from Register C
pub const MEMC: ValueCode = ValueCode { name: "[C]", code: 0x0A, time: 0, next: false };

/// Value: 0x0B, Time: 0, Name: [X]
/// Description: Value of Memory at Address from Register X
pub const MEMX: ValueCode = ValueCode { name: "[X]", code: 0x0B, time: 0, next: false };

/// Value: 0x0C, Time: 0, Name: [Y]
/// Description: Value of Memory at Address from Register Y
pub const MEMY: ValueCode = ValueCode { name: "[A]", code: 0x0C, time: 0, next: false };

/// Value: 0x0D, Time: 0, Name: [Z]
/// Description: Value of Memory at Address from Register A
pub const MEMZ: ValueCode = ValueCode { name: "[A]", code: 0x0D, time: 0, next: false };

/// Value: 0x0E, Time: 0, Name: [I]
/// Description: Value of Memory at Address from Register I
pub const MEMI: ValueCode = ValueCode { name: "[I]", code: 0x0E, time: 0, next: false };

/// Value: 0x0F, Time: 0, Name: [J]
/// Description: Value of Memory at Address from Register J
pub const MEMJ: ValueCode = ValueCode { name: "[J]", code: 0x0F, time: 0, next: false };

/// Value: 0x10, Time: 1, Name: [A + NEXT]
/// Description: Value of Memory at Address from Register A plus Memory at Address from PC
pub const NEXTA: ValueCode = ValueCode { name: "[A + NEXT]", code: 0x10, time: 1, next: true };

/// Value: 0x11, Time: 1, Name: [B + NEXT]
/// Description: Value of Memory at Address from Register B plus Memory at Address from PC
pub const NEXTB: ValueCode = ValueCode { name: "[B + NEXT]", code: 0x11, time: 1, next: true };

/// Value: 0x12, Time: 1, Name: [C + NEXT]
/// Description: Value of Memory at Address from Register C plus Memory at Address from PC
pub const NEXTC: ValueCode = ValueCode { name: "[C + NEXT]", code: 0x12, time: 1, next: true };

/// Value: 0x13, Time: 1, Name: [X + NEXT]
/// Description: Value of Memory at Address from Register Z plus Memory at Address from PC
pub const NEXTX: ValueCode = ValueCode { name: "[X + NEXT]", code: 0x13, time: 1, next: true };

/// Value: 0x14, Time: 1, Name: [Y + NEXT]
/// Description: Value of Memory at Address from Register Y plus Memory at Address from PC
pub const NEXTY: ValueCode = ValueCode { name: "[Y + NEXT]", code: 0x14, time: 1, next: true };

/// Value: 0x15, Time: 1, Name: [Z + NEXT]
/// Description: Value of Memory at Address from Register Z plus Memory at Address from PC
pub const NEXTZ: ValueCode = ValueCode { name: "[Z + NEXT]", code: 0x15, time: 1, next: true };

/// Value: 0x16, Time: 1, Name: [I + NEXT]
/// Description: Value of Memory at Address from Register I plus Memory at Address from PC
pub const NEXTI: ValueCode = ValueCode { name: "[I + NEXT]", code: 0x16, time: 1, next: true };

/// Value: 0x17, Time: 1, Name: [J + NEXT]
/// Description: Value of Memory at Address from Register J plus Memory at Address from PC
pub const NEXTJ: ValueCode = ValueCode { name: "[J + NEXT]", code: 0x17, time: 1, next: true };

/// Value: 0x18, Time: 0, Name: [SP++]
/// Description: Pop Stack Value
pub const SPOP: ValueCode = ValueCode { name: "[SP++]", code: 0x18, time: 0, next: false };

/// Value: 0x18, Time: 0, Name: [--SP]
/// Description: Push Stack Value
pub const SPUSH: ValueCode = ValueCode { name: "[--SP]", code: 0x18, time: 0, next: false };

/// Value: 0x19, Time: 0, Name: [SP++]
/// Description: Peek Stack Value
pub const SPEEK: ValueCode = ValueCode { name: "[SP]", code: 0x19, time: 0, next: false };

/// Value: 0x1A, Time: 1, Name: [SP + NEXT]
/// Description: Pick Stack Value
pub const SPICK: ValueCode = ValueCode { name: "[SP + NEXT]", code: 0x1A, time: 1, next: true };

/// Value: 0x1B, Time: 0, Name: SP
/// Description: Value of Stack Pointer
pub const SP: ValueCode = ValueCode { name: "SP", code: 0x1B, time: 0, next: false };

/// Value: 0x1C, Time: 0, Name: PC
/// Description: Value of Program Counter
pub const PC: ValueCode = ValueCode { name: "PC", code: 0x1C, time: 0, next: false };

/// Value: 0x1D, Time: 0, Name: PS
/// Description: Value of Program Status
pub const PS: ValueCode = ValueCode { name: "PS", code: 0x1D, time: 0, next: false };

/// Value: 0x1E, Time: 1, Name: [NEXT]
/// Description: Memory at Next Word
pub const MNEXT: ValueCode = ValueCode { name: "[NEXT]", code: 0x1E, time: 1, next: true };

/// Value: 0x1F, Time: 1, Name: NEXT
/// Description: Literal of Next Word
pub const LNEXT: ValueCode = ValueCode { name: "NEXT", code: 0x1F, time: 1, next: true };
