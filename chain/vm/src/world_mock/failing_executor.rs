use numbat_vm_executor1::Executor;

/// Dummy executor that fails whenever called.
///
/// Used in dummy contexts.
///
/// TODO: either remove, or move to vm-executor repo.
pub struct FailingExecutor;

impl Executor for FailingExecutor {
    fn set_vm_hooks_ptr(
        &mut self,
        _vm_hooks_ptr: *mut std::ffi::c_void,
    ) -> Result<(), numbat_vm_executor1::ExecutorError> {
        panic!("called FailingExecutor")
    }

    fn set_opcode_cost(
        &mut self,
        _opcode_cost: &numbat_vm_executor1::OpcodeCost,
    ) -> Result<(), numbat_vm_executor1::ExecutorError> {
        panic!("called FailingExecutor")
    }

    fn new_instance(
        &self,
        _wasm_bytes: &[u8],
        _compilation_options: &numbat_vm_executor1::CompilationOptions,
    ) -> Result<
        Box<dyn numbat_vm_executor1::Instance>,
        numbat_vm_executor1::ExecutorError,
    > {
        panic!("called FailingExecutor")
    }

    fn new_instance_from_cache(
        &self,
        _cache_bytes: &[u8],
        _compilation_options: &numbat_vm_executor1::CompilationOptions,
    ) -> Result<
        Box<dyn numbat_vm_executor1::Instance>,
        numbat_vm_executor1::ExecutorError,
    > {
        panic!("called FailingExecutor")
    }
}
