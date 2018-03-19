

/// Memory Array
pub struct Memory {
    buffer: [u16; 65536],
}

impl Memory {
    ///
    /// Create new Memory Buffer
    ///
    pub fn new() -> Memory {
        Memory {
            buffer: [0; 65536],
        }
    }
    ///
    /// Load Memory from Reader
    ///
    pub fn load(&mut self, reader: &mut Read) {
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
    pub fn save(&mut self, writer: &mut Write) {
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
    pub fn clear(&mut self) {
        self.mem.buffer = [0; 65536];
    }
    ///
    /// Write a slice of memory from buffer
    ///
    pub fn write(&mut self, address: u16, buffer: &[u16]) {
        self.mem.buffer[address as usize..buffer.len()].copy_from_slice(buffer)
    }
    ///
    /// Read a slice length of memory at address
    ///
    pub fn read(&mut self, address: u16, length: u16) -> &[u16] {
        &self.mem.buffer[address as usize..length as usize]
    }
    ///
    /// Set a single Cell of Memory at address
    ///
    pub fn set(&mut self, address: u16, value: u16) {
        self.mem.buffer[address as usize] = value
    }
    ///
    /// Get a single Cell of Memory at address
    ///
    pub fn get(&self, address: u16) -> u16 {
        self.mem.buffer[address as usize]
    }
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result<(), Error> {
        write!(f, "          0    1    2    3    4    5    6    7    8    9    A    B    C    D    E    F");
        for o in 0..65536.step_by(16) {
            write!(f, "0x{:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X}", o,
                   self.buffer[o + 0x0], self.buffer[o + 0x1], self.buffer[o + 0x2], self.buffer[o + 0x3],
                   self.buffer[o + 0x4], self.buffer[o + 0x5], self.buffer[o + 0x6], self.buffer[o + 0x7],
                   self.buffer[o + 0x8], self.buffer[o + 0x9], self.buffer[o + 0xA], self.buffer[o + 0xB],
                   self.buffer[o + 0xC], self.buffer[o + 0xD], self.buffer[o + 0xE], self.buffer[o + 0xF]
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;
    use rand::{Rng, SeedableRng, XorShiftRng};
    use std::io::Cursor;

    #[test]
    pub fn test_load_save() {
        // Create our Memory and external buffers
        let mut mem = Memory::new();
        let mut input: [u8; 131072] = [0; 131072];
        let mut output: [u8; 131072] = [0; 131072];

        // Fill our input Buffer
        XorShiftRng::from_seed([1; 4]).fill_bytes(&mut input[..]);

        // Load our input into Memory
        mem.load(&mut Cursor::new(&mut input[..]));

        // Save our memory to output
        mem.save(&mut Cursor::new(&mut output[..]));

        // Compare buffers
        assert_eq!(&input[..], &output[..]);
    }

    #[test]
    pub fn test_write_clear_read() {
        // Create our Memory and external buffers
        let mut mem = Memory::new();

        let read_address: u16 = 0x0100;
        let write_address: u16 = 0x0104;
        let write_buffer: [u16; 8] = [1; 8];
        let clear_buffer: [u16; 16] = [0; 16];
        let dirty_buffer: [u16; 16] = [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0];

        // Assert Buffer written
        assert_eq!(&empty_buffer[..], &mem.read(address - 8, 24));

        // Write buffer
        mem.write(address, &buffer);

        // Assert Data Written
        assert_eq!(&dirty_buffer[..], &mem.read(address - 8, 24));

        // Clear memory1
        mem.clear();

        // Assert Data Cleared
        assert_eq!(&empty_buffer[..], &mem.read(address - 8, 24));
    }

    #[test]
    pub fn test_set_get() {
        // Create our Memory and external buffers
        let mut mem = Memory::new();

        let address: u16 = 0xFFFF;
        let oldvalue: u16 = 0x0000;
        let newvalue: u16 = 0x2222;

        // Assert Memory at address equals oldvalue
        assert_eq!(oldvalue, mem.get(address));

        // Set Memory at address to newvalue
        mem.set(address, newvalue);

        // Assert Memory at address equals newvalue
        assert_eq!(newvalue, mem.get(address));
    }
}