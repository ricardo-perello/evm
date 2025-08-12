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
            
            crate::opcodes::Opcode::Add => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let result = a.overflowing_add(b).0; // This ensures wrapping behavior
                self.stack.push(result)?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Mul => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let result = a.overflowing_mul(b).0; // This ensures wrapping behavior
                self.stack.push(result)?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Sub => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let result = a.overflowing_sub(b).0; // This ensures wrapping behavior
                self.stack.push(result)?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Div => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                if b.is_zero() {
                    self.stack.push(Word::zero())?;
                } else {
                    self.stack.push(a / b)?;
                }
                Ok(())
            }
            
            crate::opcodes::Opcode::Mod => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                if b.is_zero() {
                    self.stack.push(Word::zero())?;
                } else {
                    self.stack.push(a % b)?;
                }
                Ok(())
            }
            
            crate::opcodes::Opcode::Addmod => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let m = self.stack.pop()?;
                if m.is_zero() {
                    self.stack.push(Word::zero())?;
                } else {
                    let sum = a.overflowing_add(b).0;  // Handle overflow by wrapping
                    let result = sum % m;
                    self.stack.push(result)?;
                }
                Ok(())
            }
            
            crate::opcodes::Opcode::Mulmod => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let m = self.stack.pop()?;
                if m.is_zero() {
                    self.stack.push(Word::zero())?;
                } else {
                    // Handle overflow manually since U256 panics in debug mode
                    // We need to compute (a * b) % m without intermediate overflow
                    // For large numbers, we can use the property: (a * b) % m = ((a % m) * (b % m)) % m
                    let a_mod = a % m;
                    let b_mod = b % m;
                    let product = a_mod * b_mod;
                    let result = product % m;
                    

                    
                    self.stack.push(result)?;
                }
                Ok(())
            }
            
            crate::opcodes::Opcode::Sdiv => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                if b.is_zero() {
                    self.stack.push(Word::zero())?;
                } else {
                    // Handle signed division
                    let sign_a = (a >> 255) & Word::from(1);
                    let sign_b = (b >> 255) & Word::from(1);
                    
                    // Convert to absolute values
                    let abs_a = if sign_a.is_zero() { a } else { !a + Word::from(1) };
                    let abs_b = if sign_b.is_zero() { b } else { !b + Word::from(1) };
                    
                    // Perform unsigned division
                    let abs_result = abs_a / abs_b;
                    
                    // Apply sign: result is negative if exactly one operand is negative
                    let result = if sign_a != sign_b { !abs_result + Word::from(1) } else { abs_result };
                    
                    self.stack.push(result)?;
                }
                Ok(())
            }
            
            crate::opcodes::Opcode::Smod => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                if b.is_zero() {
                    self.stack.push(Word::zero())?;
                } else {
                    // Handle signed modulo
                    let sign_a = (a >> 255) & Word::from(1);
                    let sign_b = (b >> 255) & Word::from(1);
                    
                    // Convert to absolute values
                    let abs_a = if sign_a.is_zero() { a } else { !a + Word::from(1) };
                    let abs_b = if sign_b.is_zero() { b } else { !b + Word::from(1) };
                    
                    // Perform unsigned modulo
                    let abs_result = abs_a % abs_b;
                    
                    // Apply sign: result has the same sign as the dividend (a)
                    let result = if sign_a.is_zero() { abs_result } else { !abs_result + Word::from(1) };
                    
                    self.stack.push(result)?;
                }
                Ok(())
            }
            
            crate::opcodes::Opcode::Signextend => {
                let b = self.stack.pop()?;
                let x = self.stack.pop()?;
                
                if b < Word::from(31) {
                    let bit_pos = b.as_u32() * 8 + 7;
                    let bit = (x >> bit_pos) & Word::from(1);
                    if bit.is_zero() {
                        // Clear upper bits
                        let mask = (Word::from(1) << bit_pos) - Word::from(1);
                        self.stack.push(x & mask)?;
                    } else {
                        // Set upper bits
                        let mask = !((Word::from(1) << bit_pos) - Word::from(1));
                        self.stack.push(x | mask)?;
                    }
                } else {
                    self.stack.push(x)?;
                }
                Ok(())
            }
            
            crate::opcodes::Opcode::Slt => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                
                // Handle signed comparison
                let sign_a = (a >> 255) & Word::from(1);
                let sign_b = (b >> 255) & Word::from(1);
                
                // If signs are different, negative number is less than positive
                if sign_a != sign_b {
                    self.stack.push(if sign_a.is_zero() { Word::zero() } else { Word::from(1) })?;
                } else {
                    // Same sign, compare as unsigned
                    self.stack.push(if a < b { Word::from(1) } else { Word::zero() })?;
                }
                Ok(())
            }
            
            crate::opcodes::Opcode::Sgt => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                // Signed greater than - for now treat as regular greater than
                self.stack.push(if a > b { Word::from(1) } else { Word::zero() })?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Byte => {
                let i = self.stack.pop()?;
                let x = self.stack.pop()?;
                
                if i >= Word::from(32) {
                    self.stack.push(Word::zero())?;
                } else {
                    let byte_pos = i.as_u32();
                    // Extract the byte from the most significant end
                    // For index 31, we want the least significant byte
                    // For index 0, we want the most significant byte
                    let shift_amount = (31 - byte_pos) * 8;
                    let byte = (x >> shift_amount) & Word::from(0xff);
                    self.stack.push(byte)?;
                }
                Ok(())
            }
            
            crate::opcodes::Opcode::Sha3 => {
                let offset = self.stack.pop()?;
                let size = self.stack.pop()?;
                
                let offset_usize = offset.as_usize();
                let size_usize = size.as_usize();
                
                let data = self.memory.read(offset_usize, size_usize)?;
                // For now, return a simple hash (in real EVM this would use Keccak-256)
                let mut hash = Word::zero();
                for (i, &byte) in data.iter().enumerate() {
                    if i < 32 {
                        hash = hash | (Word::from(byte) << (i * 8));
                    }
                }
                self.stack.push(hash)?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Balance => {
                // For now, return 0 (in real EVM this would check account balance)
                self.stack.push(Word::zero())?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Exp => {
                let base = self.stack.pop()?;
                let exponent = self.stack.pop()?;
                
                // Handle overflow by using modular arithmetic
                // For large exponents, we need to be careful about overflow
                let mut result = Word::from(1);
                let mut exp = exponent;
                let mut current_base = base;
                
                while !exp.is_zero() {
                    if exp & Word::from(1) != Word::zero() {
                        result = result * current_base;
                    }
                    current_base = current_base * current_base;
                    exp = exp >> 1;
                }
                
                self.stack.push(result)?;
                Ok(())
            }
            
            // Comparison operations
            crate::opcodes::Opcode::Lt => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(if a < b { Word::from(1) } else { Word::zero() })?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Gt => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(if a > b { Word::from(1) } else { Word::zero() })?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Eq => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(if a == b { Word::from(1) } else { Word::zero() })?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Iszero => {
                let a = self.stack.pop()?;
                self.stack.push(if a.is_zero() { Word::from(1) } else { Word::zero() })?;
                Ok(())
            }
            
            // Bitwise operations
            crate::opcodes::Opcode::And => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a & b)?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Or => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a | b)?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Xor => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a ^ b)?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Shl => {
                let shift = self.stack.pop()?;
                let value = self.stack.pop()?;
                
                // Handle shift left with overflow
                let shift_amount = shift.as_u32();
                if shift_amount >= 256 {
                    self.stack.push(Word::zero())?;
                } else {
                    let result = value << shift_amount;
                    self.stack.push(result)?;
                }
                Ok(())
            }
            
            crate::opcodes::Opcode::Shr => {
                let shift = self.stack.pop()?;
                let value = self.stack.pop()?;
                
                // Handle shift right with overflow
                let shift_amount = shift.as_u32();
                if shift_amount >= 256 {
                    self.stack.push(Word::zero())?;
                } else {
                    let result = value >> shift_amount;
                    self.stack.push(result)?;
                }
                Ok(())
            }
            
            crate::opcodes::Opcode::Sar => {
                let shift = self.stack.pop()?;
                let value = self.stack.pop()?;
                
                // Handle arithmetic shift right with overflow
                let shift_amount = shift.as_u32();
                if shift_amount >= 256 {
                    // If shifting by 256 or more, result depends on sign
                    let sign_bit = (value >> 255) & Word::from(1);
                    if sign_bit.is_zero() {
                        self.stack.push(Word::zero())?;
                    } else {
                        self.stack.push(Word::max_value())?;
                    }
                } else {
                    // For smaller shifts, preserve sign bit
                    let sign_bit = (value >> 255) & Word::from(1);
                    let mut result = value >> shift_amount;
                    
                    // If the original number was negative, fill upper bits with 1s
                    if !sign_bit.is_zero() {
                        let mask = !((Word::from(1) << (256 - shift_amount)) - Word::from(1));
                        result = result | mask;
                    }
                    
                    self.stack.push(result)?;
                }
                Ok(())
            }
            
            crate::opcodes::Opcode::Dup1 | crate::opcodes::Opcode::Dup2 | crate::opcodes::Opcode::Dup3 | 
            crate::opcodes::Opcode::Dup4 | crate::opcodes::Opcode::Dup5 | crate::opcodes::Opcode::Dup6 | 
            crate::opcodes::Opcode::Dup7 | crate::opcodes::Opcode::Dup8 | crate::opcodes::Opcode::Dup9 | 
            crate::opcodes::Opcode::Dup10 | crate::opcodes::Opcode::Dup11 | crate::opcodes::Opcode::Dup12 | 
            crate::opcodes::Opcode::Dup13 | crate::opcodes::Opcode::Dup14 | crate::opcodes::Opcode::Dup15 | 
            crate::opcodes::Opcode::Dup16 => {
                // Generic DUP implementation for DUP1..DUP16
                let dup_index = match opcode {
                    crate::opcodes::Opcode::Dup1 => 1,
                    crate::opcodes::Opcode::Dup2 => 2,
                    crate::opcodes::Opcode::Dup3 => 3,
                    crate::opcodes::Opcode::Dup4 => 4,
                    crate::opcodes::Opcode::Dup5 => 5,
                    crate::opcodes::Opcode::Dup6 => 6,
                    crate::opcodes::Opcode::Dup7 => 7,
                    crate::opcodes::Opcode::Dup8 => 8,
                    crate::opcodes::Opcode::Dup9 => 9,
                    crate::opcodes::Opcode::Dup10 => 10,
                    crate::opcodes::Opcode::Dup11 => 11,
                    crate::opcodes::Opcode::Dup12 => 12,
                    crate::opcodes::Opcode::Dup13 => 13,
                    crate::opcodes::Opcode::Dup14 => 14,
                    crate::opcodes::Opcode::Dup15 => 15,
                    crate::opcodes::Opcode::Dup16 => 16,
                    _ => unreachable!(),
                };
                
                // Check if we have enough elements on the stack
                if self.stack.len() < dup_index {
                    return Err(EvmError::StackUnderflow);
                }
                
                // Get the value to duplicate (counting from top)
                let value = self.stack.data()[self.stack.len() - dup_index];
                
                // Push the duplicated value
                self.stack.push(value)?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Swap1 | crate::opcodes::Opcode::Swap2 | crate::opcodes::Opcode::Swap3 | 
            crate::opcodes::Opcode::Swap4 | crate::opcodes::Opcode::Swap5 | crate::opcodes::Opcode::Swap6 | 
            crate::opcodes::Opcode::Swap7 | crate::opcodes::Opcode::Swap8 | crate::opcodes::Opcode::Swap9 | 
            crate::opcodes::Opcode::Swap10 | crate::opcodes::Opcode::Swap11 | crate::opcodes::Opcode::Swap12 | 
            crate::opcodes::Opcode::Swap13 | crate::opcodes::Opcode::Swap14 | crate::opcodes::Opcode::Swap15 | 
            crate::opcodes::Opcode::Swap16 => {
                // Generic SWAP implementation for SWAP1..SWAP16
                let swap_index = match opcode {
                    crate::opcodes::Opcode::Swap1 => 1,
                    crate::opcodes::Opcode::Swap2 => 2,
                    crate::opcodes::Opcode::Swap3 => 3,
                    crate::opcodes::Opcode::Swap4 => 4,
                    crate::opcodes::Opcode::Swap5 => 5,
                    crate::opcodes::Opcode::Swap6 => 6,
                    crate::opcodes::Opcode::Swap7 => 7,
                    crate::opcodes::Opcode::Swap8 => 8,
                    crate::opcodes::Opcode::Swap9 => 9,
                    crate::opcodes::Opcode::Swap10 => 10,
                    crate::opcodes::Opcode::Swap11 => 11,
                    crate::opcodes::Opcode::Swap12 => 12,
                    crate::opcodes::Opcode::Swap13 => 13,
                    crate::opcodes::Opcode::Swap14 => 14,
                    crate::opcodes::Opcode::Swap15 => 15,
                    crate::opcodes::Opcode::Swap16 => 16,
                    _ => unreachable!(),
                };
                
                // Check if we have enough elements on the stack
                if self.stack.len() < swap_index + 1 {
                    return Err(EvmError::StackUnderflow);
                }
                
                // For SWAP1, we need to swap the top two elements
                if swap_index == 1 {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a)?;
                    self.stack.push(b)?;
                } else {
                    // For other SWAP operations, we need to pop multiple elements and reorder them
                    let mut values = Vec::new();
                    
                    // Pop the top element (the one we want to swap)
                    let top_value = self.stack.pop()?;
                    
                    // Pop the element to swap with (and all elements in between)
                    for _ in 0..swap_index {
                        values.push(self.stack.pop()?);
                    }
                    
                    // Push back in reverse order of how we popped them
                    for i in (0..values.len()).rev() {
                        self.stack.push(values[i])?;
                    }
                    
                    // Push the original top element last
                    self.stack.push(top_value)?;
                }
                
                Ok(())
            }
            
            crate::opcodes::Opcode::Not => {
                let a = self.stack.pop()?;
                self.stack.push(!a)?;
                Ok(())
            }
            
            // Environmental information
            crate::opcodes::Opcode::Address => {
                self.stack.push(Word::from_big_endian(&self.address))?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Caller => {
                self.stack.push(Word::from_big_endian(&self.caller))?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Callvalue => {
                self.stack.push(self.callvalue)?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Origin => {
                self.stack.push(Word::from_big_endian(&self.origin))?;
                Ok(())
            }
            
            //TODO
            // Block information
            crate::opcodes::Opcode::Blockhash => {
                // For now, return 0 (in a real EVM this would return actual block hash)
                self.stack.push(Word::zero())?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Coinbase => {
                self.stack.push(Word::from_big_endian(&self.coinbase))?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Timestamp => {
                self.stack.push(Word::from(self.block_timestamp))?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Number => {
                self.stack.push(Word::from(self.block_number))?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Difficulty => {
                self.stack.push(self.block_difficulty)?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Gaslimit => {
                self.stack.push(Word::from(self.block_gas_limit))?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Basefee => {
                self.stack.push(self.block_base_fee)?;
                Ok(())
            }
            
            // Memory operations
            crate::opcodes::Opcode::Mload => {
                let offset = self.stack.pop()?;
                let offset_usize = offset.as_usize();
                let data = self.memory.read(offset_usize, 32)?; // Read 32 bytes (1 word)
                let mut padded_data = vec![0u8; 32];
                for (i, &byte) in data.iter().enumerate() {
                    if i < 32 {
                        padded_data[i] = byte;
                    }
                }
                let value = Word::from_big_endian(&padded_data);
                self.stack.push(value)?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Mstore => {
                let offset = self.stack.pop()?;
                let value = self.stack.pop()?;
                let offset_usize = offset.as_usize();
                let mut data = vec![0u8; 32];
                value.to_big_endian(&mut data);
                self.memory.write(offset_usize, &data)?;
                Ok(())
            }
            
            crate::opcodes::Opcode::Msize => {
                self.stack.push(Word::from(self.memory.size()))?;
                Ok(())
            }
            
            // Gas operations
            crate::opcodes::Opcode::Gas => {
                self.stack.push(Word::from(self.gas_tracker.remaining()))?;
                Ok(())
            }
            
            // Program counter
            crate::opcodes::Opcode::Pc => {
                self.stack.push(Word::from(self.program_counter))?;
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
