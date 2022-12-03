pub use taichi_sys::{
    taichi_core::aliases::*,
    taichi_cpu::aliases::*,
    taichi_cuda::aliases::*,
    taichi_opengl::aliases::*,
    taichi_unity::aliases::*,
    taichi_vulkan::aliases::*,
};

#[cfg(test)]
mod tests;

mod runtime;
mod memory;
mod ndarray;
mod aot_module;
mod compute_graph;

pub type TaichiResult<T> = std::result::Result<T, Error>;
pub use TaichiResult as Result;

pub use runtime::Runtime;
pub use memory::Memory;
pub use ndarray::NdArray;
pub use aot_module::AotModule;
pub use compute_graph::ComputeGraph;

fn check_taichi_error() -> Result<()> {
    let err = unsafe {
        get_last_error(0, std::ptr::null_mut())
    };
    if err == Error::Success {
        Ok(())
    } else {
        Err(err)
    }
}
