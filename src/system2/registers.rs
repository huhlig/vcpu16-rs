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
use std::fmt;

/// VCPU16 Internal Registers
#[derive(Clone, Copy)]
pub struct Registers {
    /// Stack Pointer
    pub sp: Word,
    /// Program Counter
    pub pc: Word,
    /// Program Status
    pub ps: Word,
    /// Interrupt Address
    pub ia: Word,
    /// System Flags
    pub sf: Word,
    /// Register A
    pub a: Word,
    /// Register B
    pub b: Word,
    /// Register C
    pub c: Word,
    /// Register X
    pub x: Word,
    /// Register Y
    pub y: Word,
    /// Register Z
    pub z: Word,
    /// Register I
    pub i: Word,
    /// Register J
    pub j: Word,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            pc: 0,
            sp: 0,
            ps: 0,
            ia: 0,
            sf: 0,
            a: 0,
            b: 0,
            c: 0,
            x: 0,
            y: 0,
            z: 0,
            i: 0,
            j: 0,
        }
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PC: {:04X}, SP: {:04X}, PS: {:04X}, IA: {:04X}, A: {:04X}, B: {:04X}, C: {:04X}, \
        X: {:04X}, Y: {:04X}, Z: {:04X}, I: {:04X}, J: {:04X}", self.pc, self.sp, self.ps, self.ia,
               self.a, self.b, self.c, self.x, self.y, self.z, self.i, self.j)
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Registers ( PC: {:04X}, SP: {:04X}, PS: {:04X}, IA: {:04X}, A: {:04X}, B: {:04X}, \
        C: {:04X}, X: {:04X}, Y: {:04X}, Z: {:04X}, I: {:04X}, J: {:04X} )", self.pc, self.sp,
               self.ps, self.ia, self.a, self.b, self.c, self.x, self.y, self.z, self.i, self.j)
    }
}

#[cfg(test)]
mod tests {
    use super::Registers;

    #[test]
    pub fn test_display() {
        println!();
        let reg = Registers::new();

        println!("{}", reg);
    }
}
