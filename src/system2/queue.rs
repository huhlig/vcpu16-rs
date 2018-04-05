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
use super::SystemError;
use super::Word;

/// Interrupt Request Queue
#[derive(Clone, Copy)]
pub struct Queue {
    interrupts: [Word; 256],
    write: u8,
    read: u8,
}

impl fmt::Debug for Queue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IRQ ( Disabled: {} Queue: ", self.enabled)?;
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

impl Queue {
    pub fn new() -> Queue {
        Queue {
            interrupts: [0; 256],
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
    pub fn enqueue(&mut self, value: Word) -> Result<(), SystemError> {
        if self.write.wrapping_add(1) == self.read {
            return Err(SystemError::InterruptOverflow);
        }
        self.interrupts[self.write as usize] = value;
        self.write = self.write.wrapping_add(1);
        Ok(())
    }
    pub fn dequeue(&mut self) -> Result<Word, SystemError> {
        if self.read == self.write {
            return Err(SystemError::InterruptUnderflow);
        }
        let value = self.interrupts[self.read as usize];
        self.read = self.read.wrapping_add(1);
        Ok(value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_pic() {
        let mut queue = Queue::new();

        assert!(queue.is_empty());
        assert!(queue.is_disabled());
        for input in 0..512u16 {
            queue.enqueue(input).unwrap();
            queue.enable();
            assert!(queue.is_enabled());
            let output = queue.dequeue().unwrap();
            assert_eq!(input, output);
            queue.disable();
        }
    }

    #[test]
    pub fn test_fill() {
        let mut irq = Queue::new();

        assert!(irq.is_empty());
        assert!(irq.is_disabled());
        for input in 0..255u16 {
            irq.enqueue(input).unwrap();
        }

        assert_eq!(IRQError::QueueFull, irq.enqueue(0).unwrap_err());
    }
}