/**
 * EVM From Scratch
 * Rust template
 *
 * To work on EVM From Scratch in Rust:
 *
 * - Install Rust: https://www.rust-lang.org/tools/install
 * - Edit `rust/lib.rs`
 * - Run `cd rust && cargo run` to run the tests
 *
 * Hint: most people who were trying to learn Rust and EVM at the same
 * gave up and switched to JavaScript, Python, or Go. If you are new
 * to Rust, implement EVM in another programming language first.
 */

use evm::types::Block;
use primitive_types::U256;
use serde::Deserialize;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Deserialize)]
struct Evmtest {
    name: String,
    hint: String,
    code: Code,
    expect: Expect,
    block: Option<Block>,
    tx: Option<evm::types::TestTransaction>,
    state: Option<evm::types::TestState>,
}

#[derive(Debug, Deserialize)]
struct Code {
    asm: String,
    bin: String,
}

#[derive(Debug, Deserialize)]
struct Expect {
    stack: Option<Vec<String>>,
    success: bool,
    // #[serde(rename = "return")]
    // ret: Option<String>,
}


fn main() {
    let text = std::fs::read_to_string("../evm.json").unwrap();
    let data: Vec<Evmtest> = serde_json::from_str(&text).unwrap();

    let total = data.len();

    for (index, test) in data.iter().enumerate() {
        println!("Test {} of {}: {}", index + 1, total, test.name);

        let code: Vec<u8> = hex::decode(&test.code.bin).unwrap();

        // Create EVM configuration from test block data
        let mut config = evm::EvmConfig::default();
        
        if let Some(ref block) = test.block {
            // Configure coinbase
            if let Some(ref coinbase_hex) = block.coinbase {
                let coinbase_clean = coinbase_hex.trim_start_matches("0x");
                // Pad odd-length hex strings with leading zero
                let padded_hex = if coinbase_clean.len() % 2 == 1 {
                    format!("0{}", coinbase_clean)
                } else {
                    coinbase_clean.to_string()
                };
                let coinbase_bytes = hex::decode(&padded_hex).unwrap_or_default();
                let mut coinbase = [0u8; 20];
                
                // Place the bytes at the end of the 20-byte array (right-aligned)
                let start_pos = 20 - coinbase_bytes.len();
                for (i, &byte) in coinbase_bytes.iter().enumerate() {
                    coinbase[start_pos + i] = byte;
                }
                config.coinbase = coinbase;
            }
            
            // Configure base fee
            if let Some(ref base_fee_hex) = block.basefee {
                let base_fee_clean = base_fee_hex.trim_start_matches("0x");
                let base_fee = U256::from_str_radix(base_fee_clean, 16).unwrap_or_default();
                config.block_base_fee = base_fee;
            }
            
            // Configure gas limit
            if let Some(ref gas_limit_hex) = block.gaslimit {
                let gas_limit_clean = gas_limit_hex.trim_start_matches("0x");
                let gas_limit = U256::from_str_radix(gas_limit_clean, 16).unwrap_or_default();
                config.block_gas_limit = gas_limit;

            }
            
            // Configure block number
            if let Some(ref number_hex) = block.number {
                let number_clean = number_hex.trim_start_matches("0x");
                let number = u64::from_str_radix(number_clean, 16).unwrap_or_default();
                config.block_number = number;
            }
            
            // Configure timestamp
            if let Some(ref timestamp_hex) = block.timestamp {
                let timestamp_clean = timestamp_hex.trim_start_matches("0x");
                let timestamp = u64::from_str_radix(timestamp_clean, 16).unwrap_or_default();
                config.block_timestamp = timestamp;
            }
            
            // Configure chain id
            if let Some(ref chainid_hex) = block.chainid {
                let chainid_clean = chainid_hex.trim_start_matches("0x");
                let chainid = U256::from_str_radix(chainid_clean, 16).unwrap_or_default();
                config.chain_id = chainid;
            }
            
            // Configure difficulty
            if let Some(ref difficulty_hex) = block.difficulty {
                let difficulty_clean = difficulty_hex.trim_start_matches("0x");
                let difficulty = U256::from_str_radix(difficulty_clean, 16).unwrap_or_default();
                config.block_difficulty = difficulty;
            }
        }

        // Parse test transaction if provided
        if let Some(ref test_tx) = test.tx {
            if let Some(ref to_hex) = test_tx.to {
                let to_clean = to_hex.trim_start_matches("0x");
                let to_bytes = hex::decode(to_clean).unwrap_or_default();
                if to_bytes.len() == 20 {
                    config.transaction.to = to_bytes.try_into().unwrap_or(config.transaction.to);
                    println!("DEBUG: Setting transaction to address to {}", to_hex);
                }
            }
            if let Some(ref value_hex) = test_tx.value {
                let value_clean = value_hex.trim_start_matches("0x");
                let value = U256::from_str_radix(value_clean, 16).unwrap_or_default();
                config.transaction.value = value;
                println!("DEBUG: Setting transaction value to {:#X}", value);
            }
            if let Some(ref data_hex) = test_tx.data {
                let data_clean = data_hex.trim_start_matches("0x");
                let data = hex::decode(data_clean).unwrap_or_default();
                config.transaction.data = data.clone();
                println!("DEBUG: Setting transaction data to {} bytes", data.len());
            }
        }

        // Parse test state if provided
        if let Some(ref test_state) = test.state {
            // Store the state in the config for the EVM to use, wrapped in Rc<RefCell> for shared access
            config.test_state = Some(Rc::new(RefCell::new(test_state.clone())));
        }

        let vm = evm::Evm::new(config);
        let result = vm.execute(code);

        let mut expected_stack: Vec<U256> = Vec::new();
        if let Some(ref stacks) = test.expect.stack {
            for value in stacks {
                expected_stack.push(U256::from_str_radix(value, 16).unwrap());
            }
        }

        let mut matching = result.stack.len() == expected_stack.len();
        if matching {
            for i in 0..result.stack.len() {
                if result.stack[i] != expected_stack[i] {
                    matching = false;
                    break;
                }
            }
        }
        
        matching = matching && result.success == test.expect.success;

        if !matching {
            println!("Instructions: \n{}\n", test.code.asm);

            println!("Expected success: {:?}", test.expect.success);
            println!("Expected stack: [");
            for v in expected_stack {
                println!("  {:#X},", v);
            }
            println!("]\n");
            
            println!("Actual success: {:?}", result.success);
            println!("Actual stack: [");
            for v in result.stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("\nHint: {}\n", test.hint);
            println!("Progress: {}/{}\n\n", index, total);
            panic!("Test failed");
        }
        println!("PASS");
    }
    println!("Congratulations!");
}
