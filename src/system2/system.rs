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

use super::Bus;
use super::Clock;
use super::Hardware;
use super::Memory;
use super::Queue;
use super::PIC;
use super::Registers;
use super::State;
use super::SystemError;
use std::fmt;

/// A System is a container for all Hardware.
/// A Primary CPU always exists in Hardware Slot 0.
pub struct System {
    /// System Registers
    registers: Registers,
    /// System Hardware
    hardware: Vec<Box<Hardware>>,
    /// System Memory
    memory: Memory,
    /// System Clock
    clock: Clock,
    /// System State
    state: State,
    /// Interrupt Request Queue
    irq: Queue,
}

impl System {
    /// Create a new System
    pub fn new() -> System {
        System {
            registers: Registers::new(),
            hardware: Vec::new(),
            memory: Memory::new(),
            clock: Clock::new(),
            state: State::Idle,
            irq: Queue::new(),
        }
    }
    /// Step the System forward one clock cycle
    pub fn step(&mut self) -> Result<(), SystemError> {
        // Advance the clock
        self.clock.step()?;
        match self.state {
            State::Idle => {
                /// Fetch
                let base_address = self.registers.pc;
                let opcode_word = self.memory.get(base_address);
                self.registers.pc += 1;
            },
        };
        // Fetch
        let base = self.registers.pc;
        let word = self.memory.get(base);
        // Decode


        // Iterate through Hardware
        for device in &self.hardware {
            //registers: &Registers, memory: &Memory, clock: &Clock, pic: &PIC
            device.update(&self.clock, &mut self.registers, &mut self.memory, &mut PIC)?;
        }
    }
}

