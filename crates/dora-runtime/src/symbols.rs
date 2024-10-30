// Debug functions
pub const DEBUG_PRINT: &str = "dora_debug_print";
// System functions
pub const WRITE_RESULT: &str = "dora_write_result";
pub const EXTEND_MEMORY: &str = "dora_extend_memory";
pub const KECCAK256_HASHER: &str = "dora_keccak256_hasher";
pub const STORAGE_WRITE: &str = "dora_write_storage";
pub const STORAGE_READ: &str = "dora_read_storage";
pub const APPEND_LOG: &str = "dora_append_log";
pub const APPEND_LOG_ONE_TOPIC: &str = "dora_append_log_with_one_topic";
pub const APPEND_LOG_TWO_TOPICS: &str = "dora_append_log_with_two_topics";
pub const APPEND_LOG_THREE_TOPICS: &str = "dora_append_log_with_three_topics";
pub const APPEND_LOG_FOUR_TOPICS: &str = "dora_append_log_with_four_topics";
pub const GET_CALLDATA_PTR: &str = "dora_get_calldata_ptr";
pub const GET_CALLDATA_SIZE: &str = "dora_get_calldata_size";
pub const GET_CODESIZE_FROM_ADDRESS: &str = "dora_get_codesize_from_address";
pub const COPY_CODE_TO_MEMORY: &str = "dora_copy_code_to_memory";
pub const GET_ADDRESS_PTR: &str = "dora_get_address_ptr";
pub const GET_GASLIMIT: &str = "dora_get_gaslimit";
pub const STORE_IN_CALLVALUE_PTR: &str = "dora_store_in_callvalue_ptr";
pub const STORE_IN_BLOBBASEFEE_PTR: &str = "dora_store_in_blobbasefee_ptr";
pub const GET_BLOB_HASH_AT_INDEX: &str = "dora_get_blob_hash_at_index";
pub const STORE_IN_BALANCE: &str = "dora_store_in_balance";
pub const GET_COINBASE_PTR: &str = "dora_get_coinbase_ptr";
pub const STORE_IN_TIMESTAMP_PTR: &str = "dora_store_in_timestamp_ptr";
pub const STORE_IN_BASEFEE_PTR: &str = "dora_store_in_basefee_ptr";
pub const STORE_IN_CALLER_PTR: &str = "dora_store_in_caller_ptr";
pub const GET_ORIGIN: &str = "dora_get_origin";
pub const GET_CHAINID: &str = "dora_get_chainid";
pub const STORE_IN_GASPRICE_PTR: &str = "dora_store_in_gasprice_ptr";
pub const GET_BLOCK_NUMBER: &str = "dora_get_block_number";
pub const STORE_IN_SELFBALANCE_PTR: &str = "dora_store_in_selfbalance_ptr";
pub const COPY_EXT_CODE_TO_MEMORY: &str = "dora_copy_ext_code_to_memory";
pub const GET_PREVRANDAO: &str = "dora_get_prevrandao";
pub const GET_BLOCK_HASH: &str = "dora_get_block_hash";
pub const GET_CODE_HASH: &str = "dora_get_code_hash";
pub const CALL: &str = "dora_call";
pub const CREATE: &str = "dora_create";
pub const CREATE2: &str = "dora_create2";
pub const GET_RETURN_DATA_SIZE: &str = "dora_get_return_data_size";
pub const COPY_RETURN_DATA_INTO_MEMORY: &str = "dora_copy_return_data_into_memory";
pub const TRANSIENT_STORAGE_READ: &str = "dora_transient_storage_read";
pub const TRANSIENT_STORAGE_WRITE: &str = "dora_transient_storage_write";
pub const SELFDESTRUCT: &str = "dora_selfdestruct";
