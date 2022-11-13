use std::os::raw::{c_void, c_char};
use bitflags::bitflags;

// alias.bool
pub type TiBool = u32;

// definition.false
pub const TI_FALSE: u32 = 0;

// definition.true
pub const TI_TRUE: u32 = 1;

// alias.flags
pub type TiFlags = u32;

// definition.null_handle
pub const TI_NULL_HANDLE: u32 = 0;

// handle.runtime
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TiRuntime(pub usize);
impl TiRuntime {
    pub fn null() -> Self {
        TiRuntime(0)
    }
}

// handle.aot_module
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TiAotModule(pub usize);
impl TiAotModule {
    pub fn null() -> Self {
        TiAotModule(0)
    }
}

// handle.event
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TiEvent(pub usize);
impl TiEvent {
    pub fn null() -> Self {
        TiEvent(0)
    }
}

// handle.memory
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TiMemory(pub usize);
impl TiMemory {
    pub fn null() -> Self {
        TiMemory(0)
    }
}

// handle.image
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TiImage(pub usize);
impl TiImage {
    pub fn null() -> Self {
        TiImage(0)
    }
}

// handle.sampler
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TiSampler(pub usize);
impl TiSampler {
    pub fn null() -> Self {
        TiSampler(0)
    }
}

// handle.kernel
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TiKernel(pub usize);
impl TiKernel {
    pub fn null() -> Self {
        TiKernel(0)
    }
}

// handle.compute_graph
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TiComputeGraph(pub usize);
impl TiComputeGraph {
    pub fn null() -> Self {
        TiComputeGraph(0)
    }
}

// enumeration.error
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TiError {
  Truncated = 1,
  Success = 0,
  NotSupported = -1,
  CorruptedData = -2,
  NameNotFound = -3,
  InvalidArgument = -4,
  ArgumentNull = -5,
  ArgumentOutOfRange = -6,
  ArgumentNotFound = -7,
  InvalidInterop = -8,
  InvalidState = -9,
}

// enumeration.arch
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TiArch {
  X64 = 0,
  Arm64 = 1,
  Js = 2,
  Cc = 3,
  Wasm = 4,
  Cuda = 5,
  Metal = 6,
  Opengl = 7,
  Dx11 = 8,
  Dx12 = 9,
  Opencl = 10,
  Amdgpu = 11,
  Vulkan = 12,
}

// enumeration.capability
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TiCapability {
  SpirvVersion = 0,
  SpirvHasInt8 = 1,
  SpirvHasInt16 = 2,
  SpirvHasInt64 = 3,
  SpirvHasFloat16 = 4,
  SpirvHasFloat64 = 5,
  SpirvHasAtomicI64 = 6,
  SpirvHasAtomicFloat16 = 7,
  SpirvHasAtomicFloat16Add = 8,
  SpirvHasAtomicFloat16Minmax = 9,
  SpirvHasAtomicFloat = 10,
  SpirvHasAtomicFloatAdd = 11,
  SpirvHasAtomicFloatMinmax = 12,
  SpirvHasAtomicFloat64 = 13,
  SpirvHasAtomicFloat64Add = 14,
  SpirvHasAtomicFloat64Minmax = 15,
  SpirvHasVariablePtr = 16,
  SpirvHasPhysicalStorageBuffer = 17,
  SpirvHasSubgroupBasic = 18,
  SpirvHasSubgroupVote = 19,
  SpirvHasSubgroupArithmetic = 20,
  SpirvHasSubgroupBallot = 21,
  SpirvHasNonSemanticInfo = 22,
  SpirvHasNoIntegerWrapDecoration = 23,
}

// structure.capability_level_info
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiCapabilityLevelInfo {
  pub capability: TiCapability,
  pub level: u32,
}

// enumeration.data_type
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TiDataType {
  F16 = 0,
  F32 = 1,
  F64 = 2,
  I8 = 3,
  I16 = 4,
  I32 = 5,
  I64 = 6,
  U1 = 7,
  U8 = 8,
  U16 = 9,
  U32 = 10,
  U64 = 11,
  Gen = 12,
  Unknown = 13,
}

// enumeration.argument_type
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TiArgumentType {
  I32 = 0,
  F32 = 1,
  Ndarray = 2,
  Texture = 3,
}

// bit_field.memory_usage
bitflags! {
pub struct TiMemoryUsageFlags: u32 {
  const STORAGE_BIT = 1 << 0;
  const UNIFORM_BIT = 1 << 1;
  const VERTEX_BIT = 1 << 2;
  const INDEX_BIT = 1 << 3;
}
}

// structure.memory_allocate_info
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiMemoryAllocateInfo {
  pub size: u64,
  pub host_write: TiBool,
  pub host_read: TiBool,
  pub export_sharing: TiBool,
  pub usage: TiMemoryUsageFlags,
}

// structure.memory_slice
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiMemorySlice {
  pub memory: TiMemory,
  pub offset: u64,
  pub size: u64,
}

// structure.nd_shape
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiNdShape {
  pub dim_count: u32,
  pub dims: [u32; 16],
}

// structure.nd_array
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiNdArray {
  pub memory: TiMemory,
  pub shape: TiNdShape,
  pub elem_shape: TiNdShape,
  pub elem_type: TiDataType,
}

// bit_field.image_usage
bitflags! {
pub struct TiImageUsageFlags: u32 {
  const STORAGE_BIT = 1 << 0;
  const SAMPLED_BIT = 1 << 1;
  const ATTACHMENT_BIT = 1 << 2;
}
}

// enumeration.image_dimension
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TiImageDimension {
  D1D = 0,
  D2D = 1,
  D3D = 2,
  D1DArray = 3,
  D2DArray = 4,
  Cube = 5,
}

// enumeration.image_layout
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TiImageLayout {
  Undefined = 0,
  ShaderRead = 1,
  ShaderWrite = 2,
  ShaderReadWrite = 3,
  ColorAttachment = 4,
  ColorAttachmentRead = 5,
  DepthAttachment = 6,
  DepthAttachmentRead = 7,
  TransferDst = 8,
  TransferSrc = 9,
  PresentSrc = 10,
}

// enumeration.format
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TiFormat {
  Unknown = 0,
  R8 = 1,
  Rg8 = 2,
  Rgba8 = 3,
  Rgba8Srgb = 4,
  Bgra8 = 5,
  Bgra8Srgb = 6,
  R8U = 7,
  Rg8U = 8,
  Rgba8U = 9,
  R8I = 10,
  Rg8I = 11,
  Rgba8I = 12,
  R16 = 13,
  Rg16 = 14,
  Rgb16 = 15,
  Rgba16 = 16,
  R16U = 17,
  Rg16U = 18,
  Rgb16U = 19,
  Rgba16U = 20,
  R16I = 21,
  Rg16I = 22,
  Rgb16I = 23,
  Rgba16I = 24,
  R16F = 25,
  Rg16F = 26,
  Rgb16F = 27,
  Rgba16F = 28,
  R32U = 29,
  Rg32U = 30,
  Rgb32U = 31,
  Rgba32U = 32,
  R32I = 33,
  Rg32I = 34,
  Rgb32I = 35,
  Rgba32I = 36,
  R32F = 37,
  Rg32F = 38,
  Rgb32F = 39,
  Rgba32F = 40,
  Depth16 = 41,
  Depth24Stencil8 = 42,
  Depth32F = 43,
}

// structure.image_offset
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiImageOffset {
  pub x: u32,
  pub y: u32,
  pub z: u32,
  pub array_layer_offset: u32,
}

// structure.image_extent
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiImageExtent {
  pub width: u32,
  pub height: u32,
  pub depth: u32,
  pub array_layer_count: u32,
}

// structure.image_allocate_info
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiImageAllocateInfo {
  pub dimension: TiImageDimension,
  pub extent: TiImageExtent,
  pub mip_level_count: u32,
  pub format: TiFormat,
  pub export_sharing: TiBool,
  pub usage: TiImageUsageFlags,
}

// structure.image_slice
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiImageSlice {
  pub image: TiImage,
  pub offset: TiImageOffset,
  pub extent: TiImageExtent,
  pub mip_level: u32,
}

// enumeration.filter
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TiFilter {
  Nearest = 0,
  Linear = 1,
}

// enumeration.address_mode
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TiAddressMode {
  Repeat = 0,
  MirroredRepeat = 1,
  ClampToEdge = 2,
}

// structure.sampler_create_info
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiSamplerCreateInfo {
  pub mag_filter: TiFilter,
  pub min_filter: TiFilter,
  pub address_mode: TiAddressMode,
  pub max_anisotropy: f32,
}

// structure.texture
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiTexture {
  pub image: TiImage,
  pub sampler: TiSampler,
  pub dimension: TiImageDimension,
  pub extent: TiImageExtent,
  pub format: TiFormat,
}

// union.argument_value
#[repr(C)]
#[derive(Clone, Copy)]
pub union TiArgumentValue {
  pub r#i32: i32,
  pub r#f32: f32,
  pub ndarray: TiNdArray,
  pub texture: TiTexture,
}

// structure.argument
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiArgument {
  pub r#type: TiArgumentType,
  pub value: TiArgumentValue,
}

// structure.named_argument
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiNamedArgument {
  pub name: *const c_char,
  pub argument: TiArgument,
}

// function.get_last_error
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_get_last_error(
  message_size: u64,
  message: *mut c_char
) -> TiError;
}

// function.set_last_error
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_set_last_error(
  error: TiError,
  message: *const c_char
) -> ();
}

// function.create_runtime
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_create_runtime(
  arch: TiArch
) -> TiRuntime;
}

// function.destroy_runtime
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_destroy_runtime(
  runtime: TiRuntime
) -> ();
}

// function.set_runtime_capabilities
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_set_runtime_capabilities_ext(
  runtime: TiRuntime,
  capability_count: u32,
  capabilities: *const TiCapability
) -> ();
}

// function.allocate_memory
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_allocate_memory(
  runtime: TiRuntime,
  allocate_info: *const TiMemoryAllocateInfo
) -> TiMemory;
}

// function.free_memory
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_free_memory(
  runtime: TiRuntime,
  memory: TiMemory
) -> ();
}

// function.map_memory
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_map_memory(
  runtime: TiRuntime,
  memory: TiMemory
) -> *mut c_void;
}

// function.unmap_memory
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_unmap_memory(
  runtime: TiRuntime,
  memory: TiMemory
) -> ();
}

// function.allocate_image
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_allocate_image(
  runtime: TiRuntime,
  allocate_info: *const TiImageAllocateInfo
) -> TiImage;
}

// function.free_image
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_free_image(
  runtime: TiRuntime,
  image: TiImage
) -> ();
}

// function.create_sampler
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_create_sampler(
  runtime: TiRuntime,
  create_info: *const TiSamplerCreateInfo
) -> TiSampler;
}

// function.destroy_sampler
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_destroy_sampler(
  runtime: TiRuntime,
  sampler: TiSampler
) -> ();
}

// function.create_event
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_create_event(
  runtime: TiRuntime
) -> TiEvent;
}

// function.destroy_event
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_destroy_event(
  event: TiEvent
) -> ();
}

// function.copy_memory_device_to_device
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_copy_memory_device_to_device(
  runtime: TiRuntime,
  dst_memory: *const TiMemorySlice,
  src_memory: *const TiMemorySlice
) -> ();
}

// function.copy_image_device_to_device
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_copy_image_device_to_device(
  runtime: TiRuntime,
  dst_image: *const TiImageSlice,
  src_image: *const TiImageSlice
) -> ();
}

// function.track_image
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_track_image_ext(
  runtime: TiRuntime,
  image: TiImage,
  layout: TiImageLayout
) -> ();
}

// function.transition_image
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_transition_image(
  runtime: TiRuntime,
  image: TiImage,
  layout: TiImageLayout
) -> ();
}

// function.launch_kernel
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_launch_kernel(
  runtime: TiRuntime,
  kernel: TiKernel,
  arg_count: u32,
  args: *const TiArgument
) -> ();
}

// function.launch_compute_graph
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_launch_compute_graph(
  runtime: TiRuntime,
  compute_graph: TiComputeGraph,
  arg_count: u32,
  args: *const TiNamedArgument
) -> ();
}

// function.signal_event
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_signal_event(
  runtime: TiRuntime,
  event: TiEvent
) -> ();
}

// function.reset_event
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_reset_event(
  runtime: TiRuntime,
  event: TiEvent
) -> ();
}

// function.wait_event
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_wait_event(
  runtime: TiRuntime,
  event: TiEvent
) -> ();
}

// function.submit
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_submit(
  runtime: TiRuntime
) -> ();
}

// function.wait
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_wait(
  runtime: TiRuntime
) -> ();
}

// function.load_aot_module
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_load_aot_module(
  runtime: TiRuntime,
  module_path: *const c_char
) -> TiAotModule;
}

// function.destroy_aot_module
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_destroy_aot_module(
  aot_module: TiAotModule
) -> ();
}

// function.get_aot_module_kernel
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_get_aot_module_kernel(
  aot_module: TiAotModule,
  name: *const c_char
) -> TiKernel;
}

// function.get_aot_module_compute_graph
#[link(name = "taichi_c_api")]
extern "C" {
pub fn ti_get_aot_module_compute_graph(
  aot_module: TiAotModule,
  name: *const c_char
) -> TiComputeGraph;
}
