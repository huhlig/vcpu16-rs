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

mod clock;
mod error;
mod memory;
mod queue;
mod registers;
mod program;
mod system;

pub mod hardware;
pub use self::bus::Bus;
pub use self::clock::Clock;
pub use self::error::SystemError;
pub use self::memory::Memory;
pub use self::sic::PIC;
pub use self::registers::Registers;
pub use self::program::State;
pub use self::system::System;

/// System Word
pub type Word = u16;
