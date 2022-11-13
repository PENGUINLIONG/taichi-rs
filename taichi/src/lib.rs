use taichi_sys::*;

#[cfg(test)]
mod tests;

mod runtime;
mod memory;
mod ndarray;
mod aot_module;
mod compute_graph;

pub type TaichiResult<T> = std::result::Result<T, TiError>;
use TaichiResult as Result;

pub use runtime::Runtime;
pub use memory::Memory;
pub use ndarray::NdArray;
pub use aot_module::AotModule;
pub use compute_graph::ComputeGraph;

fn check_taichi_error() -> Result<()> {
    let err = unsafe {
        ti_get_last_error(0, std::ptr::null_mut())
    };
    if err == TiError::Success {
        Ok(())
    } else {
        Err(err)
    }
}
