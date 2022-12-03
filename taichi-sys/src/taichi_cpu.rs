#[allow(unused_imports)]
use std::os::raw::{c_void, c_char};
#[allow(unused_imports)]
use bitflags::bitflags;
#[allow(unused_imports)]
use crate::taichi_core::*;

// structure.cpu_memory_interop_info
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiCpuMemoryInteropInfo {
  pub ptr: *mut c_void,
  pub size: u64,
}

// function.export_cpu_memory
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_export_cpu_memory(
  runtime: TiRuntime,
  memory: TiMemory,
  interop_info: *mut TiCpuMemoryInteropInfo
) -> ();
}

pub mod aliases {
pub use super::TiCpuMemoryInteropInfo as CpuMemoryInteropInfo;
pub use super::ti_export_cpu_memory as export_cpu_memory;
}
