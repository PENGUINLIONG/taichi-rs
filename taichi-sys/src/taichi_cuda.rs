#[allow(unused_imports)]
use std::os::raw::{c_void, c_char};
#[allow(unused_imports)]
use bitflags::bitflags;
#[allow(unused_imports)]
use crate::taichi_core::*;

/// Structure `TiCudaMemoryInteropInfo`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiCudaMemoryInteropInfo {
  pub ptr: *mut c_void,
  pub size: u64,
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_export_cuda_memory`
pub fn ti_export_cuda_memory(
  runtime: TiRuntime,
  memory: TiMemory,
  interop_info: *mut TiCudaMemoryInteropInfo,
) -> ();
}

pub mod aliases {
pub use super::TiCudaMemoryInteropInfo as CudaMemoryInteropInfo;
pub use super::ti_export_cuda_memory as export_cuda_memory;
}
