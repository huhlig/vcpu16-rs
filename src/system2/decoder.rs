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

/// CPU State
pub enum State {
    /// Step 1
    FetchBase {
        address: Word,
    },
    Decode {
        address: Word,

    },

    Execute {
        address: Word,
    },
}

#[derive(Copy, Clone, Debug)]
enum Register {
    PC,
    SP,
    PS,
    A,
    B,
    C,
    X,
    Y,
    Z,
    I,
    J,
}

#[derive(Copy, Clone, Debug)]
enum Argument {
    Memory(Word),
    Literal(Word),
    Register(Register),
}

enum OpCode {
    SET,
    ADD,
    SUB,
    MUL,
    MLI,
    DIV,
    DVI,
    MOD,
    MDI,
    AND,
    BOR,
    XOR,
    LLS,
    LRS,
    ARS,
    IFB,
    IFC,
    IFE,
    IFN,
    IFG,
    IFA,
    IFL,
    IFU,
    ADX,
    SBX,
    STI,
    STD,
}

