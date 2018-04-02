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

/// Errors thrown by the System
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SystemError {
    /// Clock has Stopped
    ClockHalted,
    /// Failure during Hardware Update
    HardwareFailure,
    /// Address has Overflowed
    AddressOverflow,
    /// Interrupt Queue has Overflowed
    InterruptOverflow,
    /// Interrupt Queue has Underflowed
    InterruptUnderflow,
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Clock Halted: {} Cycles: {}", match f {
            &SystemError::ClockHalted => "SystemError::ClockHalted",
            &SystemError::HardwareFailure => "SystemError::HardwareFailure",
            &SystemError::AddressOverflow => "SystemError::AddressOverflow",
            &SystemError::InterruptOverflow => "SystemError::InterruptOverflow",
            &SystemError::InterruptUnderflow => "SystemError::InterruptUnderflow",
        })
    }
}

impl fmt::Debug for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Clock Halted: {} Cycles: {}", match f {
            &SystemError::ClockHalted => "SystemError::ClockHalted",
            &SystemError::HardwareFailure => "SystemError::HardwareFailure",
            &SystemError::AddressOverflow => "SystemError::AddressOverflow",
            &SystemError::InterruptOverflow => "SystemError::InterruptOverflow",
            &SystemError::InterruptUnderflow => "SystemError::InterruptUnderflow",
        })
    }
}
