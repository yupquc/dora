use thiserror::Error;

pub const MAX_STACK_SIZE: usize = 1024;
pub const CALL_STACK_LIMIT: usize = 1024;

/// MLIR main entry point name.
pub const MAIN_ENTRYPOINT: &str = "main";
/// Versioning and blob constants
pub const VERSIONED_HASH_VERSION_KZG: u8 = 0x01;
/// Target number of the blob per block.
pub const TARGET_BLOB_NUMBER_PER_BLOCK: u64 = 3;
/// Max number of blobs per block
pub const MAX_BLOB_NUMBER_PER_BLOCK: u64 = 2 * TARGET_BLOB_NUMBER_PER_BLOCK;
/// Number of block hashes that the VM can access in the past (pre-Prague).
pub const BLOCK_HASH_HISTORY: u64 = 256;

pub mod env {
    pub const DORA_TRACING: &str = "DORA_TRACING";
    pub const DORA_DISABLE_CONSOLE: &str = "DORA_DISABLE_CONSOLE";
}

pub mod gas_cost {
    // Gas costs for various operations
    pub const ADD: i64 = 3;
    pub const MUL: i64 = 5;
    pub const SUB: i64 = 3;
    pub const DIV: i64 = 5;
    pub const SDIV: i64 = 5;
    pub const MOD: i64 = 5;
    pub const SMOD: i64 = 5;
    pub const ADDMOD: i64 = 8;
    pub const MULMOD: i64 = 8;
    pub const EXP: i64 = 10;
    pub const SIGNEXTEND: i64 = 5;
    pub const LT: i64 = 3;
    pub const GT: i64 = 3;
    pub const SLT: i64 = 3;
    pub const SGT: i64 = 3;
    pub const EQ: i64 = 3;
    pub const ISZERO: i64 = 3;
    pub const AND: i64 = 3;
    pub const OR: i64 = 3;
    pub const XOR: i64 = 3;
    pub const NOT: i64 = 3;
    pub const BYTE: i64 = 3;
    pub const SHL: i64 = 3;
    pub const SHR: i64 = 3;
    pub const SAR: i64 = 3;

    pub const BALANCE: i64 = 100;
    pub const ORIGIN: i64 = 2;
    pub const CALLER: i64 = 2;
    pub const CALLDATALOAD: i64 = 3;
    pub const CALLDATASIZE: i64 = 2;
    pub const CALLDATACOPY: i64 = 3;
    pub const CODESIZE: i64 = 2;
    pub const COINBASE: i64 = 2;
    pub const GASPRICE: i64 = 2;
    pub const SELFBALANCE: i64 = 5;
    pub const NUMBER: i64 = 2;
    pub const PREVRANDAO: i64 = 2;
    pub const BLOBBASEFEE: i64 = 2;
    pub const CHAINID: i64 = 2;
    pub const BASEFEE: i64 = 2;
    pub const BLOBHASH: i64 = 3;

    pub const POP: i64 = 2;
    pub const MLOAD: i64 = 3;
    pub const MSTORE: i64 = 3;
    pub const MSTORE8: i64 = 3;
    pub const SLOAD: i64 = 100; // assuming the key is warm for now
    pub const SSTORE: i64 = 100;

    pub const JUMP: i64 = 8;
    pub const JUMPI: i64 = 10;
    pub const PC: i64 = 2;
    pub const MSIZE: i64 = 2;
    pub const GAS: i64 = 2;
    pub const JUMPDEST: i64 = 1;

    pub const MCOPY: i64 = 3;
    pub const PUSH0: i64 = 2;
    pub const PUSHN: i64 = 3;
    pub const DUPN: i64 = 3;
    pub const SWAPN: i64 = 3;

    pub const TIMESTAMP: i64 = 2;
    pub const KECCAK256: i64 = 30;
    pub const CODECOPY: i64 = 3;
    pub const KECCAK256_WORD_COST: u64 = 6;
    pub const COPY_WORD_COST: u64 = 3;

    pub const CALLVALUE: u64 = 9000;
    pub const NEWACCOUNT: u64 = 25000;

    // Logging
    pub const LOG0: i64 = 375;
    pub const LOG1: i64 = 750;
    pub const LOG2: i64 = 1125;
    pub const LOG3: i64 = 1500;
    pub const LOG4: i64 = 1875;

    pub const BLOCKHASH: i64 = 20;
    pub const CODEDEPOSIT: u64 = 200;

    // Call and other operations
    pub const CALL: i64 = 100;
    pub const RETURN: i64 = 0;
    pub const REVERT: i64 = 0;
    pub const STATICCALL: i64 = 100;

    pub const EXTCODESIZE_WARM: i64 = 100;
    pub const EXTCODECOPY_WARM: i64 = 100;

    pub const RETURNDATASIZE: i64 = 2;
    pub const RETURNDATACOPY: i64 = 3;
    pub const EXTCODEHASH: i64 = 100;

    pub const ADDRESS: i64 = 2;
    pub const GASLIMIT: i64 = 2;

    pub const SSTORE_MIN_REMAINING_GAS: i64 = 2_300;
    pub const CREATE: i64 = 32_000;

    pub const TLOAD: i64 = 100;
    pub const TSTORE: i64 = 100;

    pub const SELFDESTRUCT: i64 = 5_000;
    pub const SELFDESTRUCT_DYNAMIC_GAS: i64 = 25_000;

    pub const MIN_BLOB_GASPRICE: u64 = 1;
    pub const BLOB_GASPRICE_UPDATE_FRACTION: u64 = 3_338_477;

    pub const BYTE_DEPOSIT_COST: i64 = 200;
    /// EIP-3860 : Limit and meter initcode
    pub const INITCODE_WORD_COST: u64 = 2;
    pub const HASH_WORD_COST: i64 = 6;

    // Transaction costs
    pub const TX_BASE_COST: u64 = 21_000;
    pub const TX_DATA_COST_PER_NON_ZERO: u64 = 16;
    pub const TX_DATA_COST_PER_ZERO: u64 = 4;
    pub const TX_CREATE_COST: u64 = 32_000;
    pub const TX_ACCESS_LIST_ADDRESS_COST: u64 = 2_400;
    pub const TX_ACCESS_LIST_STORAGE_KEY_COST: u64 = 1_900;

    /// EIP-170: Contract code size limit
    /// By default the limit is `0x6000` (~25kb)
    pub const MAX_CODE_SIZE: usize = 0x6000;
    pub const MAX_INITCODE_SIZE: usize = 2 * MAX_CODE_SIZE;

    /// EIP-1884: Repricing for trie-size-dependent opcodes
    pub const INSTANBUL_SLOAD_GAS: u64 = 800;
    pub const SSTORE_SET: u64 = 20000;
    pub const SSTORE_RESET: u64 = 5000;
    pub const REFUND_SSTORE_CLEARS: i64 = 15000;

    pub const TRANSACTION_ZERO_DATA: u64 = 4;
    pub const TRANSACTION_NON_ZERO_DATA_INIT: u64 = 16;
    pub const TRANSACTION_NON_ZERO_DATA_FRONTIER: u64 = 68;

    pub const ACCESS_LIST_ADDRESS: u64 = 2400;
    pub const ACCESS_LIST_STORAGE_KEY: u64 = 1900;
    pub const COLD_SLOAD_COST: u64 = 2100;
    pub const COLD_ACCOUNT_ACCESS_COST: u64 = 2600;
    pub const WARM_SLOAD_COST: u64 = 100;
    pub const WARM_SSTORE_RESET: u64 = SSTORE_RESET - COLD_SLOAD_COST;

    pub const CALL_STIPEND: u64 = 2300;
    pub const MIN_CALLEE_GAS: u64 = CALL_STIPEND;

    /// Calculates the gas cost for initializing a contract based on the length of the initialization code.
    ///
    /// The cost is computed by multiplying the length of the code, divided into 32-byte words, by a fixed word cost.
    ///
    /// # Parameters
    ///
    /// - `init_code_length`: The length of the initialization code in bytes.
    ///
    /// # Returns
    ///
    /// - `u64`: The gas cost for initializing the code.
    ///
    /// # Example
    ///
    /// ```no_check
    /// let cost = gas_cost::init_code_cost(512);
    /// ```
    #[inline]
    pub fn init_code_cost(init_code_length: usize) -> u64 {
        INITCODE_WORD_COST * ((init_code_length as u64 + 31) / 32)
    }

    /// Calculates the gas cost for expanding memory from a previous size to a new size.
    ///
    /// The cost is calculated based on the quadratic increase in memory usage, reflecting the increased cost
    /// of handling larger memory segments.
    ///
    /// # Parameters
    ///
    /// - `last_size`: The previous size of the memory (in bytes).
    /// - `new_size`: The new size of the memory (in bytes).
    ///
    /// # Returns
    ///
    /// - `i64`: The gas cost required to expand the memory from `last_size` to `new_size`.
    ///
    /// # Example
    ///
    /// ```no_check
    /// let cost = gas_cost::memory_expansion_cost(1024, 2048);
    /// ```
    pub fn memory_expansion_cost(last_size: u32, new_size: u32) -> i64 {
        let new_memory_size_word = (new_size + 31) / 32;
        let new_memory_cost =
            (new_memory_size_word * new_memory_size_word) / 512 + (3 * new_memory_size_word);

        let last_memory_size_word = (last_size + 31) / 32;
        let last_memory_cost =
            (last_memory_size_word * last_memory_size_word) / 512 + (3 * last_memory_size_word);

        (new_memory_cost - last_memory_cost).into()
    }

    /// Calculates the gas cost for copying memory based on the size of the memory segment.
    ///
    /// The cost is proportional to the number of 32-byte words being copied, with a fixed cost per word.
    ///
    /// # Parameters
    ///
    /// - `size`: The size of the memory segment to be copied (in bytes).
    ///
    /// # Returns
    ///
    /// - `i64`: The gas cost for copying the specified memory segment.
    ///
    /// # Example
    ///
    /// ```no_check
    /// let cost = gas_cost::memory_copy_cost(256);
    /// ```
    pub fn memory_copy_cost(size: u32) -> i64 {
        let memory_word_size = (size + 31) / 32;
        (memory_word_size * 3).into()
    }

    /// Calculates the dynamic gas cost for a `LOG` operation based on the size of the log data and the number of topics.
    ///
    /// The cost is the sum of the base gas cost per topic and the gas cost per byte of log data.
    ///
    /// # Parameters
    ///
    /// - `size`: The size of the log data (in bytes).
    /// - `topic_count`: The number of topics associated with the log.
    ///
    /// # Returns
    ///
    /// - `i64`: The dynamic gas cost for the `LOG` operation.
    ///
    /// # Example
    ///
    /// ```no_check
    /// let cost = gas_cost::log_dynamic_gas_cost(128, 2);
    /// ```
    pub fn log_dynamic_gas_cost(size: u32, topic_count: u32) -> i64 {
        (LOG0 * topic_count as i64) + (8 * size as i64)
    }

    /// Calculates the dynamic gas cost for the `EXP` operation based on the size of the exponent.
    ///
    /// The gas cost is computed based on the number of bytes required to represent the exponent, with
    /// a fixed base cost and an additional cost per byte.
    ///
    /// # Parameters
    ///
    /// - `exponent`: The value of the exponent for the `EXP` operation.
    ///
    /// # Returns
    ///
    /// - `i64`: The dynamic gas cost for the `EXP` operation.
    ///
    /// # Example
    ///
    /// ```no_check
    /// let cost = gas_cost::exp_dynamic_cost(1024);
    /// ```
    pub fn exp_dynamic_cost(exponent: u64) -> i64 {
        let exponent_byte_size = (((64 - exponent.leading_zeros()) + 7) / 8) as i64;
        10 + 50 * exponent_byte_size
    }
}

pub mod call_opcode {
    // Return codes
    pub const SUCCESS_RETURN_CODE: u8 = 1;
    pub const REVERT_RETURN_CODE: u8 = 0;

    // Gas related constants
    pub const WARM_MEMORY_ACCESS_COST: u64 = 100;
    pub const NOT_ZERO_VALUE_COST: u64 = 9000;
    pub const EMPTY_CALLEE_COST: u64 = 25_000;
    pub const STIPEND_GAS_ADDITION: u64 = 2_300;
    pub const GAS_CAP_DIVISION_FACTOR: u64 = 64;
}

/// The type of a `*CALL*` instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CallType {
    /// `CALL`.
    Call,
    /// `STATICCALL`.
    Staticcall,
    /// `DELEGATECALL`.
    Delegatecall,
    /// `CALLCODE`.
    Callcode,
}

/// The kind of a `EXT*CALL` instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ExtCallType {
    /// `EXTCALL`.
    Call,
    /// `EXTDELEGATECALL`.
    Delegatecall,
    /// `EXTSTATICCALL`.
    Staticcall,
}

#[derive(Error, Debug)]
#[error("Couldn't parse CallType from u8")]
pub struct CallTypeParseError;

impl TryFrom<u8> for CallType {
    type Error = CallTypeParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == CallType::Call as u8 => Ok(CallType::Call),
            x if x == CallType::Staticcall as u8 => Ok(CallType::Staticcall),
            x if x == CallType::Delegatecall as u8 => Ok(CallType::Delegatecall),
            x if x == CallType::Callcode as u8 => Ok(CallType::Callcode),
            _ => Err(CallTypeParseError),
        }
    }
}

impl TryFrom<u8> for ExtCallType {
    type Error = CallTypeParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == ExtCallType::Call as u8 => Ok(ExtCallType::Call),
            x if x == ExtCallType::Staticcall as u8 => Ok(ExtCallType::Staticcall),
            x if x == ExtCallType::Delegatecall as u8 => Ok(ExtCallType::Delegatecall),
            _ => Err(CallTypeParseError),
        }
    }
}

// Float truncation constants
pub const LEF32_GEQ_I32_MIN: u64 = i32::MIN as u64;
pub const GEF32_LEQ_I32_MAX: u64 = 0x4eff_ffff; // Max f32 bits for i32
pub const LEF64_GEQ_I32_MIN: u64 = i32::MIN as u64;
pub const GEF64_LEQ_I32_MAX: u64 = i32::MAX as u64;
pub const LEF32_GEQ_U32_MIN: u64 = u32::MIN as u64;
pub const GEF32_LEQ_U32_MAX: u64 = 0x4f7f_ffff; // Max f32 bits for u32
pub const LEF64_GEQ_U32_MIN: u64 = u32::MIN as u64;
pub const GEF64_LEQ_U32_MAX: u64 = u32::MAX as u64;
pub const LEF32_GEQ_I64_MIN: u64 = i64::MIN as u64;
pub const GEF32_LEQ_I64_MAX: u64 = 0x5eff_ffff; // Max f32 bits for i64
pub const LEF64_GEQ_I64_MIN: u64 = i64::MIN as u64;
pub const GEF64_LEQ_I64_MAX: u64 = i64::MAX as u64;
pub const LEF32_GEQ_U64_MIN: u64 = u64::MIN;
pub const GEF32_LEQ_U64_MAX: u64 = 0x5f7f_ffff; // Max f32 bits for u64
pub const LEF64_GEQ_U64_MIN: u64 = u64::MIN;
pub const GEF64_LEQ_U64_MAX: u64 = u64::MAX;
