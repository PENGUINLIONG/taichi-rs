/// # Vulkan Backend Features
/// 
/// Taichi's Vulkan API gives you further control over the Vulkan version and extension requirements and allows you to interop with external Vulkan applications with shared resources.
/// 
/// ## API Reference
#[allow(unused_imports)]
use std::os::raw::{c_void, c_char};
#[allow(unused_imports)]
use bitflags::bitflags;
#[allow(unused_imports)]
use crate::taichi_core::*;

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_create_vulkan_runtime_ext`
/// 
/// Creates a Vulkan Taichi runtime with user-controlled capability settings.
pub fn ti_create_vulkan_runtime_ext(
  api_version: u32,
  instance_extension_count: u32,
  instance_extensions: *const *const c_char,
  device_extension_count: u32,
  device_extensions: *const *const c_char,
) -> TiRuntime;
}

pub mod aliases {
pub use super::ti_create_vulkan_runtime_ext as create_vulkan_runtime_ext;
}
