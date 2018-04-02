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

use super::Clock;
use super::Memory;
use super::PIC;
use super::Registers;
use super::SSystemError;
use super::Word;

/// Hardware Trait
trait Hardware {
    /// Get Manufacturer ID
    fn mfg_id(&self) -> Word;
    /// Get Hardware ID
    fn hdw_id(&self) -> Word;
    /// Get Device ID
    fn dev_id(&self) -> Word;
    /// Trigger Device Interrupt
    fn interrupt(&mut self, value: Word) -> SystemError;
    /// Increment Device one Cycle
    fn update(&mut self, clock: &Clock, registers: &mut Registers, memory: &mut Memory, pic: &mut PIC) -> SystemError;
}