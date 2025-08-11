use primitive_types::U256;

/// Core EVM data types
pub type Address = [u8; 20];
pub type Word = U256;
pub type Gas = u64;

/// EVM configuration
#[derive(Debug, Clone)]
pub struct EvmConfig {
    pub gas_limit: Gas,
    pub block_number: u64,
    pub block_timestamp: u64,
    pub block_difficulty: U256,
    pub block_gas_limit: Gas,
    pub block_base_fee: U256,
}

impl Default for EvmConfig {
    fn default() -> Self {
        Self {
            gas_limit: 30_000_000,
            block_number: 0,
            block_timestamp: 0,
            block_difficulty: U256::zero(),
            block_gas_limit: 30_000_000,
            block_base_fee: U256::zero(),
        }
    }
}

/// EVM execution result
#[derive(Debug, Clone)]
pub struct EvmResult {
    pub success: bool,
    pub gas_used: Gas,
    pub stack: Vec<Word>,
    pub return_data: Vec<u8>,
    pub logs: Vec<Log>,
}

/// EVM log entry
#[derive(Debug, Clone)]
pub struct Log {
    pub address: Address,
    pub topics: Vec<U256>,
    pub data: Vec<u8>,
}

/// EVM execution error
#[derive(Debug, Clone)]
pub enum EvmError {
    OutOfGas,
    InvalidOpcode(u8),
    StackUnderflow,
    StackOverflow,
    MemoryOutOfBounds,
    InvalidJumpDestination,
    ExecutionReverted,
    Unknown(String),
}
