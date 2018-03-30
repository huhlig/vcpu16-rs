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
use super::Memory;
use super::hardware::VCPU16;
use std::fmt;

/// A System is a container for all Hardware.
/// A Primary CPU always exists in Hardware Slot 0.
pub struct System {
    hardware: Vec<Box<Hardware>>,
    memory: Memory,
    clock: Clock,
    bus: Bus,
}

impl System {
    /// Create a new System
    pub fn new() -> System {
        System {
            hardware: vec![VCPU16::new(0)],
            memory: Memory::new(),
            clock: Clock::new(),
            bus: Bus::new(),
        }
    }
    /// Step the System forward one clock cycle
    pub fn step(&mut self) -> Result<(), Box<Error>> {
        // Advance the clock
        self.clk.step()?;
        // Iterate through Hardware
        for device in &self.hardware {
            device.update(self.bus, self.clock, self.memory)?;
        }
    }
}
