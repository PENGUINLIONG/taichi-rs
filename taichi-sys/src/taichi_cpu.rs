use std::os::raw::{c_void, c_char};
use bitflags::bitflags;
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