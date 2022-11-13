use std::collections::HashMap;
use std::ffi::{CString};
use std::{rc::Rc};
use std::marker::PhantomData;

use taichi_sys::*;

mod error;

use crate::error::{TaichiResult as Result};


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

pub struct Runtime_ {
    arch: TiArch,
    runtime: TiRuntime,
}
impl Runtime_ {
    fn new(arch: TiArch) -> Result<Self> {
        let runtime = unsafe {
            ti_create_runtime(arch)
        };
        check_taichi_error()?;
        Ok(Runtime_ { arch, runtime })
    }
}
impl Drop for Runtime_ {
    fn drop(&mut self) {
        unsafe {
            ti_destroy_runtime(self.runtime)
        };
    }
}
#[derive(Clone)]
pub struct Runtime(Rc<Runtime_>);
impl Runtime {
    pub fn new(arch: TiArch) -> Result<Self> {
        let out = Runtime_::new(arch)?;
        Ok(Runtime(Rc::new(out)))
    }

    pub fn arch(&self) -> TiArch {
        self.0.arch
    }
    pub fn runtime(&self) -> TiRuntime {
        self.0.runtime
    }

    pub fn allocate_memory(&self) -> MemoryBuilder<'_> {
        MemoryBuilder::new(self)
    }
    pub fn allocate_ndarray<T>(&self) -> NdArrayBuilder<'_, T> {
        NdArrayBuilder::<T>::new(self)
    }

    pub fn load_aot_module(&self, module_dir: &str) -> Result<AotModule> {
        AotModule::new(self, module_dir)
    }

    pub fn wait(&self) -> Result<()> {
        unsafe {
            ti_wait(self.0.runtime);
        }
        check_taichi_error()?;
        Ok(())
    }

}

pub struct MemoryBuilder<'a> {
    runtime: &'a Runtime,
    allocate_info: TiMemoryAllocateInfo,
}
impl<'a> MemoryBuilder<'a> {
    fn new(runtime: &'a Runtime) -> Self {
        let allocate_info = TiMemoryAllocateInfo {
            size: 0,
            host_write: TI_FALSE,
            host_read: TI_FALSE,
            export_sharing: TI_FALSE,
            usage: TiMemoryUsageFlags::STORAGE_BIT
        };
        MemoryBuilder {
            runtime,
            allocate_info,
        }
    }
    pub fn size(&mut self, size: usize) -> &mut Self {
        self.allocate_info.size = size as u64;
        self
    }
    pub fn host_read(&mut self, value: bool) -> &mut Self {
        if value {
            self.allocate_info.host_read = TI_TRUE;
        } else {
            self.allocate_info.host_read = TI_FALSE;
        }
        self
    }
    pub fn host_write(&mut self, value: bool) -> &mut Self {
        if value {
            self.allocate_info.host_write = TI_TRUE;
        } else {
            self.allocate_info.host_write = TI_FALSE;
        }
        self
    }
    pub fn usage(&mut self, usage: TiMemoryUsageFlags) -> &mut Self {
        self.allocate_info.usage = usage;
        self
    }

    pub fn build(&mut self) -> Result<Memory> {
        Memory::new(self.runtime, &self.allocate_info)
    }
}

pub struct Memory_ {
    runtime: Rc<Runtime_>,
    memory: TiMemory,
    allocate_info: TiMemoryAllocateInfo,
}
impl Memory_ {
    fn new(runtime: &Runtime, allocate_info: &TiMemoryAllocateInfo) -> Result<Self> {
        let memory = unsafe {
            ti_allocate_memory(runtime.runtime(), allocate_info)
        };
        check_taichi_error()?;
        Ok(Memory_ { runtime: runtime.0.clone(), memory, allocate_info: allocate_info.clone() })
    }
}
impl Drop for Memory_ {
    fn drop(&mut self) {
        unsafe { ti_free_memory(self.runtime.runtime, self.memory) };
    }
}
#[derive(Clone)]
pub struct Memory(Rc<Memory_>);
impl Memory {
    fn new(runtime: &Runtime, allocate_info: &TiMemoryAllocateInfo) -> Result<Memory> {
        let out = Memory_::new(runtime, allocate_info)?;
        Ok(Memory(Rc::new(out)))
    }

    pub fn map<T>(&self) -> Result<MappedMemory<'_, T>> {
        MappedMemory::new(self)
    }

    pub fn read<T: Clone>(&self, dst: &mut [T]) -> Result<()> {
        if !self.host_read() {
            return Err(TiError::InvalidState);
        }
        let mapped = MappedMemory::new(self)?;
        let len = self.size() as usize / std::mem::size_of::<T>();
        debug_assert_eq!(dst.len(), len);
        let src = unsafe {
            std::slice::from_raw_parts(mapped.ptr(), len)
        };
        dst.clone_from_slice(src);
        Ok(())
    }
    pub fn write<T: Clone>(&self, src: &[T]) -> Result<()> {
        if !self.host_write() {
            return Err(TiError::InvalidState);
        }
        let mapped = MappedMemory::<T>::new(self)?;
        let len = self.size() / std::mem::size_of::<T>();
        debug_assert_eq!(src.len(), len);
        let dst = unsafe {
            std::slice::from_raw_parts_mut(mapped.ptr_mut(), len)
        };
        dst.clone_from_slice(src);
        Ok(())
    }

    pub fn runtime(&self) -> TiRuntime {
        self.0.runtime.runtime
    }
    pub fn memory(&self) -> TiMemory {
        self.0.memory
    }

    pub fn size(&self) -> usize {
        self.0.allocate_info.size as usize
    }
    pub fn host_read(&self) -> bool {
        self.0.allocate_info.host_read != 0
    }
    pub fn host_write(&self) -> bool {
        self.0.allocate_info.host_write != 0
    }
    pub fn usage(&self) -> TiMemoryUsageFlags {
        self.0.allocate_info.usage
    }
}

pub struct MappedMemory<'a, T>(&'a Memory, *mut T);
impl<'a, T> MappedMemory<'a, T> {
    fn new(memory: &'a Memory) -> Result<Self> {
        debug_assert!(memory.size() % std::mem::size_of::<T>() == 0);
        let mapped = unsafe {
            ti_map_memory(memory.runtime(), memory.memory()) as *mut T
        };
        check_taichi_error()?;
        Ok(MappedMemory(memory, mapped))
    }

    pub fn memory(&self) -> &Memory {
        self.0
    }
    pub fn ptr(&self) -> *const T {
        self.1
    }
    pub fn ptr_mut(&self) -> *mut T {
        self.1
    }
}
impl<'a, T> Drop for MappedMemory<'a, T> {
    fn drop(&mut self) {
        unsafe {
            ti_unmap_memory(self.memory().runtime(), self.memory().memory());
        }
    }
}

pub struct NdArrayBuilder<'a, T> {
    memory_builder: MemoryBuilder<'a>,
    ndarray: TiNdArray,
    phantom: PhantomData<T>,
}
impl<'a, T> NdArrayBuilder<'a, T> {
    pub fn new(runtime: &'a Runtime) -> NdArrayBuilder<'a, T> {
        NdArrayBuilder {
            memory_builder: runtime.allocate_memory(),
            ndarray: TiNdArray {
                memory: TiMemory::null(),
                shape: TiNdShape { dim_count: 0, dims: [0; 16] },
                elem_shape: TiNdShape { dim_count: 0, dims: [0; 16] },
                elem_type: TiDataType::Unknown,
            },
            phantom: Default::default(),
        }
    }

    pub fn host_read(&mut self, value: bool) -> &mut Self {
        self.memory_builder.host_read(value);
        self
    }
    pub fn host_write(&mut self, value: bool) -> &mut Self {
        self.memory_builder.host_write(value);
        self
    }
    pub fn usage(&mut self, usage: TiMemoryUsageFlags) -> &mut Self {
        self.memory_builder.usage(usage);
        self
    }
    pub fn shape<S: AsRef<[u32]>>(&mut self, shape: S) -> &mut Self {
        let shape = shape.as_ref();
        self.ndarray.shape.dim_count = shape.len() as u32;
        for (i, x) in shape.iter().take(16).enumerate() {
            self.ndarray.shape.dims[i] = *x;
        }
        self
    }
    pub fn elem_shape<S: AsRef<[u32]>>(&mut self, elem_shape: S) -> &mut Self {
        let elem_shape = elem_shape.as_ref();
        self.ndarray.elem_shape.dim_count = elem_shape.len() as u32;
        for (i, x) in elem_shape.iter().take(16).enumerate() {
            self.ndarray.elem_shape.dims[i] = *x;
        }
        self
    }

    fn build_impl(&mut self, elem_type: TiDataType) -> Result<NdArray<T>> {
        let mut size = std::mem::size_of::<T>();
        for i in 0..self.ndarray.shape.dim_count as usize {
            size *= self.ndarray.shape.dims[i] as usize;
        }
        for i in 0..self.ndarray.elem_shape.dim_count as usize {
            size *= self.ndarray.elem_shape.dims[i] as usize;
        }
        let memory = self.memory_builder
            .size(size)
            .build()?;
        self.ndarray.elem_type = elem_type;
        self.ndarray.memory = memory.memory();
        NdArray::<T>::new(memory, self.ndarray)
    }
}
impl<'a> NdArrayBuilder<'a, f32> {
    pub fn build(&mut self) -> Result<NdArray<f32>> {
        self.build_impl(TiDataType::F32)
    }
}
impl<'a> NdArrayBuilder<'a, f64> {
    pub fn build(&mut self) -> Result<NdArray<f64>> {
        self.build_impl(TiDataType::F64)
    }
}
impl<'a> NdArrayBuilder<'a, i8> {
    pub fn build(&mut self) -> Result<NdArray<i8>> {
        self.build_impl(TiDataType::I8)
    }
}
impl<'a> NdArrayBuilder<'a, i16> {
    pub fn build(&mut self) -> Result<NdArray<i16>> {
        self.build_impl(TiDataType::I16)
    }
}
impl<'a> NdArrayBuilder<'a, i32> {
    pub fn build(&mut self) -> Result<NdArray<i32>> {
        self.build_impl(TiDataType::I32)
    }
}
impl<'a> NdArrayBuilder<'a, i64> {
    pub fn build(&mut self) -> Result<NdArray<i64>> {
        self.build_impl(TiDataType::I64)
    }
}
impl<'a> NdArrayBuilder<'a, u8> {
    pub fn build(&mut self) -> Result<NdArray<u8>> {
        self.build_impl(TiDataType::U8)
    }
}
impl<'a> NdArrayBuilder<'a, u16> {
    pub fn build(&mut self) -> Result<NdArray<u16>> {
        self.build_impl(TiDataType::U16)
    }
}
impl<'a> NdArrayBuilder<'a, u32> {
    pub fn build(&mut self) -> Result<NdArray<u32>> {
        self.build_impl(TiDataType::U32)
    }
}
impl<'a> NdArrayBuilder<'a, u64> {
    pub fn build(&mut self) -> Result<NdArray<u64>> {
        self.build_impl(TiDataType::U64)
    }
}

pub struct NdArray<T> {
    memory: Memory,
    ndarray: TiNdArray,
    phantom: PhantomData<T>,
}
impl<T> NdArray<T> {
    fn new(
        memory: Memory,
        ndarray: TiNdArray,
    ) -> Result<NdArray<T>> {
        debug_assert_eq!(memory.memory(), ndarray.memory);
        Ok(NdArray { memory, ndarray, phantom: Default::default() })
    }

    pub fn map(&self) -> Result<MappedMemory<'_, T>> {
        self.memory.map()
    }

    pub fn read<U: Clone>(&self, dst: &mut [U]) -> Result<()> {
        self.memory.read(dst)
    }
    pub fn write<U: Clone>(&self, src: &[U]) -> Result<()> {
        self.memory.write(src)
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }
    pub fn shape(&self) -> &[u32] {
        let n = self.ndarray.shape.dim_count as usize;
        &self.ndarray.shape.dims[..n]
    }
    pub fn elem_shape(&self) -> &[u32] {
        let n = self.ndarray.elem_shape.dim_count as usize;
        &self.ndarray.elem_shape.dims[..n]
    }
    pub fn elem_type(&self) -> TiDataType {
        self.ndarray.elem_type
    }
    pub fn ndarray(&self) -> &TiNdArray {
        &self.ndarray
    }
}

struct AotModule_ {
    runtime: Rc<Runtime_>,
    module: TiAotModule,
}
impl AotModule_ {
    pub fn new(runtime: &Runtime, module_dir: &str) -> Result<AotModule_> {
        let module_dir = CString::new(module_dir)
            .map_err(|_| TiError::InvalidArgument)?;
        let module = unsafe {
            ti_load_aot_module(runtime.runtime(), module_dir.as_ptr())
        };
        check_taichi_error()?;
        Ok(AotModule_ { runtime: runtime.0.clone(), module })
    }
}
impl Drop for AotModule_ {
    fn drop(&mut self) {
        unsafe {
            ti_destroy_aot_module(self.module);
        }
    }
}

pub struct AotModule(Rc<AotModule_>);
impl AotModule {
    fn new(runtime: &Runtime, module_dir: &str) -> Result<AotModule> {
        let out = AotModule_::new(runtime, module_dir)?;
        Ok(AotModule(Rc::new(out)))
    }

    pub fn get_compute_graph(&self, name: &str) -> Result<ComputeGraph> {
        ComputeGraph::new(self, name)
    }

    pub fn aot_module(&self) -> TiAotModule {
        self.0.module
    }
}


struct ComputeGraph_ {
    aot_module: Rc<AotModule_>,
    compute_graph: TiComputeGraph,
}
impl ComputeGraph_ {
    fn new(aot_module: &AotModule, name: &str) -> Result<ComputeGraph_> {
        let name = CString::new(name)
            .map_err(|_| TiError::InvalidArgument)?;
        let compute_graph = unsafe {
            ti_get_aot_module_compute_graph(aot_module.aot_module(), name.as_ptr())
        };
        check_taichi_error()?;
        Ok(ComputeGraph_ { aot_module: aot_module.0.clone(), compute_graph })
    }
}

pub struct ComputeGraph {
    compute_graph: Rc<ComputeGraph_>,
    args: HashMap<CString, TiArgument>,
}
impl ComputeGraph {
    pub fn new(aot_module: &AotModule, name: &str) -> Result<ComputeGraph> {
        let out = ComputeGraph_::new(aot_module, name)?;
        Ok(ComputeGraph { compute_graph: Rc::new(out), args: Default::default() })
    }

    pub fn set_arg_i32(&mut self, name: &str, value: i32) -> Result<&mut Self> {
        let name = CString::new(name)
            .map_err(|_| TiError::InvalidArgument)?;
        let arg = TiArgument {
            r#type: TiArgumentType::I32,
            value: TiArgumentValue {
                r#i32: value,
            },
        };
        self.args.insert(name, arg);
        Ok(self)
    }
    pub fn set_arg_f32(&mut self, name: &str, value: f32) -> Result<&mut Self> {
        let name = CString::new(name)
            .map_err(|_| TiError::InvalidArgument)?;
        let arg = TiArgument {
            r#type: TiArgumentType::F32,
            value: TiArgumentValue {
                r#f32: value,
            },
        };
        self.args.insert(name, arg);
        Ok(self)
    }
    pub fn set_arg_ndarray<T>(&mut self, name: &str, value: &NdArray<T>) -> Result<&mut Self> {
        let name = CString::new(name)
            .map_err(|_| TiError::InvalidArgument)?;
        let arg = TiArgument {
            r#type: TiArgumentType::Ndarray,
            value: TiArgumentValue {
                ndarray: value.ndarray().clone(),
            },
        };
        self.args.insert(name, arg);
        Ok(self)
    }

    pub fn launch(&self) -> Result<()> {
        let mut args = Vec::with_capacity(self.args.len());

        for (name, argument) in self.args.iter() {
            let arg = TiNamedArgument {
                name: name.as_ptr(),
                argument: argument.clone(),
            };
            args.push(arg);
        }

        let runtime = self.compute_graph.aot_module.runtime.runtime;
        let compute_graph = self.compute_graph.compute_graph;
        unsafe {
            ti_launch_compute_graph(runtime, compute_graph, args.len() as u32, args.as_ptr());
        }
        check_taichi_error()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_runtime() {
        Runtime::new(TiArch::Vulkan).unwrap();
    }
    #[test]
    fn test_host_accessible_memory_read_write() {
        let runtime = Runtime::new(TiArch::Vulkan).unwrap();
        let memory = runtime.allocate_memory()
            .size(128 * std::mem::size_of::<u32>())
            .host_read(true)
            .host_write(true)
            .build()
            .unwrap();

        let input = (0..128).into_iter().collect::<Vec<u32>>();
        memory.write(&input).unwrap();
        let mut output = [0u32; 128].to_vec();
        memory.read(&mut output).unwrap();
        assert_eq!(input, output);
    }
    #[test]
    fn test_host_accessible_ndarray_read_write() {
        let runtime = Runtime::new(TiArch::Vulkan).unwrap();
        let ndarray = runtime.allocate_ndarray::<u32>()
            .shape([128, ])
            .host_read(true)
            .host_write(true)
            .build()
            .unwrap();

        let input = (0..128).into_iter().collect::<Vec<u32>>();
        ndarray.write(&input).unwrap();
        let mut output = [0u32; 128].to_vec();
        ndarray.read(&mut output).unwrap();
        assert_eq!(input, output);
    }
    #[test]
    fn test_load_aot_module() {
        let runtime = Runtime::new(TiArch::Vulkan).unwrap();
        let ndarray = runtime.allocate_ndarray::<i32>()
            .shape([16, 16])
            .host_read(true)
            .build()
            .unwrap();
        let module = runtime.load_aot_module("../tmp/module").unwrap();
        let mut g_run = module.get_compute_graph("g_run").unwrap();
        g_run.set_arg_ndarray("arr", &ndarray).unwrap();
        g_run.launch().unwrap();
        runtime.wait().unwrap();

        let mut expect_data = Vec::new();
        for i in 0..16 {
            for j in 0..16 {
                let x = (j * (16 + 1) + i) % 2;
                expect_data.push(x);
            }
        }
        let mut actual_data = [0; 16 * 16].to_vec();
        ndarray.read(&mut actual_data).unwrap();
        assert_eq!(expect_data, actual_data);
    }

}
