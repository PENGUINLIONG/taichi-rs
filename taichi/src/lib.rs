pub use taichi_sys::{
    taichi_core::aliases::*,
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
        let mut msg_size: u64 = 0;
        get_last_error(&mut msg_size as *mut u64, std::ptr::null_mut())
    };
    if err == Error::Success {
        Ok(())
    } else {
        Err(err)
    }
}
