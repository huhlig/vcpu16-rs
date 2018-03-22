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

/// Clock Errors
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClockError {
    /// Attempted to Advance a halted Clock
    Halted,
}

/// System Clock
#[derive(Clone, Copy, Debug)]
pub struct Clock {
    halted: bool,
    cycles: u64,
}

impl Clock {
    /// Create a new Clock
    pub fn new() -> Clock {
        Clock {
            halted: false,
            cycles: 0,
        }
    }
    /// Is Clock Still Active
    pub fn halted(&self) -> bool {
        self.halted
    }
    /// Current Clock Cycles Since Startup
    pub fn cycles(&self) -> u64 {
        self.cycles
    }
    /// Advance Clock if not halted
    pub fn advance(&mut self) -> Result<u64, ClockError> {
        if self.halted {
            Err(ClockError::Halted)
        } else {
            self.cycles += 1;
            Ok(self.cycles)
        }
    }
    /// Halt Clock
    pub fn halt(&mut self) {
        self.halted = true;
    }
}

#[cfg(test)]
mod tests {
    use super::Clock;

    #[test]
    pub fn test_clock() {
        let mut clk = Clock::new();

        for expected in 0..200u64 {
            assert_eq!(expected, clk.cycles());
            clk.advance().unwrap();
        }
        clk.halt();
        assert!(clk.halted())
    }
}