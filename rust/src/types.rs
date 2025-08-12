use primitive_types::U256;

/// Core EVM data types
pub type Address = [u8; 20];
pub type Word = U256;
pub type Gas = u64;

/// Transaction data
#[derive(Debug, Clone)]
pub struct Transaction {
    pub to: Address,      // Contract address (or zero for contract creation)
    pub from: Address,    // Sender address
    pub value: U256,      // Transaction value
    pub gas_price: U256,  // Gas price
}

/// EVM configuration
#[derive(Debug, Clone)]
pub struct EvmConfig {
    pub gas_limit: Gas,
    pub block_number: u64,
    pub block_timestamp: u64,
    pub block_difficulty: U256,
    pub block_gas_limit: U256,
    pub block_base_fee: U256,
    pub coinbase: Address,
    pub transaction: Transaction,
}

impl Default for EvmConfig {
    fn default() -> Self {
        Self {
            gas_limit: 30_000_000,
            block_number: 0,
            block_timestamp: 0,
            block_difficulty: U256::zero(),
            block_gas_limit: U256::from(30_000_000),
            block_base_fee: U256::from(1),
            coinbase: [0u8; 20],
            transaction: Transaction {
                to: [0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, 0xAA],
                from: [0x1E, 0x79, 0xB0, 0x45, 0xDC, 0x29, 0xEA, 0xE9, 0xFD, 0xC6, 0x96, 0x73, 0xC9, 0xDC, 0xD7, 0xC5, 0x3E, 0x5E, 0x15, 0x9D],
                value: U256::zero(),
                gas_price: U256::from(0x99),
            },
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

/// Block information for test configuration
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Block {
    pub coinbase: Option<String>,
    pub basefee: Option<String>,
    pub gaslimit: Option<String>,
    pub number: Option<String>,
    pub timestamp: Option<String>,
    pub difficulty: Option<String>,
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
