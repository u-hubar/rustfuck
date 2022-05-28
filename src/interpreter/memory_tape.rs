use std::io::{self, Read};

pub struct MemoryTape {
    pub storage: Vec<u8>,
    pub pointer: u16,
}

impl MemoryTape {
    pub fn new(pointer: u16) -> Self {
        let storage: Vec<u8> = vec![0; u16::MAX as usize + 1];

        Self {
            storage,
            pointer,
        }
    }

    pub fn move_left(&mut self, step: u16) {
        self.pointer = self.pointer.wrapping_sub(step);
    }

    pub fn move_right(&mut self, step: u16) {
        self.pointer = self.pointer.wrapping_add(step);
    }

    pub fn increment(&mut self, addend: u8) {
        self.storage[self.pointer as usize] = self.storage[
            self.pointer as usize
        ].wrapping_add(addend);
    }

    pub fn decrement(&mut self, subtrahend: u8) {
        self.storage[self.pointer as usize] = self.storage[
            self.pointer as usize
        ].wrapping_sub(subtrahend);
    }

    pub fn clear(&mut self) {
        self.storage[self.pointer as usize] = 0;
    }

    pub fn output(&mut self) {
        print!("{}", self.storage[self.pointer as usize] as char)
    }

    pub fn input(&mut self) {
        let mut input: [u8; 1] = [0];
        io::stdin()
            .read_exact(&mut input)
            .expect("Failed to read input from stdin");
        self.storage[self.pointer as usize] = input[0];
    }
}
