//! EVM (Ethereum Virtual Machine) implementation in Rust
//! 
//! This crate provides a complete implementation of the EVM with the following modules:
//! - `types`: Core data types and configuration
//! - `stack`: Stack operations
//! - `memory`: Memory management
//! - `gas`: Gas calculation and tracking
//! - `opcodes`: Opcode definitions and execution framework
//! - `state`: EVM execution state management
//! - `vm`: Main VM orchestration

pub mod types;
pub mod stack;
pub mod memory;
pub mod gas;
pub mod opcodes;
pub mod state;
pub mod vm;

// Re-export main types for convenience
pub use types::{EvmConfig, EvmResult, EvmError, Address, Word, Gas};
pub use vm::{Evm, EvmBuilder};
pub use state::EvmState;

/// Execute EVM bytecode with default configuration
/// 
/// This is a convenience function that creates a default EVM instance
/// and executes the provided bytecode.
/// 
/// # Arguments
/// * `code` - EVM bytecode to execute
/// 
/// # Returns
/// * `EvmResult` - The result of execution including success status, gas used, and return data
/// 
/// # Example
/// ```
/// use evm::evm;
/// 
/// let code = vec![0x00]; // STOP instruction
/// let result = evm(code);
/// assert!(result.success);
/// ```
pub fn evm(code: impl AsRef<[u8]>) -> EvmResult {
    let vm = Evm::default();
    vm.execute(code.as_ref().to_vec())
}

/// Execute EVM bytecode with transaction data
/// 
/// This function creates an EVM instance with the specified transaction data
/// and executes the provided bytecode.
/// 
/// # Arguments
/// * `code` - EVM bytecode to execute
/// * `to` - Contract address (or zero for contract creation)
/// * `from` - Sender address
/// * `value` - Transaction value
/// 
/// # Returns
/// * `EvmResult` - The result of execution including success status, gas used, and return data
/// 
/// # Example
/// ```
/// use evm::evm_with_tx;
/// 
/// let code = vec![0x30]; // ADDRESS instruction
/// let to = [0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xAA];
/// let from = [0u8; 20];
/// let value = U256::zero();
/// let result = evm_with_tx(code, to, from, value);
/// ```
pub fn evm_with_tx(code: impl AsRef<[u8]>, to: Address, from: Address, value: Word) -> EvmResult {
    let mut config = EvmConfig::default();
    config.transaction.to = to;
    config.transaction.from = from;
    config.transaction.value = value;
    
    let vm = Evm::new(config);
    vm.execute(code.as_ref().to_vec())
}
