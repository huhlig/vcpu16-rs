use std::error::Error;
use std::result::Result;

pub struct CircularBuffer<T> {
    buffer: [T; 256],
    write: usize,
    read: usize,
}

enum BufferError {
    BufferFull,
}

impl std::error::Error for BufferError {
    fn description(&self) -> &str {
        match self {
            BufferError::BufferFull => &"Buffer is full"
        }
    }
}

impl CircularBuffer<T> {
    pub fn new() -> CircularBuffer<T> {
        CircularBuffer {
            buffer: [0; 256],
            write: 0,
            read: 0,
        }
    }
    pub fn enqueue(&mut self, value: T) -> Result<(), BufferError> {

    }
    pub fn dequeue(&mut self) -> Result<T, BufferError> {}
}