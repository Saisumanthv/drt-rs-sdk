use std::{ops::Deref, sync::Arc};

use numbat_vm_executor1::Executor;

use super::BuiltinFunctionContainer;

pub struct BlockchainVM {
    pub builtin_functions: BuiltinFunctionContainer,
    pub executor: Box<dyn Executor + Send + Sync>,
}

#[derive(Clone)]
pub struct BlockchainVMRef(Arc<BlockchainVM>);

impl BlockchainVM {
    pub fn new(executor: Box<dyn Executor + Send + Sync>) -> Self {
        BlockchainVM {
            builtin_functions: BuiltinFunctionContainer,
            executor,
        }
    }
}

impl BlockchainVMRef {
    pub fn new(executor: Box<dyn Executor + Send + Sync>) -> Self {
        BlockchainVMRef(Arc::new(BlockchainVM::new(executor)))
    }
}

impl Deref for BlockchainVMRef {
    type Target = BlockchainVM;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
