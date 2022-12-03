#[allow(unused_imports)]
use std::os::raw::{c_void, c_char};
#[allow(unused_imports)]
use bitflags::bitflags;
#[allow(unused_imports)]
use crate::taichi_core::*;

/// Handle `TixNativeBufferUnity`
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TixNativeBufferUnity(pub usize);
impl TixNativeBufferUnity {
    pub fn null() -> Self {
        TixNativeBufferUnity(0)
    }
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `tix_import_native_runtime_unity`
pub fn tix_import_native_runtime_unity(
) -> TiRuntime;
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `tix_launch_kernel_async_unity`
pub fn tix_launch_kernel_async_unity(
  runtime: TiRuntime,
  kernel: TiKernel,
  arg_count: u32,
  args: *const TiArgument,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `tix_launch_compute_graph_async_unity`
pub fn tix_launch_compute_graph_async_unity(
  runtime: TiRuntime,
  compute_graph: TiComputeGraph,
  arg_count: u32,
  args: *const TiNamedArgument,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `tix_copy_memory_to_native_buffer_async_unity`
pub fn tix_copy_memory_to_native_buffer_async_unity(
  runtime: TiRuntime,
  dst: TixNativeBufferUnity,
  dst_offset: u64,
  src: *const TiMemorySlice,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `tix_copy_memory_device_to_host_unity`
pub fn tix_copy_memory_device_to_host_unity(
  runtime: TiRuntime,
  dst: *mut c_void,
  dst_offset: u64,
  src: *const TiMemorySlice,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `tix_copy_memory_host_to_device_unity`
pub fn tix_copy_memory_host_to_device_unity(
  runtime: TiRuntime,
  dst: *const TiMemorySlice,
  src: *const c_void,
  src_offset: u64,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `tix_submit_async_unity`
pub fn tix_submit_async_unity(
  runtime: TiRuntime,
) -> *mut c_void;
}

pub mod aliases {
pub use super::tix_import_native_runtime_unity as _import_native_runtime_unity;
pub use super::tix_launch_kernel_async_unity as _launch_kernel_async_unity;
pub use super::tix_launch_compute_graph_async_unity as _launch_compute_graph_async_unity;
pub use super::tix_copy_memory_to_native_buffer_async_unity as _copy_memory_to_native_buffer_async_unity;
pub use super::tix_copy_memory_device_to_host_unity as _copy_memory_device_to_host_unity;
pub use super::tix_copy_memory_host_to_device_unity as _copy_memory_host_to_device_unity;
pub use super::tix_submit_async_unity as _submit_async_unity;
}
