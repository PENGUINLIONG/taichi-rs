use std::rc::Rc;
use taichi_sys::*;
use crate::{
    check_taichi_error, TaichiResult as Result,
    aot_module::AotModule,
    memory::MemoryBuilder,
    ndarray::NdArrayBuilder,
};

struct Runtime_ {
    arch: TiArch,
    runtime: TiRuntime,
}
impl Runtime_ {
    fn new(arch: TiArch, device_index: u32) -> Result<Self> {
        let runtime = unsafe {
            ti_create_runtime(arch, device_index)
        };
        check_taichi_error()?;
        Ok(Runtime_ { arch, runtime })
    }
}
impl Drop for Runtime_ {
    fn drop(&mut self) {
        unsafe {
            ti_destroy_runtime(self.runtime)
        };
    }
}

#[derive(Clone)]
pub struct Runtime {
    inner: Rc<Runtime_>,
}
impl Runtime {
    pub fn new(arch: TiArch) -> Result<Self> {
        return Self::with_device_index(arch, 0);
    }
    pub fn with_device_index(arch: TiArch, device_index: u32) -> Result<Self> {
        let inner = Runtime_::new(arch, device_index)?;
        let out = Runtime {
            inner: Rc::new(inner),
        };
        Ok(out)
    }

    pub fn arch(&self) -> TiArch {
        self.inner.arch
    }
    pub fn runtime(&self) -> TiRuntime {
        self.inner.runtime
    }

    pub fn allocate_memory(&self) -> MemoryBuilder<'_> {
        MemoryBuilder::new(self)
    }
    pub fn allocate_ndarray<T>(&self) -> NdArrayBuilder<'_, T> {
        NdArrayBuilder::<T>::new(self)
    }

    pub fn load_aot_module(&self, module_dir: &str) -> Result<AotModule> {
        AotModule::load(self, module_dir)
    }
    pub fn create_aot_module(&self, tcm: &[u8]) -> Result<AotModule> {
        AotModule::new(self, tcm)
    }

    pub fn wait(&self) -> Result<()> {
        unsafe {
            ti_wait(self.runtime());
        }
        check_taichi_error()?;
        Ok(())
    }

}
