use std::rc::Rc;
use std::ffi::CString;
use std::collections::HashMap;
use taichi_sys::*;
use crate::{
    get_last_error, Error, Result,
    aot_module::AotModule,
    ndarray::NdArray,
};

struct ComputeGraph_ {
    aot_module: AotModule,
    compute_graph: TiComputeGraph,
}
impl ComputeGraph_ {
    pub fn new(aot_module: &AotModule, name: &str) -> Result<ComputeGraph_> {
        let name = CString::new(name)
            .map_err(|_| Error::InvalidArgument(name))?;
        let compute_graph = unsafe {
            ti_get_aot_module_compute_graph(aot_module.aot_module(), name.as_ptr())
        };
        get_last_error()?;
        let out = ComputeGraph_ {
            aot_module: aot_module.clone(),
            compute_graph,
        };
        Ok(out)
    }
}

#[derive(Clone)]
pub struct ComputeGraph {
    inner: Rc<ComputeGraph_>,
    args: HashMap<CString, TiArgument>,
}
impl ComputeGraph {
    pub fn new(aot_module: &AotModule, name: &str) -> Result<ComputeGraph> {
        let inner = ComputeGraph_::new(aot_module, name)?;
        let out = ComputeGraph {
            inner: Rc::new(inner),
            args: Default::default()
        };
        Ok(out)
    }

    pub fn set_arg_i32(&mut self, name: &str, value: i32) -> Result<&mut Self> {
        let name = CString::new(name)
            .map_err(|_| Error::InvalidArgument(name))?;
        let arg = TiArgument {
            r#type: TiArgumentType::I32,
            value: TiArgumentValue {
                r#i32: value,
            },
        };
        self.args.insert(name, arg);
        Ok(self)
    }
    pub fn set_arg_f32(&mut self, name: &str, value: f32) -> Result<&mut Self> {
        let name = CString::new(name)
            .map_err(|_| Error::InvalidArgument(name))?;
        let arg = TiArgument {
            r#type: TiArgumentType::F32,
            value: TiArgumentValue {
                r#f32: value,
            },
        };
        self.args.insert(name, arg);
        Ok(self)
    }
    pub fn set_arg_ndarray<T>(&mut self, name: &str, value: &NdArray<T>) -> Result<&mut Self> {
        let name = CString::new(name)
            .map_err(|_| Error::InvalidArgument(name))?;
        let arg = TiArgument {
            r#type: TiArgumentType::Ndarray,
            value: TiArgumentValue {
                ndarray: value.ndarray().clone(),
            },
        };
        self.args.insert(name, arg);
        Ok(self)
    }

    pub fn runtime(&self) -> TiRuntime {
        self.inner.aot_module.runtime()
    }
    pub fn compute_graph(&self) -> TiComputeGraph {
        self.inner.compute_graph
    }

    pub fn launch(&self) -> Result<()> {
        let mut args = Vec::with_capacity(self.args.len());

        for (name, argument) in self.args.iter() {
            let arg = TiNamedArgument {
                name: name.as_ptr(),
                argument: argument.clone(),
            };
            args.push(arg);
        }

        let runtime = self.runtime();
        let compute_graph = self.compute_graph();
        unsafe {
            ti_launch_compute_graph(runtime, compute_graph, args.len() as u32, args.as_ptr());
        }
        get_last_error()?;

        Ok(())
    }
}
