use crate::errors::Result;
use dora_ir::IRTypes;
use num_bigint::BigUint;

type U256 = BigUint;

/// Integer comparison condition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntCC {
    /// `==`.
    Equal,
    /// `!=`.
    NotEqual,
    /// Signed `<`.
    SignedLessThan,
    /// Signed `>=`.
    SignedGreaterThanOrEqual,
    /// Signed `>`.
    SignedGreaterThan,
    /// Signed `<=`.
    SignedLessThanOrEqual,
    /// Unsigned `<`.
    UnsignedLessThan,
    /// Unsigned `>=`.
    UnsignedGreaterThanOrEqual,
    /// Unsigned `>`.
    UnsignedGreaterThan,
    /// Unsigned `<=`.
    UnsignedLessThanOrEqual,
}

/// Linkage type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Linkage {
    /// Defined outside of the module.
    Import,
    /// Defined in the module and visible outside.
    Public,
    /// Defined in the module, but not visible outside.
    Private,
}

/// `TypeMethods` defines methods related to types within the intermediate representation (IR).
/// These methods allow for the creation and manipulation of basic IR types such as pointers and integers.
pub trait TypeMethods: IRTypes {
    /// Returns a pointer type in the IR.
    ///
    /// # Returns
    /// - `Self::Type`: The type representing a pointer.
    fn type_ptr(&self) -> Self::Type;

    /// Returns an integer type in the IR with a specified number of bits.
    ///
    /// # Arguments
    /// - `bits`: The number of bits for the integer type (e.g., 32 for i32, 64 for i64).
    ///
    /// # Returns
    /// - `Self::Type`: The type representing an integer with the specified bit width.
    fn type_int(&self, bits: u32) -> Self::Type;
}

/// `Builder` defines methods for constructing and manipulating values and control flow in the IR.
/// This trait extends `TypeMethods` for handling type creation and implements various operations
/// such as constants, stack manipulation, memory operations, branching, and arithmetic.
pub trait Builder: IRTypes + TypeMethods {
    /// Creates a boolean constant.
    ///
    /// # Arguments
    /// - `value`: A boolean value (`true` or `false`).
    ///
    /// # Returns
    /// - `Result<Self::Value>`: A value representing the boolean constant.
    fn bool_const(&mut self, value: bool) -> Result<Self::Value>;

    /// Creates a signed integer constant with sign extension.
    ///
    /// # Arguments
    /// - `ty`: The target type to sign-extend to.
    /// - `value`: The signed integer value.
    ///
    /// # Returns
    /// - `Result<Self::Value>`: A value representing the constant.
    fn iconst(&mut self, ty: Self::Type, value: i64) -> Result<Self::Value>;

    /// Creates an unsigned integer constant.
    ///
    /// # Arguments
    /// - `ty`: The target type for the constant.
    /// - `value`: The unsigned integer value.
    ///
    /// # Returns
    /// - `Result<Self::Value>`: A value representing the constant.
    fn uconst(&mut self, ty: Self::Type, value: u64) -> Result<Self::Value>;

    /// Creates a 32-bit signed integer constant.
    ///
    /// # Arguments
    /// - `value`: The signed integer value.
    ///
    /// # Returns
    /// - `Result<Self::Value>`: A value representing the constant.
    fn iconst_32(&mut self, value: i32) -> Result<Self::Value>;

    /// Creates a 64-bit signed integer constant.
    ///
    /// # Arguments
    /// - `value`: The signed integer value.
    ///
    /// # Returns
    /// - `Result<Self::Value>`: A value representing the constant.
    fn iconst_64(&mut self, value: i64) -> Result<Self::Value>;

    /// Creates a 256-bit signed integer constant.
    ///
    /// # Arguments
    /// - `value`: The 256-bit integer value (using `U256`).
    ///
    /// # Returns
    /// - `Result<Self::Value>`: A value representing the constant.
    fn iconst_256(&mut self, value: U256) -> Result<Self::Value>;

    /// Creates a floating-point constant.
    ///
    /// # Arguments
    /// - `ty`: The floating-point type (e.g., f32 or f64).
    /// - `value`: The floating-point value.
    ///
    /// # Returns
    /// - `Result<Self::Value>`: A value representing the constant.
    fn fconst(&mut self, ty: Self::Type, value: f64) -> Result<Self::Value>;

    /// Creates a 32-bit floating-point constant.
    ///
    /// # Arguments
    /// - `value`: The floating-point value.
    ///
    /// # Returns
    /// - `Result<Self::Value>`: A value representing the constant.
    fn fconst_32(&mut self, value: f32) -> Result<Self::Value>;

    /// Creates a 64-bit floating-point constant.
    ///
    /// # Arguments
    /// - `value`: The floating-point value.
    ///
    /// # Returns
    /// - `Result<Self::Value>`: A value representing the constant.
    fn fconst_64(&mut self, value: f64) -> Result<Self::Value>;

    /// Pushes a value onto the stack.
    ///
    /// # Arguments
    /// - `value`: The value to push onto the stack.
    ///
    /// # Returns
    /// - `Result<()>`: Indicates whether the operation succeeded.
    fn stack_push(&mut self, value: Self::Value) -> Result<()>;

    /// Pops a value off the stack.
    ///
    /// # Returns
    /// - `Result<Self::Value>`: The popped value from the stack.
    fn stack_pop(&mut self) -> Result<Self::Value>;

    /// Peeks at the top value on the stack without popping it.
    ///
    /// # Returns
    /// - `Result<Self::Value>`: The top value on the stack.
    fn stack_peek(&mut self) -> Result<Self::Value>;

    /// Peeks at the nth value on the stack without popping it.
    ///
    /// # Arguments
    /// - `n`: The index of the value to peek at.
    ///
    /// # Returns
    /// - `Result<Self::Value>`: The nth value on the stack.
    fn stack_peek_nth(&mut self, n: usize) -> Result<Self::Value>;

    /// Exchanges the values at indices `n` and `m` on the stack.
    ///
    /// # Arguments
    /// - `n`: The first index.
    /// - `m`: The second index.
    ///
    /// # Returns
    /// - `Result<()>`: Indicates whether the operation succeeded.
    fn stack_exchange(&mut self, n: usize, m: usize) -> Result<()>;

    /// Loads a value from memory at a specific pointer.
    ///
    /// # Arguments
    /// - `ty`: The type of the value being loaded.
    /// - `ptr`: The pointer from which to load the value.
    ///
    /// # Returns
    /// - `Result<Self::Value>`: The loaded value.
    fn load(&mut self, ty: Self::Type, ptr: Self::Value) -> Result<Self::Value>;

    /// Stores a value into memory at a specific pointer.
    ///
    /// # Arguments
    /// - `value`: The value to store.
    /// - `ptr`: The pointer where the value will be stored.
    fn store(&mut self, value: Self::Value, ptr: Self::Value);

    /// Performs a no-op operation.
    fn nop(&mut self);

    /// Returns from the function with the provided values.
    ///
    /// # Arguments
    /// - `values`: A slice of values to return.
    fn ret(&mut self, values: &[Self::Value]);

    /// Performs an integer comparison between two values.
    ///
    /// # Arguments
    /// - `cond`: The condition to check (e.g., equality, greater than).
    /// - `lhs`: The left-hand side value.
    /// - `rhs`: The right-hand side value.
    ///
    /// # Returns
    /// - `Result<Self::Value>`: The result of the comparison.
    fn icmp(&mut self, cond: IntCC, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Performs an integer comparison with an immediate value.
    ///
    /// # Arguments
    /// - `cond`: The condition to check (e.g., equality, greater than).
    /// - `lhs`: The left-hand side value.
    /// - `rhs`: The right-hand side immediate value.
    ///
    /// # Returns
    /// - `Result<Self::Value>`: The result of the comparison.
    fn icmp_imm(&mut self, cond: IntCC, lhs: Self::Value, rhs: i64) -> Result<Self::Value>;

    /// Checks whether a pointer is null.
    ///
    /// # Arguments
    /// - `ptr`: The pointer value to check.
    ///
    /// # Returns
    /// - `Result<Self::Value>`: A boolean value indicating whether the pointer is null.
    fn is_null(&mut self, ptr: Self::Value) -> Result<Self::Value>;

    /// Checks whether a pointer is not null.
    ///
    /// # Arguments
    /// - `ptr`: The pointer value to check.
    ///
    /// # Returns
    /// - `Result<Self::Value>`: A boolean value indicating whether the pointer is not null.
    fn is_not_null(&mut self, ptr: Self::Value) -> Result<Self::Value>;

    /// Unconditionally branch to the specified destination block.
    ///
    /// # Arguments
    ///
    /// * `dest` - The basic block to branch to.
    fn br(&mut self, dest: Self::BasicBlock);

    /// Conditionally branch to either the `then_block` or the `else_block` based on the value of `cond`.
    ///
    /// # Arguments
    ///
    /// * `cond` - The condition to evaluate.
    /// * `then_block` - The basic block to branch to if the condition is true.
    /// * `else_block` - The basic block to branch to if the condition is false.
    /// * `then_values` - The values to pass to `then_block`.
    /// * `else_values` - The values to pass to `else_block`.
    fn brif(
        &mut self,
        cond: Self::Value,
        then_block: Self::BasicBlock,
        else_block: Self::BasicBlock,
        then_values: &[Self::Value],
        else_values: &[Self::Value],
    );

    /// Conditionally branch to either `then_block` or `else_block` based on `cond`, with an option for cold blocks.
    ///
    /// This is an optimization hint where cold blocks are less likely to be executed.
    ///
    /// # Arguments
    ///
    /// * `cond` - The condition to evaluate.
    /// * `then_block` - The basic block to branch to if the condition is true.
    /// * `else_block` - The basic block to branch to if the condition is false.
    /// * `then_is_cold` - Whether the `then_block` is cold (unlikely to be executed).
    fn brif_cold(
        &mut self,
        cond: Self::Value,
        then_block: Self::BasicBlock,
        else_block: Self::BasicBlock,
        then_is_cold: bool,
    ) {
        let _ = then_is_cold;
        self.brif(cond, then_block, else_block, &[], &[]);
    }

    /// Create a `switch` statement that jumps to different blocks based on `index`.
    ///
    /// # Arguments
    ///
    /// * `default` - The default basic block if no cases match.
    /// * `targets` - Pairs of target values and basic blocks to jump to.
    /// * `flag` - A flag value that affects the switch.
    /// * `flag_type` - The type of the flag.
    /// * `default_is_cold` - Indicates if the default case is cold.
    fn switch(
        &mut self,
        default: Self::BasicBlock,
        targets: &[(u64, Self::BasicBlock)],
        flag: Self::Value,
        flag_type: Self::Type,
        default_is_cold: bool,
    ) -> Result<()>;

    /// Indirect branch to the `successor` basic block.
    ///
    /// # Arguments
    ///
    /// * `successor` - The basic block to branch to indirectly.
    fn br_indirect(&mut self, successor: Self::BasicBlock);

    /// Create a select operation, which chooses between two values based on a condition.
    ///
    /// # Arguments
    ///
    /// * `cond` - The condition to evaluate.
    /// * `then_value` - The value to choose if the condition is true.
    /// * `else_value` - The value to choose if the condition is false.
    fn select(
        &mut self,
        cond: Self::Value,
        then_value: Self::Value,
        else_value: Self::Value,
    ) -> Result<Self::Value>;

    /// Perform an integer addition of `lhs` and `rhs`.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The right-hand side operand.
    fn iadd(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Perform an integer subtraction of `lhs` and `rhs`.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The right-hand side operand.
    fn isub(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Perform an integer multiplication of `lhs` and `rhs`.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The right-hand side operand.
    fn imul(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Perform an unsigned integer division of `lhs` by `rhs`.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The dividend.
    /// * `rhs` - The divisor.
    fn udiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Perform a signed integer division of `lhs` by `rhs`.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The dividend.
    /// * `rhs` - The divisor.
    fn sdiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Perform an unsigned modulo operation (`lhs % rhs`).
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The right-hand side operand.
    fn umod(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Perform a signed modulo operation (`lhs % rhs`).
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The right-hand side operand.
    fn smod(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Perform an addition modulo `modulus`.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The first operand.
    /// * `rhs` - The second operand.
    /// * `modulus` - The modulus.
    fn addmod(
        &mut self,
        lhs: Self::Value,
        rhs: Self::Value,
        modulus: Self::Value,
    ) -> Result<Self::Value>;

    /// Perform a multiplication modulo `modulus`.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The first operand.
    /// * `rhs` - The second operand.
    /// * `modulus` - The modulus.
    fn mulmod(
        &mut self,
        lhs: Self::Value,
        rhs: Self::Value,
        modulus: Self::Value,
    ) -> Result<Self::Value>;

    /// Perform exponentiation of `base` raised to the power of `exponent`.
    ///
    /// # Arguments
    ///
    /// * `base` - The base value.
    /// * `exponent` - The exponent value.
    fn exp(&mut self, base: Self::Value, exponent: Self::Value) -> Result<Self::Value>;

    /// Perform a sign extension of `value`, extending from the specified `byte`.
    ///
    /// # Arguments
    ///
    /// * `byte` - The byte to extend from.
    /// * `value` - The value to extend.
    fn signextend(&mut self, byte: Self::Value, value: Self::Value) -> Result<Self::Value>;

    /// Compare if `lhs` is less than `rhs` (unsigned).
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The right-hand side operand.
    fn lt(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Compare if `lhs` is greater than `rhs` (unsigned).
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The right-hand side operand.
    fn gt(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Compare if `lhs` is less than `rhs` (signed).
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The right-hand side operand.
    fn slt(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Compare if `lhs` is greater than `rhs` (signed).
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The right-hand side operand.
    fn sgt(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Compare if `lhs` is equal to `rhs`.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The right-hand side operand.
    fn eq(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Check if a value is zero.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check.
    fn iszero(&mut self, value: Self::Value) -> Result<Self::Value>;

    /// Perform a bitwise AND operation on `lhs` and `rhs`.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The right-hand side operand.
    fn and(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Perform a bitwise OR operation on `lhs` and `rhs`.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The right-hand side operand.
    fn or(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Perform a bitwise XOR operation on `lhs` and `rhs`.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The right-hand side operand.
    fn xor(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Perform a bitwise NOT operation on `value`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to negate.
    fn not(&mut self, value: Self::Value) -> Result<Self::Value>;

    /// Extract a byte from `value` at the given `index`.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the byte.
    /// * `value` - The value to extract from.
    fn byte(&mut self, index: Self::Value, value: Self::Value) -> Result<Self::Value>;

    /// Perform a left shift on `value` by `shift`.
    ///
    /// # Arguments
    ///
    /// * `shift` - The amount to shift.
    /// * `value` - The value to shift.
    fn shl(&mut self, shift: Self::Value, value: Self::Value) -> Result<Self::Value>;

    /// Perform a logical right shift on `value` by `shift`.
    ///
    /// # Arguments
    ///
    /// * `shift` - The amount to shift.
    /// * `value` - The value to shift.
    fn shr(&mut self, shift: Self::Value, value: Self::Value) -> Result<Self::Value>;

    /// Perform an arithmetic right shift on `value` by `shift`.
    ///
    /// # Arguments
    ///
    /// * `shift` - The amount to shift.
    /// * `value` - The value to shift.
    fn sar(&mut self, shift: Self::Value, value: Self::Value) -> Result<Self::Value>;

    /// Perform an unsigned remainder operation (`lhs % rhs`).
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The right-hand side operand.
    fn urem(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Perform a signed remainder operation (`lhs % rhs`).
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The right-hand side operand.
    fn srem(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value>;

    /// Perform an integer addition of `lhs` and an immediate value `rhs`.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The immediate value.
    fn iadd_imm(&mut self, lhs: Self::Value, rhs: i64) -> Result<Self::Value>;

    /// Perform an integer subtraction of `lhs` and an immediate value `rhs`.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The immediate value.
    fn isub_imm(&mut self, lhs: Self::Value, rhs: i64) -> Result<Self::Value>;

    /// Perform an integer multiplication of `lhs` and an immediate value `rhs`.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The immediate value.
    fn imul_imm(&mut self, lhs: Self::Value, rhs: i64) -> Result<Self::Value>;

    /// Truncate or reduce the integer value.
    ///
    /// # Arguments
    ///
    /// * `to` - The target type to reduce to.
    /// * `value` - The value to reduce.
    #[doc(alias = "trunc")]
    fn ireduce(&mut self, to: Self::Type, value: Self::Value) -> Result<Self::Value>;

    /// Perform a get element pointer (GEP) operation, calculating the address of an element.
    ///
    /// # Arguments
    ///
    /// * `ptr` - The base pointer.
    /// * `indices` - The indices to offset the pointer.
    /// * `element_ty` - The element type.
    /// * `result_ty` - The result type (calculated pointer type).
    fn gep(
        &mut self,
        ptr: Self::Value,
        indices: &[i32],
        element_ty: Self::Type,
        result_ty: Self::Type,
    ) -> Result<Self::Value>;

    /// Perform a memory copy from `src` to `dst` for `len` bytes.
    ///
    /// # Arguments
    ///
    /// * `dst` - The destination address.
    /// * `src` - The source address.
    /// * `len` - The number of bytes to copy.
    fn memcpy(&mut self, dst: Self::Value, src: Self::Value, len: Self::Value) -> Result<()>;

    /// Perform an inline memory copy from `src` to `dst` for `len` bytes (immediate length).
    ///
    /// # Arguments
    ///
    /// * `dst` - The destination address.
    /// * `src` - The source address.
    /// * `len` - The number of bytes to copy.
    fn memcpy_inline(&mut self, dst: Self::Value, src: Self::Value, len: i64) -> Result<()> {
        let len = self.iconst(self.type_int(64), len)?;
        self.memcpy(dst, src, len)
    }

    /// Mark a location in the code as unreachable.
    fn unreachable(&mut self);
}

/// The `EVMBuilder` trait provides an interface for building and manipulating Ethereum Virtual Machine (EVM) instructions.
/// This trait defines the core operations related to logging events, interacting with the contract's state, accessing block
/// and transaction information, handling memory operations, and performing contract creation and function calls.
///
/// The trait inherits from the `Builder` trait and is expected to be implemented by types that generate EVM-related code.
pub trait EVMBuilder: Builder {
    /// Calculate the Keccak-256 hash of a memory range.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting offset of the memory.
    /// * `length` - The number of bytes to hash.
    fn keccak256(&mut self, start: Self::Value, length: Self::Value) -> Result<Self::Value>;

    /// Retrieve the address of the contract currently executing.
    fn address(&mut self) -> Result<Self::Value>;

    /// Retrieve the address of the caller (sender of the transaction or message).
    fn caller(&mut self) -> Result<Self::Value>;

    /// Retrieve the amount of Ether sent with the current call.
    fn callvalue(&mut self) -> Result<Self::Value>;

    /// Load data from the call's input data starting at `offset`.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset in the call data to start loading from.
    fn calldataload(&mut self, offset: Self::Value) -> Result<Self::Value>;

    /// Retrieve the size of the input data.
    fn calldatasize(&mut self) -> Result<Self::Value>;

    /// Copy data from the call input to memory.
    ///
    /// # Arguments
    ///
    /// * `dest_offset` - Memory offset to copy to.
    /// * `data_offset` - Offset in the call data to start copying from.
    /// * `length` - Number of bytes to copy.
    fn calldatacopy(
        &mut self,
        dest_offset: Self::Value,
        data_offset: Self::Value,
        length: Self::Value,
    ) -> Result<()>;

    /// Retrieve the size of the contract's code.
    fn codesize(&mut self) -> Result<Self::Value>;

    /// Copy data from the contract's code to memory.
    ///
    /// # Arguments
    ///
    /// * `dest_offset` - Memory offset to copy to.
    /// * `data_offset` - Offset in the contract code to start copying from.
    /// * `length` - Number of bytes to copy.
    fn codecopy(
        &mut self,
        dest_offset: Self::Value,
        data_offset: Self::Value,
        length: Self::Value,
    ) -> Result<()>;

    /// Retrieve the size of the return data from the last call.
    fn returndatasize(&mut self) -> Result<Self::Value>;

    /// Copy data from the last call's return data to memory.
    ///
    /// # Arguments
    ///
    /// * `dest_offset` - Memory offset to copy to.
    /// * `data_offset` - Offset in the return data to start copying from.
    /// * `length` - Number of bytes to copy.
    fn returndatacopy(
        &mut self,
        dest_offset: Self::Value,
        data_offset: Self::Value,
        length: Self::Value,
    ) -> Result<()>;

    /// Load a value from the last call's return data at `data_offset`.
    ///
    /// # Arguments
    ///
    /// * `data_offset` - Offset in the return data to start loading from.
    fn returndataload(&mut self, data_offset: Self::Value) -> Result<Self::Value>;

    /// Retrieve the amount of gas remaining for execution.
    fn gas(&mut self) -> Result<Self::Value>;

    /// Retrieve the balance of the account at `account`.
    ///
    /// # Arguments
    ///
    /// * `account` - The account whose balance to retrieve.
    fn balance(&mut self, account: Self::Value) -> Result<Self::Value>;

    /// Retrieve the balance of the current contract.
    fn selfbalance(&mut self) -> Result<Self::Value>;

    /// Retrieve the size of the code at `account`.
    ///
    /// # Arguments
    ///
    /// * `account` - The account whose code size to retrieve.
    fn extcodesize(&mut self, account: Self::Value) -> Result<Self::Value>;

    /// Retrieve the hash of the code at `account`.
    ///
    /// # Arguments
    ///
    /// * `account` - The account whose code hash to retrieve.
    fn extcodehash(&mut self, account: Self::Value) -> Result<Self::Value>;

    /// Copy code from `account` to memory.
    ///
    /// # Arguments
    ///
    /// * `account` - The account whose code to copy.
    /// * `dest_offset` - Memory offset to copy to.
    /// * `code_offset` - Offset in the code to start copying from.
    /// * `length` - Number of bytes to copy.
    fn extcodecopy(
        &mut self,
        account: Self::Value,
        dest_offset: Self::Value,
        code_offset: Self::Value,
        length: Self::Value,
    );

    /// Retrieve the hash of the given block.
    ///
    /// # Arguments
    ///
    /// * `block_number` - The block number to retrieve the hash of.
    fn blockhash(&mut self, block_number: Self::Value) -> Result<Self::Value>;

    /// Load a value from storage at `key`.
    ///
    /// # Arguments
    ///
    /// * `key` - The storage key to load from.
    fn sload(&mut self, key: Self::Value) -> Result<Self::Value>;

    /// Store a value in storage at `key`.
    ///
    /// # Arguments
    ///
    /// * `key` - The storage key to store to.
    /// * `value` - The value to store.
    fn sstore(&mut self, key: Self::Value, value: Self::Value) -> Result<()>;

    /// Store a value in transient storage at `key`.
    ///
    /// # Arguments
    ///
    /// * `key` - The transient storage key.
    /// * `value` - The value to store.
    fn tstore(&mut self, key: Self::Value, value: Self::Value) -> Result<()>;

    /// Perform a load operation on transient storage.
    ///
    /// # Arguments
    ///
    /// * `key` - The transient storage key.
    fn tload(&mut self, key: Self::Value) -> Result<Self::Value>;

    /// Emits a log entry with data but no topics.
    ///
    /// # Parameters:
    /// - `offset`: The memory offset where the data to log begins.
    /// - `length`: The length of the data to log.
    fn log0(&mut self, offset: Self::Value, length: Self::Value);

    /// Emits a log entry with data and one topic.
    ///
    /// # Parameters:
    /// - `offset`: The memory offset where the data to log begins.
    /// - `length`: The length of the data to log.
    /// - `topic`: The log's single topic.
    fn log1(&mut self, offset: Self::Value, length: Self::Value, topic: Self::Value);

    /// Emits a log entry with data and two topics.
    ///
    /// # Parameters:
    /// - `offset`: The memory offset where the data to log begins.
    /// - `length`: The length of the data to log.
    /// - `topic1`: The first log topic.
    /// - `topic2`: The second log topic.
    fn log2(
        &mut self,
        offset: Self::Value,
        length: Self::Value,
        topic1: Self::Value,
        topic2: Self::Value,
    );

    /// Emits a log entry with data and three topics.
    ///
    /// # Parameters:
    /// - `offset`: The memory offset where the data to log begins.
    /// - `length`: The length of the data to log.
    /// - `topic1`: The first log topic.
    /// - `topic2`: The second log topic.
    /// - `topic3`: The third log topic.
    fn log3(
        &mut self,
        offset: Self::Value,
        length: Self::Value,
        topic1: Self::Value,
        topic2: Self::Value,
        topic3: Self::Value,
    );

    /// Emits a log entry with data and four topics.
    ///
    /// # Parameters:
    /// - `offset`: The memory offset where the data to log begins.
    /// - `length`: The length of the data to log.
    /// - `topic1`: The first log topic.
    /// - `topic2`: The second log topic.
    /// - `topic3`: The third log topic.
    /// - `topic4`: The fourth log topic.
    fn log4(
        &mut self,
        offset: Self::Value,
        length: Self::Value,
        topic1: Self::Value,
        topic2: Self::Value,
        topic3: Self::Value,
        topic4: Self::Value,
    );

    /// Destroys the contract and sends the remaining balance to the recipient.
    ///
    /// # Parameters:
    /// - `recipient`: The address that will receive the remaining balance of the contract.
    fn selfdestruct(&mut self, recipient: Self::Value);

    /// Returns the chain ID of the network.
    fn chainid(&mut self) -> Result<Self::Value>;

    /// Returns the address of the current block's miner (coinbase).
    fn coinbase(&mut self) -> Result<Self::Value>;

    /// Returns the current block's timestamp.
    fn timestamp(&mut self) -> Result<Self::Value>;

    /// Returns the current block number.
    fn number(&mut self) -> Result<Self::Value>;

    /// Returns the randomness value (prevrandao) of the previous block.
    fn prevrandao(&mut self) -> Result<Self::Value>;

    /// Returns the gas limit for the current block.
    fn gaslimit(&mut self) -> Result<Self::Value>;

    /// Returns the gas price for the current transaction.
    fn gasprice(&mut self) -> Result<Self::Value>;

    /// Returns the base fee of the current block.
    fn basefee(&mut self) -> Result<Self::Value>;

    /// Returns the transaction origin's address.
    fn origin(&mut self) -> Result<Self::Value>;

    /// Returns the hash of a blob at a specified index.
    ///
    /// # Parameters:
    /// - `index`: The index of the blob.
    fn blobhash(&mut self, index: Self::Value) -> Result<Self::Value>;

    /// Returns the base fee for a blob transaction.
    fn blobbasefee(&mut self) -> Result<Self::Value>;

    /// Loads a word from memory at the given offset.
    ///
    /// # Parameters:
    /// - `offset`: The memory offset to load from.
    fn mload(&mut self, offset: Self::Value) -> Result<Self::Value>;

    /// Stores a word to memory at the given offset.
    ///
    /// # Parameters:
    /// - `offset`: The memory offset to store the word at.
    /// - `data`: The data to store.
    fn mstore(&mut self, offset: Self::Value, data: Self::Value);

    /// Stores a single byte to memory at the given offset.
    ///
    /// # Parameters:
    /// - `offset`: The memory offset to store the byte at.
    /// - `data`: The data (byte) to store.
    fn mstore8(&mut self, offset: Self::Value, data: Self::Value);

    /// Returns the current memory size in bytes.
    fn msize(&mut self) -> Result<Self::Value>;

    /// Copies a section of memory from the source offset to the destination offset.
    ///
    /// # Parameters:
    /// - `dest_offset`: The destination memory offset.
    /// - `src_offset`: The source memory offset.
    /// - `length`: The number of bytes to copy.
    fn mcopy(&mut self, dest_offset: Self::Value, src_offset: Self::Value, length: Self::Value);

    /// Creates a new contract.
    ///
    /// # Parameters:
    /// - `value`: The amount of Ether to send to the new contract.
    /// - `offset`: The memory offset where the contract code begins.
    /// - `length`: The length of the contract code.
    fn create(
        &mut self,
        value: Self::Value,
        offset: Self::Value,
        length: Self::Value,
    ) -> Result<Self::Value>;

    /// Creates a new contract using `CREATE2` with a salt value for deterministic address generation.
    ///
    /// # Parameters:
    /// - `value`: The amount of Ether to send to the new contract.
    /// - `offset`: The memory offset where the contract code begins.
    /// - `length`: The length of the contract code.
    /// - `salt`: The salt value used for address generation.
    fn create2(
        &mut self,
        value: Self::Value,
        offset: Self::Value,
        length: Self::Value,
        salt: Self::Value,
    ) -> Result<Self::Value>;

    /// Performs a function call on a contract.
    ///
    /// # Parameters:
    /// - `gas`: The amount of gas to allocate for the call.
    /// - `address`: The address of the contract to call.
    /// - `value`: The amount of Ether to send with the call.
    /// - `input_offset`: The memory offset where the input data begins.
    /// - `input_length`: The length of the input data.
    /// - `output_offset`: The memory offset where the output data will be stored.
    /// - `output_length`: The length of the output data.
    #[allow(clippy::too_many_arguments)]
    fn call(
        &mut self,
        gas: Self::Value,
        address: Self::Value,
        value: Self::Value,
        input_offset: Self::Value,
        input_length: Self::Value,
        output_offset: Self::Value,
        output_length: Self::Value,
    ) -> Result<Self::Value>;

    /// Performs a function call on a contract, but the calling contract's code is executed in the context of the callee contract.
    #[allow(clippy::too_many_arguments)]
    fn callcode(
        &mut self,
        gas: Self::Value,
        address: Self::Value,
        value: Self::Value,
        input_offset: Self::Value,
        input_length: Self::Value,
        output_offset: Self::Value,
        output_length: Self::Value,
    ) -> Result<Self::Value>;

    /// Performs a function call where the caller's storage is used instead of the callee's storage.
    fn delegatecall(
        &mut self,
        gas: Self::Value,
        address: Self::Value,
        input_offset: Self::Value,
        input_length: Self::Value,
        output_offset: Self::Value,
        output_length: Self::Value,
    ) -> Result<Self::Value>;

    /// Performs a function call but does not allow the callee to modify state.
    fn staticcall(
        &mut self,
        gas: Self::Value,
        address: Self::Value,
        input_offset: Self::Value,
        input_length: Self::Value,
        output_offset: Self::Value,
        output_length: Self::Value,
    ) -> Result<Self::Value>;

    /// Returns execution control to the calling contract with return data.
    fn creturn(&mut self, offset: Self::Value, length: Self::Value);

    /// Aborts execution and reverts state changes, returning a specified output.
    fn revert(&mut self, offset: Self::Value, length: Self::Value);

    /// Halts execution without returning any data.
    fn stop(&mut self);

    /// Marks the current operation as invalid.
    fn invalid(&mut self);
}

pub trait WASMBuilder: Builder {}
