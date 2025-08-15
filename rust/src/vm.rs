use crate::types::{EvmError, EvmConfig, EvmResult, Address, Word};
use crate::state::EvmState;
use crate::Gas;
use primitive_types::U256;

/// Main EVM virtual machine
pub struct Evm {
    config: EvmConfig,
}

impl Evm {
    pub fn new(config: EvmConfig) -> Self {
        Self { config }
    }

    /// Execute EVM bytecode
    pub fn execute(&self, code: Vec<u8>) -> EvmResult {
        let mut state = EvmState::new(code, self.config.clone()); //todo could be a problem here
        
        // Execute until halted or error
        while state.status() == crate::state::ExecutionStatus::Running {
            if let Err(_) = state.step() {
                // On error, execution stops and returns failure
                state.reverted = true;
                break;
            }
        }
        
        state.result()
    }

    /// Get the current configuration
    pub fn config(&self) -> &EvmConfig {
        &self.config
    }

    /// Update the configuration
    pub fn update_config(&mut self, config: EvmConfig) {
        self.config = config;
    }
}

impl Default for Evm {
    fn default() -> Self {
        Self::new(EvmConfig::default())
    }
}

/// Builder pattern for EVM configuration
pub struct EvmBuilder {
    config: EvmConfig,
}

impl EvmBuilder {
    pub fn new() -> Self {
        Self {
            config: EvmConfig::default(),
        }
    }

    pub fn gas_limit(mut self, gas_limit: Gas) -> Self {
        self.config.gas_limit = gas_limit;
        self
    }

    pub fn block_number(mut self, block_number: u64) -> Self {
        self.config.block_number = block_number;
        self
    }

    pub fn block_timestamp(mut self, block_timestamp: u64) -> Self {
        self.config.block_timestamp = block_timestamp;
        self
    }

    pub fn block_difficulty(mut self, block_difficulty: Word) -> Self {
        self.config.block_difficulty = block_difficulty;
        self
    }

    pub fn block_gas_limit(mut self, block_gas_limit: Gas) -> Self {
        self.config.block_gas_limit = U256::from(block_gas_limit);
        self
    }

    pub fn block_base_fee(mut self, block_base_fee: Word) -> Self {
        self.config.block_base_fee = block_base_fee;
        self
    }

    pub fn build(self) -> Evm {
        Evm::new(self.config)
    }
}

impl Default for EvmBuilder {
    fn default() -> Self {
        Self::new()
    }
}
