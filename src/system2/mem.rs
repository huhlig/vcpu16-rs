//
// Copyright 2017 Hans W. Uhlig.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use super::Word;
use std::char;
use std::fmt;
use std::io::{Read, Write};
use std::mem;
use std::slice;

/// Memory Array
pub struct Memory {
    pub(crate) buffer: [Word; 65536],
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
            let memory_size = mem::size_of_val(&self.buffer);
            let memory_slice = slice::from_raw_parts_mut(
                &mut self.buffer as *mut _ as *mut u8,
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
            let memory_size = mem::size_of_val(&self.buffer);
            let memory_slice = slice::from_raw_parts_mut(
                &mut self.buffer as *mut _ as *mut u8,
                memory_size,
            );
            writer.write(memory_slice).unwrap();
        }
    }
    ///
    /// Clear Memory
    ///
    pub fn clear(&mut self) {
        self.buffer = [0; 65536];
    }
    ///
    /// Write a slice of memory from buffer
    ///
    pub fn write(&mut self, address: Word, buffer: &[Word]) {
        self.buffer[address as usize..buffer.len()].copy_from_slice(buffer)
    }
    ///
    /// Read a slice length of memory at address
    ///
    pub fn read(&mut self, address: Word, length: Word) -> &[Word] {
        &self.buffer[address as usize..length as usize]
    }
    ///
    /// Set a single Cell of Memory at address
    ///
    pub fn set(&mut self, address: Word, value: Word) {
        self.buffer[address as usize] = value
    }
    ///
    /// Get a single Cell of Memory at address
    ///
    pub fn get(&self, address: Word) -> Word {
        self.buffer[address as usize]
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Memory    0    1    2    3    4    5    6    7    8    9    A    B    C    D    E    F")?;
        for base in (0..4096usize).map(|o| o * 16) {  // for o in (0..65536).step_by(16) {
            write!(f, "0x{:04X}", base)?;
            for offset in 0..16usize {
                write!(f, " {:04X}", self.buffer[base + offset])?;
            }
            write!(f, " ")?;
            for offset in 0..16usize {
                if let Some(ch) = char::from_u32(self.buffer[base + offset] as u32) {
                    if ch.is_ascii_alphanumeric() {
                        write!(f, "{}", ch)?;
                    } else {
                        write!(f, "{}", '.')?;
                    }
                } else {
                    write!(f, "{}", '.')?;
                }
            }
            writeln!(f, " ")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Word;
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

        let read_address: Word = 0x0100;
        let write_address: Word = 0x0104;
        let write_buffer: [Word; 8] = [1; 8];
        let empty_buffer: [Word; 16] = [0; 16];
        let dirty_buffer: [Word; 16] = [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0];

        // Assert Buffer written
        assert_eq!(&empty_buffer[..], mem.read(read_address, 16));

        // Write buffer
        mem.write(write_address, &write_buffer);

        // Assert Data Written
        assert_eq!(&dirty_buffer[..], mem.read(read_address, 16));

        // Clear memory
        mem.clear();

        // Assert Data Cleared
        assert_eq!(&empty_buffer[..], mem.read(read_address, 16));
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

    #[test]
    pub fn test_display() {
        // Create our Memory and external buffers
        let mut mem = Memory::new();

        // TODO: Change this to an inclusive range ..=
        for addr in 0..65536u32 {
            let addr = addr as u16;
            mem.set(addr, addr);
        }

        println!("{}", mem);
    }
}