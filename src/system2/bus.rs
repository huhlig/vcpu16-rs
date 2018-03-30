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

/// Bus is a set of 3 Parallel Registers
pub struct Bus {
    /// Interrupts
    interrupts: Map<Word,Vec<Word>>,
    /// Bus Register X
    x: Word,
    /// Bus Register Y
    y: Word,
    /// Bus Register Z
    z: Word,
}

impl Bus {
    /// Create a new Bus
    pub fn new() -> Bus {
        Bus {
            interrupts: Map::new(),
            x: 0,
            y: 0,
            z: 0,
        }
    }
    pub fn x(&self) -> Word {
        self.x
    }
    pub fn y(&self) -> Word {
        self.y
    }
    pub fn z(&self) -> Word {
        self.z
    }
}


#[cfg(test)]
mod tests {
    use super::Bus;

    #[test]
    pub fn test_bus() {
        let bus = Bus::new();
        assert_eq!(bus.x(), 0);
        assert_eq!(bus.y(), 0);
        assert_eq!(bus.z(), 0);
        bus.x = 0xABCD;
        bus.y = 0xABCD;
        bus.z = 0xABCD;
        assert_eq!(bus.x(), 0xABCD);
        assert_eq!(bus.y(), 0xABCD);
        assert_eq!(bus.z(), 0xABCD);
    }
