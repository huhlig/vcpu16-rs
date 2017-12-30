use std::convert::From;
use std::convert::Into;
use std::io::{Read, Write};
use std::mem;
use std::slice;

pub struct Memory {
    memory: [u16; 65536],
}

impl Memory {
    ///
    /// Create new Memory
    ///
    pub fn new() -> Memory {
        Memory { memory: [0; 65536] }
    }
    ///
    /// Load Memory from Reader
    ///
    pub fn load(&mut self, reader: &mut Read) {
        unsafe {
            let memory_size = mem::size_of_val(&self.memory);
            let memory_slice = slice::from_raw_parts_mut(
                &mut self.memory as *mut _ as *mut u8,
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
            let memory_size = mem::size_of_val(&self.memory);
            let memory_slice = slice::from_raw_parts_mut(
                &mut self.memory as *mut _ as *mut u8,
                memory_size,
            );
            writer.write(memory_slice).unwrap();
        }
    }
    ///
    /// Write a slice of memory from buffer
    ///
    pub fn write(&mut self, address: u16, buffer: &[u16]) {
        self.memory[address as usize..buffer.len()].copy_from_slice(buffer)
    }
    ///
    /// Read a slice length of memory at address
    ///
    pub fn read(&mut self, address: u16, length: u16) -> &[u16] {
        &self.memory[address as usize..length as usize]
    }
    ///
    /// Set a single Cell of Memory at address
    ///
    pub fn set(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = value
    }
    ///
    /// Get a single Cell of Memory at address
    ///
    pub fn get(&self, address: u16) -> u16 {
        self.memory[address as usize]
    }
}

impl<'a> From<&'a [u16]> for Memory {
    fn from(buffer: &'a [u16]) -> Self {
        let mut memory = [0; 65536];
        memory.clone_from_slice(buffer);
        Memory { memory }
    }
}

impl<'a> Into<&'a [u16]> for Memory {
    fn into(self) -> &'a [u16] {
        &self.memory
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;
    use rand::{Rng, SeedableRng, XorShiftRng};
    use std::io::Cursor;

    #[test]
    pub fn test_loadsave() {
        // Create our Memory and external buffers
        let mut ram = Memory::new();
        let mut input: [u8; 131072] = [0; 131072];
        let mut output: [u8; 131072] = [0; 131072];

        // Fill our input Buffer
        XorShiftRng::from_seed([1; 4]).fill_bytes(&mut input[..]);

        // Load our input into Memory
        ram.load(&mut Cursor::new(&mut input[..]));

        // Save our memory to output
        ram.save(&mut Cursor::new(&mut output[..]));

        // Compare buffers
        assert_eq!(&input[..], &output[..]);
    }

    #[test]
    pub fn test_set_get() {
        // Create our Memory and external buffers
        let mut ram = Memory::new();

        let address: u16 = 0xFFFF;
        let oldvalue: u16 = 0x0000;
        let newvalue: u16 = 0x2222;

        // Assert Memory at address equals oldvalue
        assert_eq!(oldvalue, ram.get(address));

        // Set Memory at address to newvalue
        ram.set(address, newvalue);

        // Assert Memory at address equals newvalue
        assert_eq!(newvalue, ram.get(address));
    }

    #[test]
    pub fn test_read_write() {
        // Create our Memory and external buffers
        let mut ram = Memory::new();

        let address: u16 = 0x1111;
        let oldvalue: u16 = 0x0000;
        let newvalue: u16 = 0x2222;

        // Assert Memory at address equals oldvalue
        assert_eq!(oldvalue, ram.get(address));

        // Set Memory at address to newvalue
        ram.set(address, newvalue);

        // Assert Memory at address equals newvalue
        assert_eq!(newvalue, ram.get(address));
    }
}