use std::marker::PhantomData;
use taichi_sys::*;
use crate::{
    Result,
    runtime::{Runtime},
    memory::{MappedMemory, Memory, MemoryBuilder},
};

pub struct NdArrayBuilder<'a, T> {
    memory_builder: MemoryBuilder<'a>,
    ndarray: TiNdArray,
    phantom: PhantomData<T>,
}
impl<'a, T> NdArrayBuilder<'a, T> {
    pub fn new(runtime: &'a Runtime) -> Self {
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
    pub fn host_access(&mut self, value: bool) -> &mut Self {
        self.host_read(value)
            .host_write(value)
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

    pub fn to_vec<U: Clone + Default>(&self) -> Result<Vec<U>> {
        let n = self.memory.size() as usize / std::mem::size_of::<U>();
        let mut out = Vec::new();
        out.resize(n, U::default());
        self.read(&mut out)?;
        Ok(out)
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
    pub fn elem_count(&self) -> usize {
        let mut elem_count: usize = 1;
        let shape = &self.ndarray.shape;
        for i in shape.dims[..shape.dim_count as usize].iter() {
            elem_count *= *i as usize;
        }
        elem_count
    }
    pub fn scalar_count(&self) -> usize {
        let mut scalar_count: usize = self.elem_count();
        let elem_shape = &self.ndarray.elem_shape;
        for i in elem_shape.dims[..elem_shape.dim_count as usize].iter() {
            scalar_count *= *i as usize;
        }
        scalar_count
    }
    pub fn elem_type(&self) -> TiDataType {
        self.ndarray.elem_type
    }
    pub fn ndarray(&self) -> &TiNdArray {
        &self.ndarray
    }
}
