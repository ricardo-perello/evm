use crate::types::{EvmError, Word};

/// EVM memory implementation
/// Memory is a byte array that can be expanded as needed
pub struct Memory {
    data: Vec<u8>,
    active_words: usize,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            active_words: 0,
        }
    }

    /// Read data from memory
    pub fn read(&self, offset: usize, size: usize) -> Result<Vec<u8>, EvmError> {
        if offset + size > self.data.len() {
            return Err(EvmError::MemoryOutOfBounds);
        }
        
        let mut result = Vec::with_capacity(size);
        for i in 0..size {
            result.push(self.data[offset + i]);
        }
        Ok(result)
    }

    /// Write data to memory
    pub fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), EvmError> {
        let required_size = offset + data.len();
        
        if required_size > self.data.len() {
            self.expand(required_size)?;
        }
        
        for (i, &byte) in data.iter().enumerate() {
            self.data[offset + i] = byte;
        }
        
        // Update active words if we wrote beyond current active area
        let new_active_words = (required_size + 31) / 32; // Round up to nearest word
        if new_active_words > self.active_words {
            self.active_words = new_active_words;
        }
        
        Ok(())
    }

    /// Expand memory to accommodate the required size
    pub fn expand(&mut self, size: usize) -> Result<(), EvmError> {
        if size > self.data.len() {
            self.data.resize(size, 0);
        }
        Ok(())
    }

    /// Get the current memory size in bytes
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Get the current memory size in words (32-byte chunks)
    pub fn size_words(&self) -> usize {
        self.active_words
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
