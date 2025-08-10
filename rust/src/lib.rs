use primitive_types::U256;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let mut stack: Vec<U256> = Vec::new();
    let mut pc = 0;

    let code = _code.as_ref();

    while pc < code.len() {
        let opcode = code[pc];
        pc += 1;

        match opcode {
            0x00 => {
                // STOP
            }
            0x5f => {
                // PUSH0
                stack.push(U256::from(0));
            }
            0x60..=0x7f => {
                // PUSH1..PUSH32
                let size = (opcode - 0x60) + 1;
                let size = size as usize; // Cast to usize

                
                // Read 'size' bytes from the code
                let mut value = U256::from(0);
                for i in 0..size {
                    if pc + i < code.len() {
                        value = value << 8 | U256::from(code[pc + i]);
                    }
                }
                
                stack.push(value);
                pc += size; // Skip over the bytes we just read
            }
            _ => {
                return EvmResult {
                    stack: stack,
                    success: false,
                };
            }
        }
    }

    // TODO: Implement me

    return EvmResult {
        stack: stack.into_iter().rev().collect(),
        success: true,
    };
}
