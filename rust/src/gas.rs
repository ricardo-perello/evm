use crate::types::{EvmError, Gas};

/// Gas cost constants for EVM operations
pub const GAS_BASE: Gas = 2;
pub const GAS_VERY_LOW: Gas = 3;
pub const GAS_LOW: Gas = 5;
pub const GAS_MID: Gas = 8;
pub const GAS_HIGH: Gas = 10;
pub const GAS_EXTCODE: Gas = 700;
pub const GAS_BALANCE: Gas = 400;
pub const GAS_SLOAD: Gas = 200;
pub const GAS_JUMPDEST: Gas = 1;
pub const GAS_SSTORE_SET: Gas = 20000;
pub const GAS_SSTORE_RESET: Gas = 5000;
pub const GAS_SSTORE_CLEAR: Gas = 15000;

/// Gas tracker for EVM execution
pub struct GasTracker {
    gas_used: Gas,
    gas_limit: Gas,
    gas_refund: Gas,
}

impl GasTracker {
    pub fn new(gas_limit: Gas) -> Self {
        Self {
            gas_used: 0,
            gas_limit,
            gas_refund: 0,
        }
    }

    /// Consume gas for an operation
    pub fn consume(&mut self, amount: Gas) -> Result<(), EvmError> {
        if self.gas_used + amount > self.gas_limit {
            return Err(EvmError::OutOfGas);
        }
        self.gas_used += amount;
        Ok(())
    }

    /// Get remaining gas
    pub fn remaining(&self) -> Gas {
        self.gas_limit.saturating_sub(self.gas_used)
    }

    /// Get total gas used
    pub fn gas_used(&self) -> Gas {
        self.gas_used
    }

    /// Get gas limit
    pub fn gas_limit(&self) -> Gas {
        self.gas_limit
    }

    /// Check if we have enough gas for an operation
    pub fn has_gas(&self, amount: Gas) -> bool {
        self.remaining() >= amount
    }
}

impl Default for GasTracker {
    fn default() -> Self {
        Self::new(30_000_000) // 30M gas limit
    }
}
