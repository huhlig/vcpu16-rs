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

mod reg;

use self::reg::Registers;
use super::PIC;
use super::Word;

pub struct VCPU16 {
    bus: Word,
    reg: Registry,
    pic: PIC,
}

impl VCPU16 {
    pub fn new(bus_id: Word) -> VCPU16 {
        VCPU16 {
            bus: bus_id,
            reg: Registers::new(),
            pic: PIC::new(),
        }
    }
}

impl Hardware for VCPU16 {
    /// Get Manufacturer ID
    fn mfg_id(&self) -> Word {
        0x0000u16
    }
    /// Get Hardware ID
    fn hdw_id(&self) -> Word {
        0x0000u16
    }
    fn bus_id(&self) -> Word {
        self.bus
    }
    /// Trigger Device Interrupt
    fn interrupt(&mut self, value: Word) {
        match self.pic.enqueue(word) {
            Err(err) => (), // Handle Error
            Ok() => (),
        }
    }
    /// Increment Device one Cycle
    fn update(&mut self, clk: Clock, mem: Memory) {
        // Handle CPU
    }
}