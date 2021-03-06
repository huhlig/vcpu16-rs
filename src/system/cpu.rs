use std::io::{Read, Write};
use std::mem;
use std::slice;

/// VCPU16 Context
pub struct VCPU16<'sys> {
    sys: &'sys System,
    irq: IRQ,
    pc: u16,
    sp: u16,
    ps: u16,
    a: u16,
    b: u16,
    c: u16,
    x: u16,
    y: u16,
    z: u16,
    i: u16,
    j: u16,
}

/// Memory Array
struct Memory {
    /// Memory Buffer
    buffer: [u16; 65536],
}

/// Interrupt Request Queue
struct IRQ {
    /// Queue of interrupts
    interrupts: Vec<u16>,
    /// Are interrupt's currently enabled
    enabled: bool,
    /// Interrupt Address
    address: u16,
}

/// Internal Clock
struct Clock {
    /// Is CPU Halted
    halted: bool,
    /// Cycles since Startup
    cycles: u64,
    /// Countdown timer
    timer: u16,
    /// Cycles remaining till instruction completes
    busy: u8,
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
enum Argument {
    Literal(u16),
    Memory(u16),
    Register(Register),
}

impl VCPU16 {
    /// Create a new VCPU16
    pub fn new() -> VCPU16 {
        VCPU16 {
            mem: Memory {
                buffer: [0; 65536],
            },
            clk: Clock {
                halted: false,
                cycles: 0,
                timer: 0,
                busy: 0,
            },
            irq: IRQ {
                interrupts: Vec::new(),
                enabled: bool,
                address: u16,
            },
            pc: 0,
            sp: 0,
            ps: 0,
            a: 0,
            b: 0,
            c: 0,
            x: 0,
            y: 0,
            z: 0,
            i: 0,
            j: 0,
        }
    }
    ///
    /// Load Memory from Reader
    ///
    pub fn load_mem(&mut self, reader: &mut Read) {
        unsafe {
            let memory_size = mem::size_of_val(&self.mem.buffer);
            let memory_slice = slice::from_raw_parts_mut(
                &mut self.mem.buffer as *mut _ as *mut u8,
                memory_size,
            );
            reader.read_exact(memory_slice).unwrap();
        }
    }
    ///
    /// Save memory to writer
    ///
    pub fn save_mem(&mut self, writer: &mut Write) {
        unsafe {
            let memory_size = mem::size_of_val(&self.mem.buffer);
            let memory_slice = slice::from_raw_parts_mut(
                &mut self.mem.buffer as *mut _ as *mut u8,
                memory_size,
            );
            writer.write(memory_slice).unwrap();
        }
    }
    ///
    /// Clear Memory
    ///
    pub fn clear_mem(&mut self) {
        self.mem.buffer = [0; 65536];
    }
    ///
    /// Write a slice of memory from buffer
    ///
    pub fn write_mem(&mut self, address: u16, buffer: &[u16]) {
        self.mem.buffer[address as usize..buffer.len()].copy_from_slice(buffer)
    }
    ///
    /// Read a slice length of memory at address
    ///
    pub fn read_mem(&mut self, address: u16, length: u16) -> &[u16] {
        &self.mem.buffer[address as usize..length as usize]
    }
    ///
    /// Set a single Cell of Memory at address
    ///
    pub fn set_mem(&mut self, address: u16, value: u16) {
        self.mem.buffer[address as usize] = value
    }
    ///
    /// Get a single Cell of Memory at address
    ///
    pub fn get_mem(&self, address: u16) -> u16 {
        self.mem.buffer[address as usize]
    }
    /// Get value of the Program Counter (PC) Register
    pub fn get_pc(&self) -> u16 { self.pc }
    /// Get value of the Stack Pointer (SP) Register
    pub fn get_sp(&self) -> u16 { self.sp }
    /// Get value of the Program Status (PS) Register
    pub fn get_ps(&self) -> u16 { self.ps }
    /// Get value of the Interrupt Address (IA) Register
    pub fn get_ia(&self) -> u16 { self.irq.address }
    /// Get value of Register A
    pub fn get_a(&self) -> u16 { self.a }
    /// Get value of Register B
    pub fn get_b(&self) -> u16 { self.b }
    /// Get value of Register C
    pub fn get_c(&self) -> u16 { self.c }
    /// Get value of Register X
    pub fn get_x(&self) -> u16 { self.x }
    /// Get value of Register Y
    pub fn get_y(&self) -> u16 { self.y }
    /// Get value of Register Z
    pub fn get_z(&self) -> u16 { self.z }
    /// Get value of Register I
    pub fn get_i(&self) -> u16 { self.i }
    /// Get value of Register J
    pub fn get_j(&self) -> u16 { self.j }
    /// Enqueue Interrupt
    pub fn interrupt(&mut self, message: u16) {
        if self.ia != 0 {
            self.irq.interrupts.push(message);
        }
    }
    /// Step through next clock cycle
    pub fn step(&mut self) {
        if self.clk.halted {
            // CPU Halted, No further actions taken
            return;
        }
        self.clk.cycles += 1;
        if self.clk.busy > 0 {
            // CPU is busy
            self.clk.busy -= 1;
        } else if self.irq.enabled && self.irq.interrupts.len() > 0 {
            let message = self.irq.interrupts.pop().unwrap();

            // Interrupt Queued

        } else {
            self.execute();
        }
    }
    /// Was CPU Interrupted
    fn interrupted(&mut self) -> bool {

    }
    /// Execute Next Instruction
    fn execute(&mut self) {
        let address = self.pc;
        let word = self.mem.buffer[address as usize] as u16;
        fn next_pc(cpu: &mut VCPU16) -> u16 {
            let pc = cpu.pc;
            cpu.clk.busy += 1;
            cpu.pc += 1;
            cpu.mem.buffer[pc as usize] as u16
        }
        fn push_sp(cpu: &mut VCPU16) -> u16 {
            cpu.sp -= 1;
            cpu.sp
        }
        fn pop_sp(cpu: &mut VCPU16) -> u16 {
            let sp = cpu.sp;
            cpu.sp += 1;
            sp
        }
        fn write_arg(cpu: &mut VCPU16, arg: Argument, value: u16) {
            match arg {
                Argument::Literal(_) => { /* Do nothing */ }
                Argument::Memory(address) => { cpu.mem.buffer[address as usize] = value }
                Argument::Register(reg) => {
                    match reg {
                        Register::A => { cpu.a = value }
                        Register::B => { cpu.b = value }
                        Register::C => { cpu.c = value }
                        Register::X => { cpu.x = value }
                        Register::Y => { cpu.y = value }
                        Register::Z => { cpu.z = value }
                        Register::I => { cpu.i = value }
                        Register::J => { cpu.j = value }
                        Register::PC => { cpu.pc = value }
                        Register::SP => { cpu.sp = value }
                        Register::PS => { cpu.ps = value }
                    }
                }
            }
        }
        fn read_arg(cpu: &mut VCPU16, arg: Argument) -> u16 {
            match arg {
                Argument::Literal(value) => { value }
                Argument::Memory(address) => { cpu.mem.buffer[address as usize] }
                Argument::Register(reg) => {
                    match reg {
                        Register::A => { cpu.a }
                        Register::B => { cpu.b }
                        Register::C => { cpu.c }
                        Register::X => { cpu.x }
                        Register::Y => { cpu.y }
                        Register::Z => { cpu.z }
                        Register::I => { cpu.i }
                        Register::J => { cpu.j }
                        Register::PC => { cpu.pc }
                        Register::SP => { cpu.sp }
                        Register::PS => { cpu.ps }
                    }
                }
            }
        }
        fn upper(cpu: &mut VCPU16, word: u16) -> Argument {
            match (word & 0xFC00) >> 10 {
                // register
                0x00 => { Argument::Register(Register::A) }
                0x01 => { Argument::Register(Register::B) }
                0x02 => { Argument::Register(Register::C) }
                0x03 => { Argument::Register(Register::X) }
                0x04 => { Argument::Register(Register::Y) }
                0x05 => { Argument::Register(Register::Z) }
                0x06 => { Argument::Register(Register::I) }
                0x07 => { Argument::Register(Register::J) }
                // [register]
                0x08 => { Argument::Memory(cpu.a) }
                0x09 => { Argument::Memory(cpu.b) }
                0x0A => { Argument::Memory(cpu.c) }
                0x0B => { Argument::Memory(cpu.x) }
                0x0C => { Argument::Memory(cpu.y) }
                0x0D => { Argument::Memory(cpu.z) }
                0x0E => { Argument::Memory(cpu.i) }
                0x0F => { Argument::Memory(cpu.j) }
                // [register + NEXT_PC]
                0x10 => { Argument::Memory(cpu.a + next_pc(cpu)) }
                0x11 => { Argument::Memory(cpu.b + next_pc(cpu)) }
                0x12 => { Argument::Memory(cpu.c + next_pc(cpu)) }
                0x13 => { Argument::Memory(cpu.x + next_pc(cpu)) }
                0x14 => { Argument::Memory(cpu.y + next_pc(cpu)) }
                0x15 => { Argument::Memory(cpu.z + next_pc(cpu)) }
                0x16 => { Argument::Memory(cpu.i + next_pc(cpu)) }
                0x17 => { Argument::Memory(cpu.j + next_pc(cpu)) }
                // Stack Operations
                0x18 => { Argument::Memory(pop_sp(cpu)) }
                0x19 => { Argument::Memory(cpu.sp) }
                0x1A => { Argument::Memory(cpu.sp + next_pc(cpu)) }
                // Specialty Registers
                0x1B => { Argument::Register(Register::SP) }
                0x1C => { Argument::Register(Register::PC) }
                0x1D => { Argument::Register(Register::PS) }
                // Value of Memory at Next Word
                0x1E => { Argument::Memory(next_pc(cpu)) }
                // Next Word as Literal
                0x1F => { Argument::Literal(next_pc(cpu)) }
                // Literal Values
                0x20 => { Argument::Literal(0xFFFF) }
                0x21 => { Argument::Literal(0x0000) }
                0x22 => { Argument::Literal(0x0001) }
                0x23 => { Argument::Literal(0x0002) }
                0x24 => { Argument::Literal(0x0003) }
                0x25 => { Argument::Literal(0x0004) }
                0x26 => { Argument::Literal(0x0005) }
                0x27 => { Argument::Literal(0x0006) }
                0x28 => { Argument::Literal(0x0007) }
                0x29 => { Argument::Literal(0x0008) }
                0x2A => { Argument::Literal(0x0009) }
                0x2B => { Argument::Literal(0x000A) }
                0x2C => { Argument::Literal(0x000B) }
                0x2D => { Argument::Literal(0x000C) }
                0x2E => { Argument::Literal(0x000D) }
                0x2F => { Argument::Literal(0x000E) }
                0x30 => { Argument::Literal(0x000F) }
                0x31 => { Argument::Literal(0x0010) }
                0x32 => { Argument::Literal(0x0011) }
                0x33 => { Argument::Literal(0x0012) }
                0x34 => { Argument::Literal(0x0013) }
                0x35 => { Argument::Literal(0x0014) }
                0x36 => { Argument::Literal(0x0015) }
                0x37 => { Argument::Literal(0x0016) }
                0x38 => { Argument::Literal(0x0017) }
                0x39 => { Argument::Literal(0x0018) }
                0x3A => { Argument::Literal(0x0019) }
                0x3B => { Argument::Literal(0x001A) }
                0x3C => { Argument::Literal(0x001B) }
                0x3D => { Argument::Literal(0x001C) }
                0x3E => { Argument::Literal(0x001D) }
                0x3F => { Argument::Literal(0x001E) }
                _ => { Argument::Literal(0x0000) }
            }
        };
        fn middle(cpu: &mut VCPU16, word: u16) -> Argument {
            match (word & 0x03E0) >> 5 {
                // register
                0x00 => { Argument::Register(Register::A) }
                0x01 => { Argument::Register(Register::B) }
                0x02 => { Argument::Register(Register::C) }
                0x03 => { Argument::Register(Register::X) }
                0x04 => { Argument::Register(Register::Y) }
                0x05 => { Argument::Register(Register::Z) }
                0x06 => { Argument::Register(Register::I) }
                0x07 => { Argument::Register(Register::J) }
                // [register]
                0x08 => { Argument::Memory(cpu.a) }
                0x09 => { Argument::Memory(cpu.b) }
                0x0A => { Argument::Memory(cpu.c) }
                0x0B => { Argument::Memory(cpu.x) }
                0x0C => { Argument::Memory(cpu.y) }
                0x0D => { Argument::Memory(cpu.z) }
                0x0E => { Argument::Memory(cpu.i) }
                0x0F => { Argument::Memory(cpu.j) }
                // [register + NEXT_PC]
                0x10 => { Argument::Memory(cpu.a + next_pc(cpu)) }
                0x11 => { Argument::Memory(cpu.b + next_pc(cpu)) }
                0x12 => { Argument::Memory(cpu.c + next_pc(cpu)) }
                0x13 => { Argument::Memory(cpu.x + next_pc(cpu)) }
                0x14 => { Argument::Memory(cpu.y + next_pc(cpu)) }
                0x15 => { Argument::Memory(cpu.z + next_pc(cpu)) }
                0x16 => { Argument::Memory(cpu.i + next_pc(cpu)) }
                0x17 => { Argument::Memory(cpu.j + next_pc(cpu)) }
                // Stack Operations
                0x18 => { Argument::Memory(push_sp(cpu)) }
                0x19 => { Argument::Memory(cpu.sp) }
                0x1A => { Argument::Memory(cpu.sp + next_pc(cpu)) }
                // Specialty Registers
                0x1B => { Argument::Register(Register::SP) }
                0x1C => { Argument::Register(Register::PC) }
                0x1D => { Argument::Register(Register::PS) }
                // Value of Memory at Next Word
                0x1E => { Argument::Memory(next_pc(cpu)) }
                // Next Word as Literal
                0x1F => { Argument::Literal(next_pc(cpu)) }
                _ => { Argument::Literal(0x0000) }
            }
        };
        fn skip_next(cpu: &mut VCPU16) {
            let word = cpu.pc;
            if (word & 0x3FF) == 0 {
                cpu.clk.busy += 1;
                cpu.pc += 1;
            } else if (word & 0x001F) == 0 {
                cpu.clk.busy += 1;
                cpu.pc += 1;
                match (word & 0xFC00) >> 10 {
                    0x10 => { cpu.pc += 1 }
                    0x11 => { cpu.pc += 1 }
                    0x12 => { cpu.pc += 1 }
                    0x13 => { cpu.pc += 1 }
                    0x14 => { cpu.pc += 1 }
                    0x15 => { cpu.pc += 1 }
                    0x16 => { cpu.pc += 1 }
                    0x17 => { cpu.pc += 1 }
                    0x1A => { cpu.pc += 1 }
                    0x1E => { cpu.pc += 1 }
                    0x1F => { cpu.pc += 1 }
                    _ => {}
                }
            } else {
                cpu.clk.busy += 1;
                cpu.pc += 1;
                match (word & 0x03E0) >> 5 {
                    0x10 => { cpu.pc += 1 }
                    0x11 => { cpu.pc += 1 }
                    0x12 => { cpu.pc += 1 }
                    0x13 => { cpu.pc += 1 }
                    0x14 => { cpu.pc += 1 }
                    0x15 => { cpu.pc += 1 }
                    0x16 => { cpu.pc += 1 }
                    0x17 => { cpu.pc += 1 }
                    0x1A => { cpu.pc += 1 }
                    0x1E => { cpu.pc += 1 }
                    0x1F => { cpu.pc += 1 }
                    _ => {}
                }
            }
        }
        if (word & 0x3FF) == 0 {
            // Process Nullary OpCode
            let code = (word & 0xFC00) >> 10;
            match code {
                0x00 => { /* NO-OP */ }
                0x01 => { // CLK
                }
                _ => { /* Error */ }
            };
        } else if (word & 0x001F) == 0 {
            // Process Unary Opcode
            let code = (word & 0x03E0) >> 5;
            let upper = upper(self, word);
            match code {
                0x01 => {
                    // JSR u
                    // Pushes the address of the next instruction to the stack, then sets PC to u
                    self.clk.busy += 3;
                    self.mem.buffer[push_sp(self) as usize] = self.pc;
                    self.pc = read_arg(self, upper);
                }
                0x08 => {
                    // INT u
                    // Triggers a software interrupt with message u
                    self.clk.busy += 4;
                }
                _ => { /* Error */ }
            };
        } else {
            // Process Binary OpCode
            let code = (word & 0x001F) >> 0;
            let u_arg = upper(self, word);
            let m_arg = middle(self, word);
            match code {
                0x01 => {
                    // SET m, u
                    // Sets m to u
                    self.clk.busy += 1;
                    let u_val = read_arg(self, u_arg);
                    write_arg(self, m_arg, u_val);
                }
                0x02 => {
                    // ADD m, u
                    // Sets m to m + u, sets PS to 0x0001 if there's an overflow, 0x0000 otherwise
                    self.clk.busy += 2;
                    let m_val = read_arg(self, m_arg);
                    let u_val = read_arg(self, u_arg);
                    let (result, overflow) = m_val.overflowing_add(u_val);
                    write_arg(self, m_arg, result);
                    if overflow {
                        self.ps = 0x0001;
                    } else {
                        self.ps = 0x0000;
                    }
                }
                0x03 => {
                    // SUB m, u
                    // Sets m to m - u, sets PS to 0xFFFF if there's an underflow, 0x0000 otherwise
                    self.clk.busy += 2;
                    let m_val = read_arg(self, m_arg);
                    let u_val = read_arg(self, u_arg);
                    let (result, overflow) = m_val.overflowing_sub(u_val);
                    write_arg(self, m_arg, result);
                    self.ps = if overflow { 0xFFFF } else { 0x0000 }
                }
                0x04 => {
                    // MUL m, u
                    // Sets m to m * u, sets PS to ((m * u)>>16) & 0xFFFF) (treats m, u as unsigned)
                    self.clk.busy += 2;
                    let m_val = read_arg(self, m_arg) as u32;
                    let u_val = read_arg(self, u_arg) as u32;
                    let result = m_val * u_val;
                    let ps = ((result & 0xFFFF0000) >> 16) as u16;
                    let rv = ((result & 0x0000FFFF) >> 0) as u16;
                    write_arg(self, m_arg, rv);
                    self.ps = ps;
                }
                0x05 => {
                    // MLI m, u
                    // Sets m to (m * u), sets PS to ((m*u)>>16) & 0xFFFF) (treats m, u as signed)
                    self.clk.busy += 2;
                    let m_val = read_arg(self, m_arg) as i32;
                    let u_val = read_arg(self, u_arg) as i32;
                    let result = m_val * u_val;
                    let ps = ((result as u32 & 0xFFFF0000) >> 16) as u16;
                    let rv = ((result as u32 & 0x0000FFFF) >> 0) as u16;
                    write_arg(self, m_arg, rv);
                    self.ps = ps;
                }
                0x06 => {
                    // DIV m, u
                    // Sets m to m / u, sets PS to ((m<<16)/u)&0xFFFF.
                    // If u==0, sets m and PS to 0 instead. (treats m, u as unsigned)
                    self.clk.busy += 3;
                    let m_val = read_arg(self, m_arg) as u32;
                    let u_val = read_arg(self, u_arg) as u32;
                    let (rv, ps) = if u_val != 0 {
                        let rv = ((m_val / u_val) & 0xFFFF) as u16;
                        let ps = (((m_val << 16) / u_val) & 0xFFFF) as u16;
                        (rv, ps)
                    } else {
                        (0, 0)
                    };
                    write_arg(self, m_arg, rv);
                    self.ps = ps;
                }
                0x07 => {
                    // DVI m, u
                    // Sets m to m / u, sets PS to ((m<<16)/u)&0xFFFF.
                    // If u==0, sets m and PS to 0 instead. (treats m, u as signed)
                    self.clk.busy += 3;
                    let m_val = read_arg(self, m_arg) as i32;
                    let u_val = read_arg(self, u_arg) as i32;
                    let (rv, ps) = if u_val != 0 {
                        let rv = ((m_val / u_val) & 0xFFFF) as u16;
                        let ps = (((m_val << 16) / u_val) & 0xFFFF) as u16;
                        (rv, ps)
                    } else {
                        (0, 0)
                    };
                    write_arg(self, m_arg, rv);
                    self.ps = ps;
                }
                0x08 => {
                    // MOD m, u
                    // Sets m to m % u. if u==0, sets m to 0 instead.
                    self.clk.busy += 3;
                    let m_val = read_arg(self, m_arg);
                    let u_val = read_arg(self, u_arg);
                    let rv = if u_val != 0 { m_val % u_val } else { 0 };
                    write_arg(self, m_arg, rv);
                }
                0x09 => {
                    /* MDI m, u */
                    // Sets m to m % u. If u==0, sets m to 0 instead.
                    // (treats m, u as signed [MDI -7, 16 == -7])
                    self.clk.busy += 3;
                    let m_val = read_arg(self, m_arg) as i16;
                    let u_val = read_arg(self, u_arg) as i16;
                    let rv = if u_val != 0 { m_val % u_val } else { 0 };
                    write_arg(self, m_arg, rv as u16);
                }
                0x0A => {
                    // AND m, u
                    // Sets m to m & u
                    self.clk.busy += 1;
                    let m_val = read_arg(self, m_arg);
                    let u_val = read_arg(self, u_arg);
                    let rv = m_val & u_val;
                    write_arg(self, m_arg, rv);
                }
                0x0B => {
                    // BOR m, u
                    // Sets m to m | u
                    self.clk.busy += 1;
                    let m_val = read_arg(self, m_arg);
                    let u_val = read_arg(self, u_arg);
                    let rv = m_val | u_val;
                    write_arg(self, m_arg, rv);
                }
                0x0C => {
                    // XOR m, u
                    // Sets m to m ^ u
                    self.clk.busy += 1;
                    let m_val = read_arg(self, m_arg);
                    let u_val = read_arg(self, u_arg);
                    let rv = m_val ^ u_val;
                    write_arg(self, m_arg, rv);
                }
                0x0D => {
                    // LLS m, u
                    // Sets m to m << u, sets PS to ((m<<u)>>16)&0xFFFF (logical left shift)
                    self.clk.busy += 1;
                    let m_val = read_arg(self, m_arg);
                    let u_val = read_arg(self, u_arg);
                    let rv = m_val << u_val;
                    let ps = ((((m_val as u32) << (u_val as u32)) >> 16) & 0xFFFF) as u16;
                    write_arg(self, m_arg, rv);
                    self.ps = ps;
                }
                0x0E => {
                    // LRS m, u
                    // Sets m to m >> u, sets PS to ((m<<16)>>u)&0xFFFF (logical right shift)
                    self.clk.busy += 1;
                    let m_val = read_arg(self, m_arg);
                    let u_val = read_arg(self, u_arg);
                    let rv = m_val >> u_val;
                    let ps = ((((m_val as u32) << 16) >> u_val) & 0xFFFF) as u16;
                    write_arg(self, m_arg, rv);
                    self.ps = ps;
                }
                0x0F => {
                    // ARS m, u
                    // Sets m to m >>> u, sets PS to ((m<<16)>>>u)&0xFFFF (arithmetic shift) (treats m as signed)
                    self.clk.busy += 1;
                    let m_val = read_arg(self, m_arg) as i16;
                    let u_val = read_arg(self, u_arg) as u16;
                    let rv = m_val >> u_val; // i16 >>> u16
                    let ps = ((((m_val as i32) << 16) >> u_val) & 0xFFFF) as u16;
                    write_arg(self, m_arg, rv as u16);
                    self.ps = ps;
                }
                0x10 => {
                    // IFB m, u
                    // Performs next instruction only if (m & u) != 0
                    self.clk.busy += 2;
                    let m_val = read_arg(self, m_arg);
                    let u_val = read_arg(self, u_arg);
                    if m_val & u_val != 0 {
                        self.execute();
                    } else {
                        skip_next(self);
                    }
                }
                0x11 => {
                    // IFC m, u
                    // Performs next instruction only if (m & u) == 0
                    self.clk.busy += 2;
                    let m_val = read_arg(self, m_arg);
                    let u_val = read_arg(self, u_arg);
                    if m_val & u_val == 0 {
                        self.execute();
                    } else {
                        skip_next(self);
                    }
                }
                0x12 => {
                    // IFE m, u
                    // Performs next instruction only if m == u
                    self.clk.busy += 2;
                    let m_val = read_arg(self, m_arg);
                    let u_val = read_arg(self, u_arg);
                    if m_val == u_val {
                        self.execute();
                    } else {
                        skip_next(self);
                    }
                }
                0x13 => {
                    // IFN m, u
                    // Performs next instruction only if m != u
                    self.clk.busy += 2;
                    let m_val = read_arg(self, m_arg);
                    let u_val = read_arg(self, u_arg);
                    if m_val != u_val {
                        self.execute();
                    } else {
                        skip_next(self);
                    }
                }
                0x14 => {
                    // IFG m, u
                    // Performs next instruction only if m > u (unsigned)
                    self.clk.busy += 2;
                    let m_val = read_arg(self, m_arg);
                    let u_val = read_arg(self, u_arg);
                    if m_val > u_val {
                        self.execute();
                    } else {
                        skip_next(self);
                    }
                }
                0x15 => {
                    // IFA m, u
                    // Performs next instruction only if m > u (signed)
                    self.clk.busy += 2;
                    let m_val = read_arg(self, m_arg) as i16;
                    let u_val = read_arg(self, u_arg) as i16;
                    if m_val > u_val {
                        self.execute();
                    } else {
                        skip_next(self);
                    }
                }
                0x16 => {
                    // IFL m, u
                    // Performs next instruction only if m < u (unsigned)
                    self.clk.busy += 2;
                    let m_val = read_arg(self, m_arg);
                    let u_val = read_arg(self, u_arg);
                    if m_val < u_val {
                        self.execute();
                    } else {
                        skip_next(self);
                    }
                }
                0x17 => {
                    // IFU m, u
                    // Performs next instruction only if m < u (signed)
                    self.clk.busy += 2;
                    let m_val = read_arg(self, m_arg) as i16;
                    let u_val = read_arg(self, u_arg) as i16;
                    if m_val < u_val {
                        self.execute();
                    } else {
                        skip_next(self);
                    }
                }
                0x1A => {
                    // ADX m, u
                    // Sets m to m + u + PS, sets PS to 0x0001 if there is an overflow, 0x0000 otherwise
                    // TODO: Figure out Better Logic Here
                    self.clk.busy += 3;
                    let m_val = read_arg(self, m_arg) as u32;
                    let u_val = read_arg(self, u_arg) as u32;
                    let result = m_val + u_val + self.ps as u32;
                    let rv = (result % 65_536) as u16;
                    let ps = if result / 65_536 > 0 { 0x0001 } else { 0x0000 };
                    write_arg(self, m_arg, rv);
                    self.ps = ps;
                }
                0x1B => {
                    // SBX m, u
                    // Sets m to m - u + PS, sets PS to 0xFFFF if there is an underflow, 0x0000 otherwise
                    // TODO: Figure out Better Logic Here
                    self.clk.busy += 3;
                    let m_val = read_arg(self, m_arg) as u32;
                    let u_val = read_arg(self, u_arg) as u32;
                    let result = m_val - u_val + self.ps as u32;
                    let rv = (result % 65_536) as u16;
                    let ps = if result / 65_536 > 0 { 0xFFFF } else { 0x0000 };
                    write_arg(self, m_arg, rv);
                    self.ps = ps;
                }
                0x1E => {
                    // STI m, u
                    // Sets m to u, then increases I and J by 1
                    self.clk.busy += 2;
                    let u_val = read_arg(self, u_arg);
                    write_arg(self, m_arg, u_val);
                    self.i += 1;
                    self.j += 1;
                }
                0x1F => {
                    // STD m, u
                    // Sets m to u, then decreases I and J by 1
                    self.clk.busy += 2;
                    let u_val = read_arg(self, u_arg);
                    write_arg(self, m_arg, u_val);
                    self.i -= 1;
                    self.j -= 1;
                }
                _ => { /* Error */ }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VCPU16;
    use rand::{Rng, SeedableRng, XorShiftRng};
    use std::io::Cursor;

    #[test]
    pub fn test_loadsave() {
        // Create our Memory and external buffers
        let mut cpu = VCPU16::new();
        let mut input: [u8; 131072] = [0; 131072];
        let mut output: [u8; 131072] = [0; 131072];

        // Fill our input Buffer
        XorShiftRng::from_seed([1; 4]).fill_bytes(&mut input[..]);

        // Load our input into Memory
        cpu.load_mem(&mut Cursor::new(&mut input[..]));

        // Save our memory to output
        cpu.save_mem(&mut Cursor::new(&mut output[..]));

        // Compare buffers
        assert_eq!(&input[..], &output[..]);
    }

    #[test]
    pub fn test_set_get() {
        // Create our Memory and external buffers
        let mut cpu = VCPU16::new();

        let address: u16 = 0xFFFF;
        let oldvalue: u16 = 0x0000;
        let newvalue: u16 = 0x2222;

        // Assert Memory at address equals oldvalue
        assert_eq!(oldvalue, cpu.get_mem(address));

        // Set Memory at address to newvalue
        cpu.set_mem(address, newvalue);

        // Assert Memory at address equals newvalue
        assert_eq!(newvalue, cpu.get_mem(address));
    }

    #[test]
    pub fn test_read_write() {
        // Create our Memory and external buffers
        let mut cpu = VCPU16::new();

        let address: u16 = 0x1111;
        let oldvalue: u16 = 0x0000;
        let newvalue: u16 = 0x2222;

        // Assert Memory at address equals oldvalue
        assert_eq!(oldvalue, cpu.get(address));

        // Set Memory at address to newvalue
        ram.set(address, newvalue);

        // Assert Memory at address equals newvalue
        assert_eq!(newvalue, ram.get(address));
    }
}