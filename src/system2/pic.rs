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

use std::fmt;
use super::Word;

/// Errors Returned while Interacting
/// with the Programmable Interrupt Controller
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterruptError {
    /// Queue is Empty during a dequeue
    QueueEmpty,
    /// Queue is Full during an enqueue
    QueueFull,
    /// Queueing is Enabled
    Enabled,
}

/// Programmable Interrupt Controller
#[derive(Clone, Copy)]
pub struct PIC {
    interrupts: [Word; 256],
    enabled: bool,
    write: u8,
    read: u8,
}

impl fmt::Debug for PIC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PIC ( Disabled: {} Queue: ", self.enabled)?;
        if self.read < self.write {
            write!(f, "[")?;
            for i in &self.interrupts[self.read as usize..self.write as usize] {
                write!(f, " 0x{:04X}", i)?;
            }
            write!(f, " ]")?;
        } else if self.read > self.write {
            write!(f, "[")?;
            for i in &self.interrupts[self.read as usize..] {
                write!(f, " 0x{:04X}", i)?;
            }
            for i in &self.interrupts[..self.write as usize] {
                write!(f, " 0x{:04X}", i)?;
            }
            write!(f, " ]")?;
        } else {
            write!(f, "empty")?;
        }
        write!(f, " ] )")
    }
}

impl PIC {
    pub fn new() -> PIC {
        PIC {
            interrupts: [0; 256],
            enabled: false,
            write: 0,
            read: 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.read == self.write
    }
    pub fn is_full(&self) -> bool {
        self.write.wrapping_add(1) == self.read
    }
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    pub fn is_disabled(&self) -> bool {
        !self.enabled
    }
    pub fn enable(&mut self) {
        self.enabled = true
    }
    pub fn disable(&mut self) {
        self.enabled = false
    }
    pub fn enqueue(&mut self, value: Word) -> Result<(), InterruptError> {
        if self.write.wrapping_add(1) == self.read {
            return Err(InterruptError::QueueFull);
        }
        self.interrupts[self.write as usize] = value;
        self.write = self.write.wrapping_add(1);
        Ok(())
    }
    pub fn dequeue(&mut self) -> Result<Word, InterruptError> {
        if self.read == self.write {
            return Err(InterruptError::QueueEmpty);
        }
        let value = self.interrupts[self.read as usize];
        self.read = self.read.wrapping_add(1);
        Ok(value)
    }
}


#[cfg(test)]
mod tests {
    use super::{InterruptError, PIC};

    #[test]
    pub fn test_pic() {
        let mut pic = PIC::new();

        assert!(pic.is_empty());
        assert!(pic.is_disabled());
        for input in 0..512u16 {
            pic.enqueue(input).unwrap();
            pic.enable();
            assert!(pic.is_enabled());
            let output = pic.dequeue().unwrap();
            assert_eq!(input, output);
            pic.disable();
        }
    }

    #[test]
    pub fn test_fill() {
        let mut pic = PIC::new();

        assert!(pic.is_empty());
        assert!(pic.is_disabled());
        for input in 0..255u16 {
            pic.enqueue(input).unwrap();
        }

        assert_eq!(InterruptError::QueueFull, pic.enqueue(0).unwrap_err());
    }
}