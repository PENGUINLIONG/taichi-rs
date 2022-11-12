use std::os::raw::{c_void, c_char};
use bitflags::bitflags;
use crate::taichi_core::*;

// handle.native_buffer
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct TixNativeBufferUnity(usize);

// function.import_native_runtime
#[link(name = "taichi_c_api")]
extern "C" {
pub fn tix_import_native_runtime_unity(
) -> TiRuntime;
}

// function.launch_kernel_async
#[link(name = "taichi_c_api")]
extern "C" {
pub fn tix_launch_kernel_async_unity(
  runtime: TiRuntime,
  kernel: TiKernel,
  arg_count: u32,
  args: *const TiArgument
) -> ();
}

// function.launch_compute_graph_async
#[link(name = "taichi_c_api")]
extern "C" {
pub fn tix_launch_compute_graph_async_unity(
  runtime: TiRuntime,
  compute_graph: TiComputeGraph,
  arg_count: u32,
  args: *const TiNamedArgument
) -> ();
}

// function.copy_memory_to_native_buffer_async
#[link(name = "taichi_c_api")]
extern "C" {
pub fn tix_copy_memory_to_native_buffer_async_unity(
  runtime: TiRuntime,
  dst: TixNativeBufferUnity,
  dst_offset: u64,
  src: *const TiMemorySlice
) -> ();
}

// function.copy_memory_device_to_host
#[link(name = "taichi_c_api")]
extern "C" {
pub fn tix_copy_memory_device_to_host_unity(
  runtime: TiRuntime,
  dst: *mut c_void,
  dst_offset: u64,
  src: *const TiMemorySlice
) -> ();
}

// function.copy_memory_host_to_device
#[link(name = "taichi_c_api")]
extern "C" {
pub fn tix_copy_memory_host_to_device_unity(
  runtime: TiRuntime,
  dst: *const TiMemorySlice,
  src: *const c_void,
  src_offset: u64
) -> ();
}

// function.submit_async
#[link(name = "taichi_c_api")]
extern "C" {
pub fn tix_submit_async_unity(
  runtime: TiRuntime
) -> *mut c_void;
}
