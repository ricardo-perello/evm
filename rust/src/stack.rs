use crate::types::{EvmError, Word};

/// EVM stack implementation
/// The EVM stack has a maximum size of 1024 items
pub struct Stack {
    data: Vec<Word>,
    max_size: usize,
}

impl Stack {
    pub const MAX_SIZE: usize = 1024;

    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            max_size: Self::MAX_SIZE,
        }
    }

    /// Push a value onto the stack
    pub fn push(&mut self, value: Word) -> Result<(), EvmError> {
        if self.data.len() >= self.max_size {
            return Err(EvmError::StackOverflow);
        }
        self.data.push(value);
        Ok(())
    }

    /// Pop a value from the stack
    pub fn pop(&mut self) -> Result<Word, EvmError> {
        self.data.pop().ok_or(EvmError::StackUnderflow)
    }

    /// Get the current stack size
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get a reference to the internal data (for testing/debugging)
    pub fn data(&self) -> &[Word] {
        &self.data
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
