use crate::types::{EvmError, Gas};
use crate::gas::{GAS_BASE, GAS_VERY_LOW, GAS_LOW, GAS_MID, GAS_HIGH, GAS_EXTCODE, GAS_SLOAD};

/// EVM opcodes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    // Stop and arithmetic operations
    Stop = 0x00,
    Add = 0x01,
    Mul = 0x02,
    Sub = 0x03,
    Div = 0x04,
    Sdiv = 0x05,
    Mod = 0x06,
    Smod = 0x07,
    Addmod = 0x08,
    Mulmod = 0x09,
    Exp = 0x0a,
    Signextend = 0x0b,
    
    // Comparison and bitwise operations
    Lt = 0x10,
    Gt = 0x11,
    Slt = 0x12,
    Sgt = 0x13,
    Eq = 0x14,
    Iszero = 0x15,
    And = 0x16,
    Or = 0x17,
    Xor = 0x18,
    Not = 0x19,
    Byte = 0x1a,
    Shl = 0x1b,
    Shr = 0x1c,
    Sar = 0x1d,
    
    // SHA3
    Sha3 = 0x20,
    
    // Environmental information
    Address = 0x30,
    Balance = 0x31,
    Origin = 0x32,
    Caller = 0x33,
    Callvalue = 0x34,
    Calldataload = 0x35,
    Calldatasize = 0x36,
    Calldatacopy = 0x37,
    Codesize = 0x38,
    Codecopy = 0x39,
    Gasprice = 0x3a,
    Extcodesize = 0x3b,
    Extcodecopy = 0x3c,
    Returndatasize = 0x3d,
    Returndatacopy = 0x3e,
    Extcodehash = 0x3f,
    
    // Block information
    Blockhash = 0x40,
    Coinbase = 0x41,
    Timestamp = 0x42,
    Number = 0x43,
    Difficulty = 0x44,
    Gaslimit = 0x45,
    Chainid = 0x46,
    Selfbalance = 0x47,
    Basefee = 0x48,
    
    // Stack, memory, storage and flow operations
    Pop = 0x50,
    Mload = 0x51,
    Mstore = 0x52,
    Mstore8 = 0x53,
    Sload = 0x54,
    Sstore = 0x55,
    Jump = 0x56,
    Jumpi = 0x57,
    Pc = 0x58,
    Msize = 0x59,
    Gas = 0x5a,
    Jumpdest = 0x5b,
    
    // Push operations
    Push0 = 0x5f,
    Push1 = 0x60,
    Push2 = 0x61,
    Push3 = 0x62,
    Push4 = 0x63,
    Push5 = 0x64,
    Push6 = 0x65,
    Push7 = 0x66,
    Push8 = 0x67,
    Push9 = 0x68,
    Push10 = 0x69,
    Push11 = 0x6a,
    Push12 = 0x6b,
    Push13 = 0x6c,
    Push14 = 0x6d,
    Push15 = 0x6e,
    Push16 = 0x6f,
    Push17 = 0x70,
    Push18 = 0x71,
    Push19 = 0x72,
    Push20 = 0x73,
    Push21 = 0x74,
    Push22 = 0x75,
    Push23 = 0x76,
    Push24 = 0x77,
    Push25 = 0x78,
    Push26 = 0x79,
    Push27 = 0x7a,
    Push28 = 0x7b,
    Push29 = 0x7c,
    Push30 = 0x7d,
    Push31 = 0x7e,
    Push32 = 0x7f,
    
    // Duplication operations
    Dup1 = 0x80,
    Dup2 = 0x81,
    Dup3 = 0x82,
    Dup4 = 0x83,
    Dup5 = 0x84,
    Dup6 = 0x85,
    Dup7 = 0x86,
    Dup8 = 0x87,
    Dup9 = 0x88,
    Dup10 = 0x89,
    Dup11 = 0x8a,
    Dup12 = 0x8b,
    Dup13 = 0x8c,
    Dup14 = 0x8d,
    Dup15 = 0x8e,
    Dup16 = 0x8f,
    
    // Exchange operations
    Swap1 = 0x90,
    Swap2 = 0x91,
    Swap3 = 0x92,
    Swap4 = 0x93,
    Swap5 = 0x94,
    Swap6 = 0x95,
    Swap7 = 0x96,
    Swap8 = 0x97,
    Swap9 = 0x98,
    Swap10 = 0x99,
    Swap11 = 0x9a,
    Swap12 = 0x9b,
    Swap13 = 0x9c,
    Swap14 = 0x9d,
    Swap15 = 0x9e,
    Swap16 = 0x9f,
    
    // Logging operations
    Log0 = 0xa0,
    Log1 = 0xa1,
    Log2 = 0xa2,
    Log3 = 0xa3,
    Log4 = 0xa4,
    
    // System operations
    Create = 0xf0,
    Call = 0xf1,
    Callcode = 0xf2,
    Return = 0xf3,
    Delegatecall = 0xf4,
    Create2 = 0xf5,
    Staticcall = 0xfa,
    Revert = 0xfd,
    Selfdestruct = 0xff,
}

impl Opcode {
    /// Get opcode from byte value
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Opcode::Stop),
            0x01 => Some(Opcode::Add),
            0x02 => Some(Opcode::Mul),
            0x03 => Some(Opcode::Sub),
            0x04 => Some(Opcode::Div),
            0x05 => Some(Opcode::Sdiv),
            0x06 => Some(Opcode::Mod),
            0x07 => Some(Opcode::Smod),
            0x08 => Some(Opcode::Addmod),
            0x09 => Some(Opcode::Mulmod),
            0x0a => Some(Opcode::Exp),
            0x0b => Some(Opcode::Signextend),
            0x10 => Some(Opcode::Lt),
            0x11 => Some(Opcode::Gt),
            0x12 => Some(Opcode::Slt),
            0x13 => Some(Opcode::Sgt),
            0x14 => Some(Opcode::Eq),
            0x15 => Some(Opcode::Iszero),
            0x16 => Some(Opcode::And),
            0x17 => Some(Opcode::Or),
            0x18 => Some(Opcode::Xor),
            0x19 => Some(Opcode::Not),
            0x1a => Some(Opcode::Byte),
            0x1b => Some(Opcode::Shl),
            0x1c => Some(Opcode::Shr),
            0x1d => Some(Opcode::Sar),
            0x20 => Some(Opcode::Sha3),
            0x30 => Some(Opcode::Address),
            0x31 => Some(Opcode::Balance),
            0x32 => Some(Opcode::Origin),
            0x33 => Some(Opcode::Caller),
            0x34 => Some(Opcode::Callvalue),
            0x35 => Some(Opcode::Calldataload),
            0x36 => Some(Opcode::Calldatasize),
            0x37 => Some(Opcode::Calldatacopy),
            0x38 => Some(Opcode::Codesize),
            0x39 => Some(Opcode::Codecopy),
            0x3a => Some(Opcode::Gasprice),
            0x3b => Some(Opcode::Extcodesize),
            0x3c => Some(Opcode::Extcodecopy),
            0x3d => Some(Opcode::Returndatasize),
            0x3e => Some(Opcode::Returndatacopy),
            0x3f => Some(Opcode::Extcodehash),
            0x40 => Some(Opcode::Blockhash),
            0x41 => Some(Opcode::Coinbase),
            0x42 => Some(Opcode::Timestamp),
            0x43 => Some(Opcode::Number),
            0x44 => Some(Opcode::Difficulty),
            0x45 => Some(Opcode::Gaslimit),
            0x46 => Some(Opcode::Chainid),
            0x47 => Some(Opcode::Selfbalance),
            0x48 => Some(Opcode::Basefee),
            0x50 => Some(Opcode::Pop),
            0x51 => Some(Opcode::Mload),
            0x52 => Some(Opcode::Mstore),
            0x53 => Some(Opcode::Mstore8),
            0x54 => Some(Opcode::Sload),
            0x55 => Some(Opcode::Sstore),
            0x56 => Some(Opcode::Jump),
            0x57 => Some(Opcode::Jumpi),
            0x58 => Some(Opcode::Pc),
            0x59 => Some(Opcode::Msize),
            0x5a => Some(Opcode::Gas),
            0x5b => Some(Opcode::Jumpdest),
            0x5f => Some(Opcode::Push0),
            0x60..=0x7f => {
                // PUSH1..PUSH32
                let size = (byte - 0x60) + 1;
                match size {
                    1 => Some(Opcode::Push1),
                    2 => Some(Opcode::Push2),
                    3 => Some(Opcode::Push3),
                    4 => Some(Opcode::Push4),
                    5 => Some(Opcode::Push5),
                    6 => Some(Opcode::Push6),
                    7 => Some(Opcode::Push7),
                    8 => Some(Opcode::Push8),
                    9 => Some(Opcode::Push9),
                    10 => Some(Opcode::Push10),
                    11 => Some(Opcode::Push11),
                    12 => Some(Opcode::Push12),
                    13 => Some(Opcode::Push13),
                    14 => Some(Opcode::Push14),
                    15 => Some(Opcode::Push15),
                    16 => Some(Opcode::Push16),
                    17 => Some(Opcode::Push17),
                    18 => Some(Opcode::Push18),
                    19 => Some(Opcode::Push19),
                    20 => Some(Opcode::Push20),
                    21 => Some(Opcode::Push21),
                    22 => Some(Opcode::Push22),
                    23 => Some(Opcode::Push23),
                    24 => Some(Opcode::Push24),
                    25 => Some(Opcode::Push25),
                    26 => Some(Opcode::Push26),
                    27 => Some(Opcode::Push27),
                    28 => Some(Opcode::Push28),
                    29 => Some(Opcode::Push29),
                    30 => Some(Opcode::Push30),
                    31 => Some(Opcode::Push31),
                    32 => Some(Opcode::Push32),
                    _ => None,
                }
            }
            0x80..=0x8f => {
                // DUP1..DUP16
                let index = (byte - 0x80) + 1;
                match index {
                    1 => Some(Opcode::Dup1),
                    2 => Some(Opcode::Dup2),
                    3 => Some(Opcode::Dup3),
                    4 => Some(Opcode::Dup4),
                    5 => Some(Opcode::Dup5),
                    6 => Some(Opcode::Dup6),
                    7 => Some(Opcode::Dup7),
                    8 => Some(Opcode::Dup8),
                    9 => Some(Opcode::Dup9),
                    10 => Some(Opcode::Dup10),
                    11 => Some(Opcode::Dup11),
                    12 => Some(Opcode::Dup12),
                    13 => Some(Opcode::Dup13),
                    14 => Some(Opcode::Dup14),
                    15 => Some(Opcode::Dup15),
                    16 => Some(Opcode::Dup16),
                    _ => None,
                }
            }
            0x90..=0x9f => {
                // SWAP1..SWAP16
                let index = (byte - 0x90) + 1;
                match index {
                    1 => Some(Opcode::Swap1),
                    2 => Some(Opcode::Swap2),
                    3 => Some(Opcode::Swap3),
                    4 => Some(Opcode::Swap4),
                    5 => Some(Opcode::Swap5),
                    6 => Some(Opcode::Swap6),
                    7 => Some(Opcode::Swap7),
                    8 => Some(Opcode::Swap8),
                    9 => Some(Opcode::Swap9),
                    10 => Some(Opcode::Swap10),
                    11 => Some(Opcode::Swap11),
                    12 => Some(Opcode::Swap12),
                    13 => Some(Opcode::Swap13),
                    14 => Some(Opcode::Swap14),
                    15 => Some(Opcode::Swap15),
                    16 => Some(Opcode::Swap16),
                    _ => None,
                }
            }
            0xa0 => Some(Opcode::Log0),
            0xa1 => Some(Opcode::Log1),
            0xa2 => Some(Opcode::Log2),
            0xa3 => Some(Opcode::Log3),
            0xa4 => Some(Opcode::Log4),
            0xf0 => Some(Opcode::Create),
            0xf1 => Some(Opcode::Call),
            0xf2 => Some(Opcode::Callcode),
            0xf3 => Some(Opcode::Return),
            0xf4 => Some(Opcode::Delegatecall),
            0xf5 => Some(Opcode::Create2),
            0xfa => Some(Opcode::Staticcall), //this could be wrong i think its 0xfa
            0xfd => Some(Opcode::Revert),
            0xff => Some(Opcode::Selfdestruct),
            _ => None,
        }
    }

    /// Get the gas cost for this opcode
    pub fn gas_cost(&self) -> Gas {
        match self {
            // Stop and arithmetic operations
            Opcode::Stop => GAS_BASE,
            Opcode::Add | Opcode::Sub | Opcode::Not | Opcode::Lt | Opcode::Gt | Opcode::Slt | Opcode::Sgt | Opcode::Eq | Opcode::Iszero | Opcode::And | Opcode::Or | Opcode::Xor | Opcode::Byte | Opcode::Shl | Opcode::Shr | Opcode::Sar => GAS_VERY_LOW,
            Opcode::Mul | Opcode::Div | Opcode::Sdiv | Opcode::Mod | Opcode::Smod | Opcode::Signextend => GAS_LOW,
            Opcode::Addmod | Opcode::Mulmod | Opcode::Exp => GAS_MID,
            
            // SHA3
            Opcode::Sha3 => GAS_MID,
            
            // Environmental information
            Opcode::Address | Opcode::Origin | Opcode::Caller | Opcode::Callvalue | Opcode::Codesize | Opcode::Gasprice | Opcode::Chainid | Opcode::Selfbalance | Opcode::Basefee => GAS_BASE,
            Opcode::Balance | Opcode::Extcodesize | Opcode::Extcodehash => GAS_EXTCODE,
            Opcode::Calldataload | Opcode::Calldatasize | Opcode::Returndatasize => GAS_VERY_LOW,
            Opcode::Calldatacopy | Opcode::Codecopy | Opcode::Extcodecopy | Opcode::Returndatacopy => GAS_VERY_LOW,
            
            // Block information
            Opcode::Blockhash | Opcode::Coinbase | Opcode::Timestamp | Opcode::Number | Opcode::Difficulty | Opcode::Gaslimit => GAS_BASE,
            
            // Stack, memory, storage and flow operations
            Opcode::Pop | Opcode::Pc | Opcode::Msize | Opcode::Gas | Opcode::Jumpdest => GAS_BASE,
            Opcode::Mload | Opcode::Mstore | Opcode::Mstore8 => GAS_VERY_LOW,
            Opcode::Sload => GAS_SLOAD,
            Opcode::Sstore => GAS_SLOAD, // Will be calculated dynamically
            Opcode::Jump | Opcode::Jumpi => GAS_MID,
            
            // Push operations
            Opcode::Push0 => GAS_BASE,
            Opcode::Push1 | Opcode::Push2 | Opcode::Push3 | Opcode::Push4 | Opcode::Push5 | Opcode::Push6 | Opcode::Push7 | Opcode::Push8 | Opcode::Push9 | Opcode::Push10 | Opcode::Push11 | Opcode::Push12 | Opcode::Push13 | Opcode::Push14 | Opcode::Push15 | Opcode::Push16 | Opcode::Push17 | Opcode::Push18 | Opcode::Push19 | Opcode::Push20 | Opcode::Push21 | Opcode::Push22 | Opcode::Push23 | Opcode::Push24 | Opcode::Push25 | Opcode::Push26 | Opcode::Push27 | Opcode::Push28 | Opcode::Push29 | Opcode::Push30 | Opcode::Push31 | Opcode::Push32 => GAS_VERY_LOW,
            
            // Duplication operations
            Opcode::Dup1 | Opcode::Dup2 | Opcode::Dup3 | Opcode::Dup4 | Opcode::Dup5 | Opcode::Dup6 | Opcode::Dup7 | Opcode::Dup8 | Opcode::Dup9 | Opcode::Dup10 | Opcode::Dup11 | Opcode::Dup12 | Opcode::Dup13 | Opcode::Dup14 | Opcode::Dup15 | Opcode::Dup16 => GAS_VERY_LOW,
            
            // Exchange operations
            Opcode::Swap1 | Opcode::Swap2 | Opcode::Swap3 | Opcode::Swap4 | Opcode::Swap5 | Opcode::Swap6 | Opcode::Swap7 | Opcode::Swap8 | Opcode::Swap9 | Opcode::Swap10 | Opcode::Swap11 | Opcode::Swap12 | Opcode::Swap13 | Opcode::Swap14 | Opcode::Swap15 | Opcode::Swap16 => GAS_VERY_LOW,
            
            // Logging operations
            Opcode::Log0 | Opcode::Log1 | Opcode::Log2 | Opcode::Log3 | Opcode::Log4 => GAS_VERY_LOW, // Will be calculated dynamically
            
            // System operations
            Opcode::Create | Opcode::Call | Opcode::Callcode | Opcode::Delegatecall | Opcode::Create2 | Opcode::Staticcall => GAS_HIGH, // Will be calculated dynamically
            Opcode::Return | Opcode::Revert => GAS_BASE,
            Opcode::Selfdestruct => GAS_BASE,
        }
    }
}

/// Execution context for opcode execution
pub struct ExecutionContext<'a> {
    pub code: &'a [u8],
    pub pc: &'a mut usize,
    pub stack: &'a mut crate::stack::Stack,
    pub memory: &'a mut crate::memory::Memory,
    pub gas_tracker: &'a mut crate::gas::GasTracker,
}

/// Trait for opcode execution
pub trait OpcodeExecutor {
    fn execute(&self, context: &mut ExecutionContext) -> Result<(), EvmError>;
}
