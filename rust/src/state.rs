use crate::types::{EvmError, EvmConfig, Word, Address};
use crate::stack::Stack;
use crate::memory::Memory;
use crate::gas::GasTracker;

/// EVM execution state
pub struct EvmState {
    pub stack: Stack,
    pub memory: Memory,
    pub gas_tracker: GasTracker,
    pub program_counter: usize,
    pub code: Vec<u8>,
    pub return_data: Vec<u8>,
    pub logs: Vec<crate::types::Log>,
    
    // Account state (simplified for now)
    pub address: Address,
    pub caller: Address,
    pub callvalue: Word,
    pub origin: Address,
    
    // Block context
    pub block_number: u64,
    pub block_timestamp: u64,
    pub block_difficulty: Word,
    pub block_gas_limit: u64,
    pub block_base_fee: Word,
    pub coinbase: Address,
    
    // Execution flags
    pub halted: bool,
    pub reverted: bool,
}

impl EvmState {
    pub fn new(code: Vec<u8>, config: EvmConfig) -> Self {
        Self {
            stack: Stack::new(),
            memory: Memory::new(),
            gas_tracker: GasTracker::new(config.gas_limit),
            program_counter: 0,
            code,
            return_data: Vec::new(),
            logs: Vec::new(),
            
            // Default account state
            address: [0u8; 20],
            caller: [0u8; 20],
            callvalue: Word::zero(),
            origin: [0u8; 20],
            
            // Block context from config
            block_number: config.block_number,
            block_timestamp: config.block_timestamp,
            block_difficulty: config.block_difficulty,
            block_gas_limit: config.block_gas_limit,
            block_base_fee: config.block_base_fee,
            coinbase: [0u8; 20],
            
            // Execution flags
            halted: false,
            reverted: false,
        }
    }

    /// Execute a single step of the EVM
    pub fn step(&mut self) -> Result<(), EvmError> {
        if self.halted || self.reverted {
            return Ok(());
        }

        if self.program_counter >= self.code.len() {
            self.halted = true;
            return Ok(());
        }

        // Fetch and decode opcode
        let opcode_byte = self.code[self.program_counter];
        let opcode = crate::opcodes::Opcode::from_byte(opcode_byte)
            .ok_or_else(|| EvmError::InvalidOpcode(opcode_byte))?;

        // Consume gas for the opcode
        self.gas_tracker.consume(opcode.gas_cost())?;

        // Execute the opcode
        self.execute_opcode(opcode)?;

        // Increment program counter (unless opcode modified it)
        if !self.is_jump_opcode(opcode) {
            self.program_counter += 1;
        }

        Ok(())
    }

    /// Execute a specific opcode
    fn execute_opcode(&mut self, opcode: crate::opcodes::Opcode) -> Result<(), EvmError> {
        match opcode {
            crate::opcodes::Opcode::Stop => {
                self.halted = true;
                Ok(())
            }
            
            crate::opcodes::Opcode::Pop => {
                self.stack.pop()?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Push0 => {
                self.stack.push(Word::zero())
            }
            
            crate::opcodes::Opcode::Push1 | crate::opcodes::Opcode::Push2 | crate::opcodes::Opcode::Push3 | 
            crate::opcodes::Opcode::Push4 | crate::opcodes::Opcode::Push5 | crate::opcodes::Opcode::Push6 | 
            crate::opcodes::Opcode::Push7 | crate::opcodes::Opcode::Push8 | crate::opcodes::Opcode::Push9 | 
            crate::opcodes::Opcode::Push10 | crate::opcodes::Opcode::Push11 | crate::opcodes::Opcode::Push12 | 
            crate::opcodes::Opcode::Push13 | crate::opcodes::Opcode::Push14 | crate::opcodes::Opcode::Push15 | 
            crate::opcodes::Opcode::Push16 | crate::opcodes::Opcode::Push17 | crate::opcodes::Opcode::Push18 | 
            crate::opcodes::Opcode::Push19 | crate::opcodes::Opcode::Push20 | crate::opcodes::Opcode::Push21 | 
            crate::opcodes::Opcode::Push22 | crate::opcodes::Opcode::Push23 | crate::opcodes::Opcode::Push24 | 
            crate::opcodes::Opcode::Push25 | crate::opcodes::Opcode::Push26 | crate::opcodes::Opcode::Push27 | 
            crate::opcodes::Opcode::Push28 | crate::opcodes::Opcode::Push29 | crate::opcodes::Opcode::Push30 | 
            crate::opcodes::Opcode::Push31 | crate::opcodes::Opcode::Push32 => {
                let size = (opcode as u8 - 0x60) + 1;
                let size = size as usize;
                
                if self.program_counter + size >= self.code.len() {
                    return Err(EvmError::Unknown("Invalid PUSH operation".to_string()));
                }
                
                let mut value = Word::zero();
                for i in 0..size {
                    value = value << 8 | Word::from(self.code[self.program_counter + 1 + i]);
                }
                
                self.stack.push(value)?;
                self.program_counter += size;
                Ok(())
            }
            
            _ => {
                // For now, return an error for unimplemented opcodes
                Err(EvmError::Unknown(format!("Opcode {:?} not implemented", opcode)))
            }
        }
    }

    /// Check if an opcode is a jump operation
    fn is_jump_opcode(&self, opcode: crate::opcodes::Opcode) -> bool {
        matches!(opcode, crate::opcodes::Opcode::Jump | crate::opcodes::Opcode::Jumpi)
    }

    /// Get the current execution status
    pub fn status(&self) -> ExecutionStatus {
        if self.reverted {
            ExecutionStatus::Reverted
        } else if self.halted {
            ExecutionStatus::Halted
        } else {
            ExecutionStatus::Running
        }
    }

    /// Get the final result of execution
    pub fn result(&self) -> crate::types::EvmResult {
        crate::types::EvmResult {
            success: !self.reverted,
            gas_used: self.gas_tracker.gas_used(),
            stack: self.stack.data().iter().rev().cloned().collect(),
            return_data: self.return_data.clone(),
            logs: self.logs.clone(),
        }
    }
}

/// Execution status of the EVM
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionStatus {
    Running,
    Halted,
    Reverted,
}

impl Default for EvmState {
    fn default() -> Self {
        Self::new(Vec::new(), EvmConfig::default())
    }
}
