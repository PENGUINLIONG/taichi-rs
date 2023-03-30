/// # Core Functionality
/// 
/// Taichi Core exposes all necessary interfaces for offloading the AOT modules to Taichi. The following is a list of features that are available regardless of your backend. The corresponding APIs are still under development and subject to change.
/// 
/// ## Availability
/// 
/// Taichi C-API intends to support the following backends:
/// 
/// |Backend     |Offload Target   |Maintenance Tier | Stabilized? |
/// |------------|-----------------|-----------------|-------------|
/// |Vulkan      |GPU              |Tier 1           | Yes         |
/// |Metal       |GPU (macOS, iOS) |Tier 2           | No          |
/// |CUDA (LLVM) |GPU (NVIDIA)     |Tier 2           | No          |
/// |CPU (LLVM)  |CPU              |Tier 2           | No          |
/// |OpenGL      |GPU              |Tier 2           | No          |
/// |OpenGL ES   |GPU              |Tier 2           | No          |
/// |DirectX 11  |GPU (Windows)    |N/A              | No          |
/// 
/// The backends with tier-1 support are being developed and tested more intensively. And most new features will be available on Vulkan first because it has the most outstanding cross-platform compatibility among all the tier-1 backends.
/// For the backends with tier-2 support, you should expect a delay in the fixes to minor issues.
/// 
/// For convenience, in the following text and other C-API documents, the term *host* refers to the user of the C-API; the term *device* refers to the logical (conceptual) compute device, to which Taichi's runtime offloads its compute tasks. A *device* may not be a physical discrete processor other than the CPU and the *host* may *not* be able to access the memory allocated on the *device*.
/// 
/// Unless otherwise specified, **device**, **backend**, **offload target**, and **GPU** are interchangeable; **host**, **user code**, **user procedure**, and **CPU** are interchangeable.
/// 
/// ## HowTo
/// 
/// The following section provides a brief introduction to the Taichi C-API.
/// 
/// ### Create and destroy a Runtime Instance
/// 
/// You *must* create a runtime instance before working with Taichi, and *only* one runtime per thread. Currently, we do not officially claim that multiple runtime instances can coexist in a process, but please feel free to [file an issue with us](https://github.com/taichi-dev/taichi/issues) if you run into any problem with runtime instance coexistence.
/// 
/// ```cpp
/// // Create a Taichi Runtime on Vulkan device at index 0.
/// TiRuntime runtime = ti_create_runtime(TI_ARCH_VULKAN, 0);
/// ```
/// 
/// When your program runs to the end, ensure that:
/// - You destroy the runtime instance,
/// - All related resources are destroyed before the [`TiRuntime`](#handle-tiruntime) itself.
/// 
/// ```cpp
/// ti_destroy_runtime(runtime);
/// ```
/// 
/// ### Allocate and free memory
/// 
/// Allocate a piece of memory that is visible only to the device. On the GPU backends, it usually means that the memory is located in the graphics memory (GRAM).
/// 
/// ```cpp
/// TiMemoryAllocateInfo mai {};
/// mai.size = 1024; // Size in bytes.
/// mai.usage = TI_MEMORY_USAGE_STORAGE_BIT;
/// TiMemory memory = ti_allocate_memory(runtime, &mai);
/// ```
/// 
/// Allocated memory is automatically freed when the related [`TiRuntime`](#handle-tiruntime) is destroyed. You can also manually free the allocated memory.
/// 
/// ```cpp
/// ti_free_memory(runtime, memory);
/// ```
/// 
/// ### Allocate host-accessible memory
/// 
/// By default, memory allocations are physically or conceptually local to the offload target for performance reasons. You can configure the [`TiMemoryAllocateInfo`](#structure-timemoryallocateinfo) to enable host access to memory allocations. But please note that host-accessible allocations *may* slow down computation on GPU because of the limited bus bandwidth between the host memory and the device.
/// 
/// You *must* set `host_write` to [`TI_TRUE`](#definition-ti_true) to allow zero-copy data streaming to the memory.
/// 
/// ```cpp
/// TiMemoryAllocateInfo mai {};
/// mai.size = 1024; // Size in bytes.
/// mai.host_write = TI_TRUE;
/// mai.usage = TI_MEMORY_USAGE_STORAGE_BIT;
/// TiMemory steaming_memory = ti_allocate_memory(runtime, &mai);
/// 
/// // ...
/// 
/// std::vector<uint8_t> src = some_random_data_source();
/// 
/// void* dst = ti_map_memory(runtime, steaming_memory);
/// std::memcpy(dst, src.data(), src.size());
/// ti_unmap_memory(runtime, streaming_memory);
/// ```
/// 
/// To read data back to the host, `host_read` *must* be set to [`TI_TRUE`](#definition-ti_true).
/// 
/// ```cpp
/// TiMemoryAllocateInfo mai {};
/// mai.size = 1024; // Size in bytes.
/// mai.host_read = TI_TRUE;
/// mai.usage = TI_MEMORY_USAGE_STORAGE_BIT;
/// TiMemory read_back_memory = ti_allocate_memory(runtime, &mai);
/// 
/// // ...
/// 
/// std::vector<uint8_t> dst(1024);
/// void* src = ti_map_memory(runtime, read_back_memory);
/// std::memcpy(dst.data(), src, dst.size());
/// ti_unmap_memory(runtime, read_back_memory);
/// 
/// ti_free_memory(runtime, read_back_memory);
/// ```
/// 
/// > You can set `host_read` and `host_write` at the same time.
/// 
/// ### Load and destroy a Taichi AOT module
/// 
/// You can load a Taichi AOT module from the filesystem.
/// 
/// ```cpp
/// TiAotModule aot_module = ti_load_aot_module(runtime, "/path/to/aot/module");
/// ```
/// 
/// `/path/to/aot/module` should point to the directory that contains a `metadata.json`.
/// 
/// You can destroy an unused AOT module, but please ensure that there is no kernel or compute graph related to it pending to [`ti_flush`](#function-ti_flush).
/// 
/// ```cpp
/// ti_destroy_aot_module(aot_module);
/// ```
/// 
/// ### Launch kernels and compute graphs
/// 
/// You can extract kernels and compute graphs from an AOT module. Kernel and compute graphs are a part of the module, so you don't have to destroy them.
/// 
/// ```cpp
/// TiKernel kernel = ti_get_aot_module_kernel(aot_module, "foo");
/// TiComputeGraph compute_graph = ti_get_aot_module_compute_graph(aot_module, "bar");
/// ```
/// 
/// You can launch a kernel with positional arguments. Please ensure the types, the sizes and the order matches the source code in Python.
/// 
/// ```cpp
/// TiNdArray ndarray{};
/// ndarray.memory = get_some_memory();
/// ndarray.shape.dim_count = 1;
/// ndarray.shape.dims[0] = 16;
/// ndarray.elem_shape.dim_count = 2;
/// ndarray.elem_shape.dims[0] = 4;
/// ndarray.elem_shape.dims[1] = 4;
/// ndarray.elem_type = TI_DATA_TYPE_F32;
/// 
/// std::array<TiArgument, 3> args{};
/// 
/// TiArgument& arg0 = args[0];
/// arg0.type = TI_ARGUMENT_TYPE_I32;
/// arg0.value.i32 = 123;
/// 
/// TiArgument& arg1 = args[1];
/// arg1.type = TI_ARGUMENT_TYPE_F32;
/// arg1.value.f32 = 123.0f;
/// 
/// TiArgument& arg2 = args[2];
/// arg2.type = TI_ARGUMENT_TYPE_NDARRAY;
/// arg2.value.ndarray = ndarray;
/// 
/// ti_launch_kernel(runtime, kernel, args.size(), args.data());
/// ```
/// 
/// You can launch a compute graph in a similar way. But additionally please ensure the argument names matches those in the Python source.
/// 
/// ```cpp
/// std::array<TiNamedArgument, 3> named_args{};
/// TiNamedArgument& named_arg0 = named_args[0];
/// named_arg0.name = "foo";
/// named_arg0.argument = args[0];
/// TiNamedArgument& named_arg1 = named_args[1];
/// named_arg1.name = "bar";
/// named_arg1.argument = args[1];
/// TiNamedArgument& named_arg2 = named_args[2];
/// named_arg2.name = "baz";
/// named_arg2.argument = args[2];
/// 
/// ti_launch_compute_graph(runtime, compute_graph, named_args.size(), named_args.data());
/// ```
/// 
/// When you have launched all kernels and compute graphs for this batch, you should [`ti_flush`](#function-ti_flush) and [`ti_wait`](#function-ti_wait) for the execution to finish.
/// 
/// ```cpp
/// ti_flush(runtime);
/// ti_wait(runtime);
/// ```
/// 
/// **WARNING** This part is subject to change. We will introduce multi-queue in the future.
/// 
/// ## API Reference
#[allow(unused_imports)]
use std::os::raw::{c_void, c_char};
#[allow(unused_imports)]
use bitflags::bitflags;

/// Alias `TiBool`
/// 
/// A boolean value. Can be either [`TI_TRUE`](#definition-ti_true) or [`TI_FALSE`](#definition-ti_false). Assignment with other values could lead to undefined behavior.
pub type TiBool = u32;

/// Definition `TI_FALSE`
/// 
/// A condition or a predicate is not satisfied; a statement is invalid.
pub const TI_FALSE: u32 = 0;

/// Definition `TI_TRUE`
/// 
/// A condition or a predicate is satisfied; a statement is valid.
pub const TI_TRUE: u32 = 1;

/// Alias `TiFlags`
/// 
/// A bit field that can be used to represent 32 orthogonal flags. Bits unspecified in the corresponding flag enum are ignored.
/// 
/// > Enumerations and bit-field flags in the C-API have a `TI_XXX_MAX_ENUM` case to ensure the enum has a 32-bit range and in-memory size. It has no semantical impact and can be safely ignored.
pub type TiFlags = u32;

/// Definition `TI_NULL_HANDLE`
/// 
/// A sentinal invalid handle that will never be produced from a valid call to Taichi C-API.
pub const TI_NULL_HANDLE: u32 = 0;

/// Handle `TiRuntime`
/// 
/// Taichi runtime represents an instance of a logical backend and its internal dynamic state. The user is responsible to synchronize any use of [`TiRuntime`](#handle-tiruntime). The user *must not* manipulate multiple [`TiRuntime`](#handle-tiruntime)s in the same thread.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TiRuntime(pub usize);
impl TiRuntime {
    pub fn null() -> Self {
        TiRuntime(0)
    }
}

/// Handle `TiAotModule`
/// 
/// An ahead-of-time (AOT) compiled Taichi module, which contains a collection of kernels and compute graphs.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TiAotModule(pub usize);
impl TiAotModule {
    pub fn null() -> Self {
        TiAotModule(0)
    }
}

/// Handle `TiMemory`
/// 
/// A contiguous allocation of device memory.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TiMemory(pub usize);
impl TiMemory {
    pub fn null() -> Self {
        TiMemory(0)
    }
}

/// Handle `TiImage`
/// 
/// A contiguous allocation of device image.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TiImage(pub usize);
impl TiImage {
    pub fn null() -> Self {
        TiImage(0)
    }
}

/// Handle `TiSampler`
/// 
/// An image sampler. [`TI_NULL_HANDLE`](#definition-ti_null_handle) represents a default image sampler provided by the runtime implementation. The filter modes and address modes of default samplers depend on backend implementation.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TiSampler(pub usize);
impl TiSampler {
    pub fn null() -> Self {
        TiSampler(0)
    }
}

/// Handle `TiKernel`
/// 
/// A Taichi kernel that can be launched on the offload target for execution.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TiKernel(pub usize);
impl TiKernel {
    pub fn null() -> Self {
        TiKernel(0)
    }
}

/// Handle `TiComputeGraph`
/// 
/// A collection of Taichi kernels (a compute graph) to launch on the offload target in a predefined order.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TiComputeGraph(pub usize);
impl TiComputeGraph {
    pub fn null() -> Self {
        TiComputeGraph(0)
    }
}

/// Enumeration `TiError`
/// 
/// Errors reported by the Taichi C-API.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TiError {
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
  IncompatibleModule = -10,
  OutOfMemory = -11,
}

/// Enumeration `TiArch`
/// 
/// Types of backend archs.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TiArch {
  Reserved = 0,
  Vulkan = 1,
  Metal = 2,
  Cuda = 3,
  X64 = 4,
  Arm64 = 5,
  Opengl = 6,
  Gles = 7,
}

/// Enumeration `TiCapability`
/// 
/// Device capabilities.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TiCapability {
  Reserved = 0,
  SpirvVersion = 1,
  SpirvHasInt8 = 2,
  SpirvHasInt16 = 3,
  SpirvHasInt64 = 4,
  SpirvHasFloat16 = 5,
  SpirvHasFloat64 = 6,
  SpirvHasAtomicInt64 = 7,
  SpirvHasAtomicFloat16 = 8,
  SpirvHasAtomicFloat16Add = 9,
  SpirvHasAtomicFloat16Minmax = 10,
  SpirvHasAtomicFloat = 11,
  SpirvHasAtomicFloatAdd = 12,
  SpirvHasAtomicFloatMinmax = 13,
  SpirvHasAtomicFloat64 = 14,
  SpirvHasAtomicFloat64Add = 15,
  SpirvHasAtomicFloat64Minmax = 16,
  SpirvHasVariablePtr = 17,
  SpirvHasPhysicalStorageBuffer = 18,
  SpirvHasSubgroupBasic = 19,
  SpirvHasSubgroupVote = 20,
  SpirvHasSubgroupArithmetic = 21,
  SpirvHasSubgroupBallot = 22,
  SpirvHasNonSemanticInfo = 23,
  SpirvHasNoIntegerWrapDecoration = 24,
}

/// Structure `TiCapabilityLevelInfo`
/// 
/// An integral device capability level. It currently is not guaranteed that a higher level value is compatible with a lower level value.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiCapabilityLevelInfo {
  pub capability: TiCapability,
  pub level: u32,
}

/// Enumeration `TiDataType`
/// 
/// Elementary (primitive) data types. There might be vendor-specific constraints on the available data types so it's recommended to use 32-bit data types if multi-platform distribution is desired.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// Enumeration `TiArgumentType`
/// 
/// Types of kernel and compute graph argument.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TiArgumentType {
  I32 = 0,
  F32 = 1,
  Ndarray = 2,
  Texture = 3,
  Scalar = 4,
}

bitflags! {
/// BitField `TiMemoryUsageFlags`
/// 
/// Usages of a memory allocation. Taichi requires kernel argument memories to be allocated with `TI_MEMORY_USAGE_STORAGE_BIT`.
#[repr(transparent)]
pub struct TiMemoryUsageFlags: u32 {
  /// The memory can be read/write accessed by any kernel.
  const STORAGE_BIT = 1 << 0;
  /// The memory can be used as a uniform buffer in graphics pipelines.
  const UNIFORM_BIT = 1 << 1;
  /// The memory can be used as a vertex buffer in graphics pipelines.
  const VERTEX_BIT = 1 << 2;
  /// The memory can be used as an index buffer in graphics pipelines.
  const INDEX_BIT = 1 << 3;
}
}

/// Structure `TiMemoryAllocateInfo`
/// 
/// Parameters of a newly allocated memory.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiMemoryAllocateInfo {
  /// Size of the allocation in bytes.
  pub size: u64,
  /// True if the host needs to write to the allocated memory.
  pub host_write: TiBool,
  /// True if the host needs to read from the allocated memory.
  pub host_read: TiBool,
  /// True if the memory allocation needs to be exported to other backends (e.g., from Vulkan to CUDA).
  pub export_sharing: TiBool,
  /// All possible usage of this memory allocation. In most cases, `bit_field.memory_usage.storage` is enough.
  pub usage: TiMemoryUsageFlags,
}

/// Structure `TiMemorySlice`
/// 
/// A subsection of a memory allocation. The sum of `offset` and `size` cannot exceed the size of `memory`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiMemorySlice {
  /// The subsectioned memory allocation.
  pub memory: TiMemory,
  /// Offset from the beginning of the allocation.
  pub offset: u64,
  /// Size of the subsection.
  pub size: u64,
}

/// Structure `TiNdShape`
/// 
/// Multi-dimensional size of an ND-array. Dimension sizes after `dim_count` are ignored.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiNdShape {
  /// Number of dimensions.
  pub dim_count: u32,
  /// Dimension sizes.
  pub dims: [u32; 16],
}

/// Structure `TiNdArray`
/// 
/// Multi-dimensional array of dense primitive data.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiNdArray {
  /// Memory bound to the ND-array.
  pub memory: TiMemory,
  /// Shape of the ND-array.
  pub shape: TiNdShape,
  /// Shape of the ND-array elements. It *must not* be empty for vector or matrix ND-arrays.
  pub elem_shape: TiNdShape,
  /// Primitive data type of the ND-array elements.
  pub elem_type: TiDataType,
}

bitflags! {
/// BitField `TiImageUsageFlags`
/// 
/// Usages of an image allocation. Taichi requires kernel argument images to be allocated with `TI_IMAGE_USAGE_STORAGE_BIT` and `TI_IMAGE_USAGE_SAMPLED_BIT`.
#[repr(transparent)]
pub struct TiImageUsageFlags: u32 {
  /// The image can be read/write accessed by any kernel.
  const STORAGE_BIT = 1 << 0;
  /// The image can be read-only accessed by any kernel.
  const SAMPLED_BIT = 1 << 1;
  /// The image can be used as a color or depth-stencil attachment depending on its format.
  const ATTACHMENT_BIT = 1 << 2;
}
}

/// Enumeration `TiImageDimension`
/// 
/// Dimensions of an image allocation.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TiImageDimension {
  /// The image is 1-dimensional.
  D1D = 0,
  /// The image is 2-dimensional.
  D2D = 1,
  /// The image is 3-dimensional.
  D3D = 2,
  /// The image is 1-dimensional and it has one or more layers.
  D1DArray = 3,
  /// The image is 2-dimensional and it has one or more layers.
  D2DArray = 4,
  Cube = 5,
}

/// Enumeration `TiImageLayout`
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// Enumeration `TiFormat`
/// 
/// Texture formats. The availability of texture formats depends on runtime support.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// Structure `TiImageOffset`
/// 
/// Offsets of an image in X, Y, Z, and array layers.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiImageOffset {
  /// Image offset in the X direction.
  pub x: u32,
  /// Image offset in the Y direction. *Must* be 0 if the image has a dimension of `enumeration.image_dimension.1d` or `enumeration.image_dimension.1d_array`.
  pub y: u32,
  /// Image offset in the Z direction. *Must* be 0 if the image has a dimension of `enumeration.image_dimension.1d`, `enumeration.image_dimension.2d`, `enumeration.image_dimension.1d_array`, `enumeration.image_dimension.2d_array` or `enumeration.image_dimension.cube_array`.
  pub z: u32,
  /// Image offset in array layers. *Must* be 0 if the image has a dimension of `enumeration.image_dimension.1d`, `enumeration.image_dimension.2d` or `enumeration.image_dimension.3d`.
  pub array_layer_offset: u32,
}

/// Structure `TiImageExtent`
/// 
/// Extents of an image in X, Y, Z, and array layers.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiImageExtent {
  /// Image extent in the X direction.
  pub width: u32,
  /// Image extent in the Y direction. *Must* be 1 if the image has a dimension of `enumeration.image_dimension.1d` or `enumeration.image_dimension.1d_array`.
  pub height: u32,
  /// Image extent in the Z direction. *Must* be 1 if the image has a dimension of `enumeration.image_dimension.1d`, `enumeration.image_dimension.2d`, `enumeration.image_dimension.1d_array`, `enumeration.image_dimension.2d_array` or `enumeration.image_dimension.cube_array`.
  pub depth: u32,
  /// Image extent in array layers. *Must* be 1 if the image has a dimension of `enumeration.image_dimension.1d`, `enumeration.image_dimension.2d` or `enumeration.image_dimension.3d`. *Must* be 6 if the image has a dimension of `enumeration.image_dimension.cube_array`.
  pub array_layer_count: u32,
}

/// Structure `TiImageAllocateInfo`
/// 
/// Parameters of a newly allocated image.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiImageAllocateInfo {
  /// Image dimension.
  pub dimension: TiImageDimension,
  /// Image extent.
  pub extent: TiImageExtent,
  /// Number of mip-levels.
  pub mip_level_count: u32,
  /// Image texel format.
  pub format: TiFormat,
  /// True if the memory allocation needs to be exported to other backends (e.g., from Vulkan to CUDA).
  pub export_sharing: TiBool,
  /// All possible usages of this image allocation. In most cases, `bit_field.image_usage.storage` and `bit_field.image_usage.sampled` enough.
  pub usage: TiImageUsageFlags,
}

/// Structure `TiImageSlice`
/// 
/// A subsection of a memory allocation. The sum of `offset` and `extent` in each dimension cannot exceed the size of `image`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiImageSlice {
  /// The subsectioned image allocation.
  pub image: TiImage,
  /// Offset from the beginning of the allocation in each dimension.
  pub offset: TiImageOffset,
  /// Size of the subsection in each dimension.
  pub extent: TiImageExtent,
  /// The subsectioned mip-level.
  pub mip_level: u32,
}

/// Enumeration `TiFilter`
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TiFilter {
  Nearest = 0,
  Linear = 1,
}

/// Enumeration `TiAddressMode`
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TiAddressMode {
  Repeat = 0,
  MirroredRepeat = 1,
  ClampToEdge = 2,
}

/// Structure `TiSamplerCreateInfo`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiSamplerCreateInfo {
  pub mag_filter: TiFilter,
  pub min_filter: TiFilter,
  pub address_mode: TiAddressMode,
  pub max_anisotropy: f32,
}

/// Structure `TiTexture`
/// 
/// Image data bound to a sampler.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiTexture {
  /// Image bound to the texture.
  pub image: TiImage,
  /// The bound sampler that controls the sampling behavior of `structure.texture.image`.
  pub sampler: TiSampler,
  /// Image Dimension.
  pub dimension: TiImageDimension,
  /// Image extent.
  pub extent: TiImageExtent,
  /// Image texel format.
  pub format: TiFormat,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union TiScalarValue {
  /// Scalar value that fits into 8 bits.
  pub x8: u8,
  /// Scalar value that fits into 16 bits.
  pub x16: u16,
  /// Scalar value that fits into 32 bits.
  pub x32: u32,
  /// Scalar value that fits into 64 bits.
  pub x64: u64,
}

/// Structure `TiScalar`
/// 
/// A typed scalar value.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiScalar {
  pub r#type: TiDataType,
  pub value: TiScalarValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union TiArgumentValue {
  /// Value of a 32-bit one's complement signed integer. This is equivalent to `union.scalar_value.x32` with `enumeration.data_type.i32`.
  pub r#i32: i32,
  /// Value of a 32-bit IEEE 754 single-precision floating-poing number. This is equivalent to `union.scalar_value.x32` with `enumeration.data_type.f32`.
  pub r#f32: f32,
  /// An ND-array to be bound.
  pub ndarray: TiNdArray,
  /// A texture to be bound.
  pub texture: TiTexture,
  /// An scalar to be bound.
  pub scalar: TiScalar,
}

/// Structure `TiArgument`
/// 
/// An argument value to feed kernels.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiArgument {
  /// Type of the argument.
  pub r#type: TiArgumentType,
  /// Value of the argument.
  pub value: TiArgumentValue,
}

/// Structure `TiNamedArgument`
/// 
/// A named argument value to feed compute graphs.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiNamedArgument {
  /// Name of the argument.
  pub name: *const c_char,
  /// Argument body.
  pub argument: TiArgument,
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_get_version`
/// 
/// Get the current taichi version. It has the same value as `TI_C_API_VERSION` as defined in `taichi_core.h`.
pub fn ti_get_version(
) -> u32;
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_get_available_archs`
/// 
/// Gets a list of available archs on the current platform. An arch is only available if:
/// 
/// 1. The Runtime library is compiled with its support;
/// 2. The current platform is installed with a capable hardware or an emulation software.
/// 
/// An available arch has at least one device available, i.e., device index 0 is always available. If an arch is not available on the current platform, a call to [`ti_create_runtime`](#function-ti_create_runtime) with that arch is guaranteed failing.
/// 
/// **WARNING** Please also note that the order or returned archs is *undefined*.
pub fn ti_get_available_archs(
  arch_count: *mut u32,
  archs: *mut TiArch,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_get_last_error`
/// 
/// Gets the last error raised by Taichi C-API invocations. Returns the semantical error code.
///
/// Parameters:
/// - `message_size`: Size of textual error message in `function.get_last_error.message`
/// - `message`: Text buffer for the textual error message. Ignored when `message_size` is 0.
pub fn ti_get_last_error(
  message_size: *mut u64,
  message: *mut c_char,
) -> TiError;
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_set_last_error`
/// 
/// Sets the provided error as the last error raised by Taichi C-API invocations. It can be useful in extended validation procedures in Taichi C-API wrappers and helper libraries.
///
/// Parameters:
/// - `error`: Semantical error code.
/// - `message`: A null-terminated string of the textual error message or `nullptr` for empty error message.
pub fn ti_set_last_error(
  error: TiError,
  message: *const c_char,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_create_runtime`
/// 
/// Creates a Taichi Runtime with the specified [`TiArch`](#enumeration-tiarch).
///
/// Parameters:
/// - `arch`: Arch of Taichi Runtime.
/// - `device_index`: The index of device in `function.create_runtime.arch` to create Taichi Runtime on.
pub fn ti_create_runtime(
  arch: TiArch,
  device_index: u32,
) -> TiRuntime;
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_destroy_runtime`
/// 
/// Destroys a Taichi Runtime.
pub fn ti_destroy_runtime(
  runtime: TiRuntime,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_set_runtime_capabilities_ext`
/// 
/// Force override the list of available capabilities in the runtime instance.
pub fn ti_set_runtime_capabilities_ext(
  runtime: TiRuntime,
  capability_count: u32,
  capabilities: *const TiCapabilityLevelInfo,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_get_runtime_capabilities`
/// 
/// Gets all capabilities available on the runtime instance.
///
/// Parameters:
/// - `capability_count`: The total number of capabilities available.
/// - `capabilities`: Returned capabilities.
pub fn ti_get_runtime_capabilities(
  runtime: TiRuntime,
  capability_count: *mut u32,
  capabilities: *mut TiCapabilityLevelInfo,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_allocate_memory`
/// 
/// Allocates a contiguous device memory with provided parameters.
pub fn ti_allocate_memory(
  runtime: TiRuntime,
  allocate_info: *const TiMemoryAllocateInfo,
) -> TiMemory;
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_free_memory`
/// 
/// Frees a memory allocation.
pub fn ti_free_memory(
  runtime: TiRuntime,
  memory: TiMemory,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_map_memory`
/// 
/// Maps a device memory to a host-addressable space. You *must* ensure that the device is not being used by any device command before the mapping.
pub fn ti_map_memory(
  runtime: TiRuntime,
  memory: TiMemory,
) -> *mut c_void;
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_unmap_memory`
/// 
/// Unmaps a device memory and makes any host-side changes about the memory visible to the device. You *must* ensure that there is no further access to the previously mapped host-addressable space.
pub fn ti_unmap_memory(
  runtime: TiRuntime,
  memory: TiMemory,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_allocate_image`
/// 
/// Allocates a device image with provided parameters.
pub fn ti_allocate_image(
  runtime: TiRuntime,
  allocate_info: *const TiImageAllocateInfo,
) -> TiImage;
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_free_image`
/// 
/// Frees an image allocation.
pub fn ti_free_image(
  runtime: TiRuntime,
  image: TiImage,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_create_sampler`
pub fn ti_create_sampler(
  runtime: TiRuntime,
  create_info: *const TiSamplerCreateInfo,
) -> TiSampler;
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_destroy_sampler`
pub fn ti_destroy_sampler(
  runtime: TiRuntime,
  sampler: TiSampler,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_copy_memory_device_to_device` (Device Command)
/// 
/// Copies the data in a contiguous subsection of the device memory to another subsection. The two subsections *must not* overlap.
pub fn ti_copy_memory_device_to_device(
  runtime: TiRuntime,
  dst_memory: *const TiMemorySlice,
  src_memory: *const TiMemorySlice,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_copy_image_device_to_device` (Device Command)
/// 
/// Copies the image data in a contiguous subsection of the device image to another subsection. The two subsections *must not* overlap.
pub fn ti_copy_image_device_to_device(
  runtime: TiRuntime,
  dst_image: *const TiImageSlice,
  src_image: *const TiImageSlice,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_track_image_ext`
/// 
/// Tracks the device image with the provided image layout. Because Taichi tracks image layouts internally, it is *only* useful to inform Taichi that the image is transitioned to a new layout by external procedures.
pub fn ti_track_image_ext(
  runtime: TiRuntime,
  image: TiImage,
  layout: TiImageLayout,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_transition_image` (Device Command)
/// 
/// Transitions the image to the provided image layout. Because Taichi tracks image layouts internally, it is *only* useful to enforce an image layout for external procedures to use.
pub fn ti_transition_image(
  runtime: TiRuntime,
  image: TiImage,
  layout: TiImageLayout,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_launch_kernel` (Device Command)
/// 
/// Launches a Taichi kernel with the provided arguments. The arguments *must* have the same count and types in the same order as in the source code.
pub fn ti_launch_kernel(
  runtime: TiRuntime,
  kernel: TiKernel,
  arg_count: u32,
  args: *const TiArgument,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_launch_compute_graph` (Device Command)
/// 
/// Launches a Taichi compute graph with provided named arguments. The named arguments *must* have the same count, names, and types as in the source code.
pub fn ti_launch_compute_graph(
  runtime: TiRuntime,
  compute_graph: TiComputeGraph,
  arg_count: u32,
  args: *const TiNamedArgument,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_flush`
/// 
/// Submits all previously invoked device commands to the offload device for execution.
pub fn ti_flush(
  runtime: TiRuntime,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_wait`
/// 
/// Waits until all previously invoked device commands are executed. Any invoked command that has not been submitted is submitted first.
pub fn ti_wait(
  runtime: TiRuntime,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_load_aot_module`
/// 
/// Loads a pre-compiled AOT module from the file system.
/// Returns [`TI_NULL_HANDLE`](#definition-ti_null_handle) if the runtime fails to load the AOT module from the specified path.
pub fn ti_load_aot_module(
  runtime: TiRuntime,
  module_path: *const c_char,
) -> TiAotModule;
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_create_aot_module`
/// 
/// Creates a pre-compiled AOT module from TCM data.
/// Returns [`TI_NULL_HANDLE`](#definition-ti_null_handle) if the runtime fails to create the AOT module from TCM data.
pub fn ti_create_aot_module(
  runtime: TiRuntime,
  tcm: *const c_void,
  size: u64,
) -> TiAotModule;
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_destroy_aot_module`
/// 
/// Destroys a loaded AOT module and releases all related resources.
pub fn ti_destroy_aot_module(
  aot_module: TiAotModule,
) -> ();
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_get_aot_module_kernel`
/// 
/// Retrieves a pre-compiled Taichi kernel from the AOT module.
/// Returns [`TI_NULL_HANDLE`](#definition-ti_null_handle) if the module does not have a kernel of the specified name.
pub fn ti_get_aot_module_kernel(
  aot_module: TiAotModule,
  name: *const c_char,
) -> TiKernel;
}

#[link(name = "taichi_c_api")]
extern "C" {
/// Function `ti_get_aot_module_compute_graph`
/// 
/// Retrieves a pre-compiled compute graph from the AOT module.
/// Returns [`TI_NULL_HANDLE`](#definition-ti_null_handle) if the module does not have a compute graph of the specified name.
pub fn ti_get_aot_module_compute_graph(
  aot_module: TiAotModule,
  name: *const c_char,
) -> TiComputeGraph;
}
