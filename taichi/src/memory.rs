use std::rc::Rc;
use taichi_sys::{TiMemoryAllocateInfo, TI_FALSE, TiMemoryUsageFlags, TI_TRUE, TiMemory, ti_allocate_memory, ti_free_memory, TiRuntime, ti_map_memory, ti_unmap_memory};

use crate::{get_last_error, Error, Result, Runtime};

pub struct MemoryBuilder<'a> {
    runtime: &'a Runtime,
    allocate_info: TiMemoryAllocateInfo,
}
impl<'a> MemoryBuilder<'a> {
    pub fn new(runtime: &'a Runtime) -> Self {
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

struct Memory_ {
    runtime: Runtime,
    memory: TiMemory,
    allocate_info: TiMemoryAllocateInfo,
}
impl Memory_ {
    pub fn new(runtime: &Runtime, allocate_info: &TiMemoryAllocateInfo) -> Result<Self> {
        let memory = unsafe {
            ti_allocate_memory(runtime.runtime(), allocate_info)
        };
        get_last_error()?;
        let out = Memory_ {
            runtime: runtime.clone(),
            memory,
            allocate_info: allocate_info.clone()
        };
        Ok(out)
    }
}
impl Drop for Memory_ {
    fn drop(&mut self) {
        unsafe {
            ti_free_memory(self.runtime.runtime(), self.memory);
        }
    }
}

#[derive(Clone)]
pub struct Memory {
    inner: Rc<Memory_>,
}
impl Memory {
    pub fn new(runtime: &Runtime, allocate_info: &TiMemoryAllocateInfo) -> Result<Self> {
        let inner = Memory_::new(runtime, allocate_info)?;
        let out = Memory {
            inner: Rc::new(inner),
        };
        Ok(out)
    }

    pub fn map<T>(&self) -> Result<MappedMemory<'_, T>> {
        MappedMemory::new(self)
    }

    pub fn read<T: Clone>(&self, dst: &mut [T]) -> Result<()> {
        if !self.host_read() {
            return Err(Error::InvalidState("attempting to map non-host-readable memory"));
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
            return Err(Error::InvalidState("attempting to map non-host-writable memory"));
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
        self.inner.runtime.runtime()
    }
    pub fn memory(&self) -> TiMemory {
        self.inner.memory
    }

    pub fn size(&self) -> usize {
        self.inner.allocate_info.size as usize
    }
    pub fn host_read(&self) -> bool {
        self.inner.allocate_info.host_read != 0
    }
    pub fn host_write(&self) -> bool {
        self.inner.allocate_info.host_write != 0
    }
    pub fn usage(&self) -> TiMemoryUsageFlags {
        self.inner.allocate_info.usage
    }
}

pub struct MappedMemory<'a, T>(&'a Memory, *mut T);
impl<'a, T> MappedMemory<'a, T> {
    pub fn new(memory: &'a Memory) -> Result<Self> {
        debug_assert!(memory.size() % std::mem::size_of::<T>() == 0);
        let mapped = unsafe {
            ti_map_memory(memory.runtime(), memory.memory()) as *mut T
        };
        get_last_error()?;
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
