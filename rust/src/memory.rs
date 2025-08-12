use crate::types::{EvmError, Word};

/// EVM memory implementation
/// Memory is a byte array that can be expanded as needed
pub struct Memory {
    data: Vec<u8>,
    active_words: usize,
    accessed: bool,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            active_words: 0,
            accessed: false,
        }
    }

    /// Read data from memory
    /// If reading beyond memory bounds, pad with zeros (Ethereum specification)
    /// Also expands memory to accommodate the read operation
    pub fn read(&mut self, offset: usize, size: usize) -> Result<Vec<u8>, EvmError> {
        let required_size = offset + size;
        
        // Mark memory as accessed
        self.accessed = true;
        
        // Expand memory if needed
        if required_size > self.data.len() {
            self.expand(required_size)?;
        }
        
        let mut result = Vec::with_capacity(size);
        
        for i in 0..size {
            let read_offset = offset + i;
            if read_offset < self.data.len() {
                result.push(self.data[read_offset]);
            } else {
                result.push(0); // Pad with zeros for out-of-bounds reads
            }
        }
        
        // Update active words if we read beyond current active area
        let new_active_words = (required_size + 31) / 32; // Round up to nearest word
        if new_active_words > self.active_words {
            self.active_words = new_active_words;
        }
        
        Ok(result)
    }

    /// Write data to memory
    pub fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), EvmError> {
        let required_size = offset + data.len();
        
        // Mark memory as accessed
        self.accessed = true;
        
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
    
    /// Get the highest accessed memory index
    pub fn highest_accessed_index(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1
        }
    }
    
    /// Check if memory has been accessed
    pub fn has_been_accessed(&self) -> bool {
        self.accessed
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
