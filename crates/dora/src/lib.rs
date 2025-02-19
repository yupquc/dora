#[cfg(test)]
mod tests;

pub use dora_compiler as compiler;
pub use dora_primitives as primitives;
pub use dora_runtime as runtime;

pub use dora_compiler::{
    context::Context,
    dora,
    evm::{self, program::Program, CompileOptions, EVMCompiler},
    pass,
    wasm::{self, Config, WASMCompiler},
    Compiler,
};
pub use dora_primitives::{spec::SpecId, Bytecode, Bytes, Bytes32, EVMBytecode};
pub use dora_runtime::executor::{ExecuteKind, Executor};
pub use dora_runtime::{
    artifact::Artifact,
    call::CallResult,
    context::VMContext,
    handler::{Frame, Handler},
    result::{EVMError, ExecutionResult},
    vm::VM,
};
pub use dora_runtime::{context::RuntimeContext, env::TxKind};
pub use dora_runtime::{context::Stack, env::Env};
pub use dora_runtime::{
    db::{Database, MemoryDB},
    result::ResultAndState,
};
use std::sync::Arc;

/// Run the EVM environment with the given state database and return the execution result and final state.
///
/// # Arguments
///
/// * `env` - The environment configuration for the execution (e.g., gas limit, transaction data, etc.).
/// * `db` - A mutable reference to the `MemoryDB` that simulates the contract storage and state database.
///
/// # Returns
///
/// Returns `ResultAndState`, containing the execution result and the final state after execution.
///
/// # Errors
///
/// Returns an error if the program fails to execute or if the bytecode or address is invalid.
#[inline]
pub fn run<DB: Database + 'static>(
    env: Env,
    db: DB,
    spec_id: SpecId,
) -> Result<ExecutionResult, EVMError> {
    VM::new(VMContext::new(db, env, spec_id, compile_handler())).transact_commit()
}

/// Compile Handler for the VM.
#[inline]
pub fn compile_handler<'a, DB: Database + 'a>() -> Handler<'a, DB> {
    Handler {
        call_handler: Arc::new(compile_call_handler),
    }
}

/// Default frame calling hanlder, using dora compiler and runtime to run EVM and WASM contract.
fn compile_call_handler<DB: Database>(
    frame: Frame,
    ctx: &mut VMContext<'_, DB>,
) -> Result<CallResult, EVMError> {
    let code_hash = frame.contract.hash.unwrap_or_default();
    let spec_id = ctx.spec_id();
    let artifact = ctx.db.get_artifact(code_hash);
    let artifact = if let Ok(Some(artifact)) = artifact {
        artifact
    } else {
        let artifact = build_artifact::<DB>(&frame.contract.code, ctx.spec_id())
            .map_err(|e| EVMError::Custom(e.to_string()))?;
        ctx.db.set_artifact(code_hash, artifact.clone());
        artifact
    };
    let mut runtime_context = RuntimeContext::new(
        frame.contract,
        frame.depth,
        frame.is_static,
        frame.is_eof_init,
        ctx,
        spec_id,
        frame.gas_limit,
    );
    artifact.execute(&mut runtime_context, &mut Stack::new(), &mut 0);
    Ok(CallResult::new_with_runtime_context_and_gas_limit(
        &runtime_context,
        frame.gas_limit,
    ))
}

/// Run transaction with the runtime context.
pub fn run_with_context<DB: Database>(runtime_context: &mut RuntimeContext) -> anyhow::Result<u8> {
    let artifact: DB::Artifact = build_artifact::<DB>(
        &runtime_context.contract.code,
        runtime_context.inner.spec_id,
    )?;
    Ok(artifact.execute(runtime_context, &mut Stack::new(), &mut 0))
}

/// Build opcode to the artifact
pub fn build_artifact<DB: Database>(
    code: &Bytecode,
    spec_id: SpecId,
) -> anyhow::Result<DB::Artifact> {
    match code {
        Bytecode::EVM(code) => build_evm_artifact::<DB>(code, spec_id),
        Bytecode::WASM(code) => build_wasm_artifact::<DB>(code),
    }
}

/// Build EVM opcode to the artifact
pub fn build_evm_artifact<DB: Database>(
    code: &EVMBytecode,
    spec_id: SpecId,
) -> anyhow::Result<DB::Artifact> {
    // Compile the contract code
    let program = Program::from_opcodes(code.bytecode(), code.eof().cloned());
    let context = Context::new();
    let compiler = EVMCompiler::new(&context);
    let mut module = compiler.compile(
        &program,
        &(),
        &CompileOptions {
            spec_id,
            ..Default::default()
        },
    )?;
    // Lowering the EVM dialect to MLIR builtin dialects.
    evm::pass::run(&context.mlir_context, &mut module.mlir_module)?;
    dora::pass::run(
        &context.mlir_context,
        &mut module.mlir_module,
        &dora::pass::PassOptions {
            code_size: program.code_size(),
            spec_id,
            ..Default::default()
        },
    )?;
    pass::run(&context.mlir_context, &mut module.mlir_module)?;
    debug_assert!(module.mlir_module.as_operation().verify());
    let executor = Executor::new(module.module(), Default::default(), ExecuteKind::EVM);
    Ok(DB::Artifact::new(executor))
}

/// Build WASM opcode to the artifact
pub fn build_wasm_artifact<DB: Database>(code: &Bytes) -> anyhow::Result<DB::Artifact> {
    let context = Context::new();
    let compiler = WASMCompiler::new(&context, Config::default());
    // Compile WASM Bytecode to MLIR EVM Dialect
    let mut module = compiler.compile(code)?;
    let instance = compiler.build_instance(code)?;
    // Lowering the WASM dialect to the Dora dialect.
    wasm::pass::run(&context.mlir_context, &mut module.mlir_module)?;
    // Lowering the Dora dialect to MLIR builtin dialects.
    dora::pass::run(
        &context.mlir_context,
        &mut module.mlir_module,
        &dora::pass::PassOptions {
            code_size: code.len() as u32,
            ..Default::default()
        },
    )?;
    pass::run(&context.mlir_context, &mut module.mlir_module)?;
    debug_assert!(module.mlir_module.as_operation().verify());

    let executor = Executor::new(
        module.module(),
        Default::default(),
        ExecuteKind::new_wasm(instance),
    );
    Ok(DB::Artifact::new(executor))
}

/// Run hex-encoded EVM bytecode with custom calldata and return the execution result and final state.
///
/// # Arguments
///
/// * `program` - A string representing the hex-encoded EVM bytecode.
/// * `calldata` - A byte slice containing the custom calldata to use for the execution.
/// * `initial_gas` - The initial amount of gas allocated for the execution.
///
/// # Returns
///
/// Returns `ResultAndState`, containing the execution result and the final state after execution.
///
/// # Errors
///
/// Returns an error if the bytecode fails to decode or execute.
pub fn run_bytecode_with_calldata(
    program: &str,
    calldata: &str,
    initial_gas: u64,
    spec_id: SpecId,
) -> anyhow::Result<ExecutionResult> {
    let opcodes = hex::decode(program)?;
    let calldata = hex::decode(calldata)?;
    let address = Bytes32::from(40_u32).to_address();
    let mut env = Env::default();
    env.tx.transact_to = TxKind::Call(address);
    env.tx.gas_limit = initial_gas;
    env.tx.data = Bytes::from(calldata);
    env.tx.caller = Bytes32::from(10000_u32).to_address();
    let db = MemoryDB::new().with_contract(address, Bytecode::new(Bytes::from(opcodes)));
    run(env, db, spec_id).map_err(|err| anyhow::anyhow!(err))
}
