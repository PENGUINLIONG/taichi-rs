use std::{rc::Rc, ffi::c_void};
use std::ffi::CString;
use taichi_sys::*;
use crate::{
    get_last_error, Error, Result,
    runtime::Runtime,
    compute_graph::ComputeGraph,
};

struct AotModule_ {
    runtime: Runtime,
    aot_module: TiAotModule,
}
impl AotModule_ {
    pub fn load(runtime: &Runtime, module_dir: &str) -> Result<AotModule_> {
        let module_dir = CString::new(module_dir)
            .map_err(|_| Error::InvalidArgument(module_dir))?;
        let aot_module = unsafe {
            ti_load_aot_module(runtime.runtime(), module_dir.as_ptr())
        };
        get_last_error()?;
        let out = AotModule_ {
            runtime: runtime.clone(),
            aot_module,
        };
        Ok(out)
    }
    pub fn new(runtime: &Runtime, tcm: &[u8]) -> Result<AotModule_> {
        let aot_module = unsafe {
            ti_create_aot_module(runtime.runtime(), tcm.as_ptr() as *const c_void, tcm.len() as u64)
        };
        get_last_error()?;
        let out = AotModule_ {
            runtime: runtime.clone(),
            aot_module,
        };
        Ok(out)
    }
}
impl Drop for AotModule_ {
    fn drop(&mut self) {
        unsafe {
            ti_destroy_aot_module(self.aot_module);
        }
    }
}

#[derive(Clone)]
pub struct AotModule {
    inner: Rc<AotModule_>,
}
impl AotModule {
    pub fn load(runtime: &Runtime, module_dir: &str) -> Result<AotModule> {
        let inner = AotModule_::load(runtime, module_dir)?;
        let out = AotModule {
            inner: Rc::new(inner),
        };
        Ok(out)
    }
    pub fn new(runtime: &Runtime, tcm: &[u8]) -> Result<AotModule> {
        let inner = AotModule_::new(runtime, tcm)?;
        let out = AotModule {
            inner: Rc::new(inner),
        };
        Ok(out)
    }

    pub fn get_compute_graph(&self, name: &str) -> Result<ComputeGraph> {
        ComputeGraph::new(self, name)
    }

    pub fn runtime(&self) -> TiRuntime {
        self.inner.runtime.runtime()
    }
    pub fn aot_module(&self) -> TiAotModule {
        self.inner.aot_module
    }
}
