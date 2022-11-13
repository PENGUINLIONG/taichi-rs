use std::rc::Rc;
use std::ffi::CString;
use taichi_sys::*;
use crate::{
    check_taichi_error, TaichiResult as Result,
    runtime::Runtime,
    compute_graph::ComputeGraph,
};

struct AotModule_ {
    runtime: Runtime,
    aot_module: TiAotModule,
}
impl AotModule_ {
    pub fn new(runtime: &Runtime, module_dir: &str) -> Result<AotModule_> {
        let module_dir = CString::new(module_dir)
            .map_err(|_| TiError::InvalidArgument)?;
        let aot_module = unsafe {
            ti_load_aot_module(runtime.runtime(), module_dir.as_ptr())
        };
        check_taichi_error()?;
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
    pub fn new(runtime: &Runtime, module_dir: &str) -> Result<AotModule> {
        let inner = AotModule_::new(runtime, module_dir)?;
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
