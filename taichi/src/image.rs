use std::rc::Rc;
use taichi_sys::{TiImageAllocateInfo, TiImageDimension, TiImageExtent, TiFormat, TiImageUsageFlags, TI_TRUE, TiImage, ti_allocate_image, ti_free_image, TiRuntime, TI_FALSE};

use crate::{get_last_error, Result, Runtime};

pub struct ImageBuilder<'a> {
    runtime: &'a Runtime,
    allocate_info: TiImageAllocateInfo,
}
impl<'a> ImageBuilder<'a> {
    pub fn new(runtime: &'a Runtime) -> Self {
        let allocate_info = TiImageAllocateInfo {
            dimension: TiImageDimension::D2D,
            extent: TiImageExtent {
                width: 1,
                height: 1,
                depth: 1,
                array_layer_count: 1,
            },
            mip_level_count: 1,
            format: TiFormat::Rgba8,
            export_sharing: TI_FALSE,
            usage: TiImageUsageFlags::SAMPLED_BIT | TiImageUsageFlags::STORAGE_BIT,
        };
        ImageBuilder {
            runtime,
            allocate_info,
        }
    }

    pub fn dimension(&mut self, dimension: TiImageDimension) -> &mut Self {
        self.allocate_info.dimension = dimension;
        self
    }
    pub fn width(&mut self, width: usize) -> &mut Self {
        self.allocate_info.extent.width = width as u32;
        self
    }
    pub fn height(&mut self, height: usize) -> &mut Self {
        self.allocate_info.extent.height = height as u32;
        self
    }
    pub fn depth(&mut self, depth: usize) -> &mut Self {
        self.allocate_info.extent.depth = depth as u32;
        self
    }
    pub fn array_layer_count(&mut self, array_layer_count: usize) -> &mut Self {
        self.allocate_info.extent.array_layer_count = array_layer_count as u32;
        self
    }
    pub fn mip_level_count(&mut self, mip_level_count: usize) -> &mut Self {
        self.allocate_info.mip_level_count = mip_level_count as u32;
        self
    }
    pub fn format(&mut self, format: TiFormat) -> &mut Self {
        self.allocate_info.format = format;
        self
    }
    pub fn export_sharing(&mut self, export_sharing: bool) -> &mut Self {
        self.allocate_info.export_sharing = if export_sharing { TI_TRUE } else { TI_FALSE };
        self
    }
    pub fn usage(&mut self, usage: TiImageUsageFlags) -> &mut Self {
        self.allocate_info.usage = usage;
        self
    }

    pub fn build(&self) -> Result<Image> {
        Image::new(self.runtime, &self.allocate_info)
    }
}

struct Image_ {
    runtime: Runtime,
    image: TiImage,
    allocate_info: TiImageAllocateInfo,
}
impl Image_ {
    pub fn new(runtime: &Runtime, allocate_info: &TiImageAllocateInfo) -> Result<Self> {
        let image = unsafe {
            ti_allocate_image(runtime.runtime(), allocate_info)
        };
        get_last_error()?;
        Ok(Image_ {
            runtime: runtime.clone(),
            image,
            allocate_info: allocate_info.clone(),
        })
    }
}
impl Drop for Image_ {
    fn drop(&mut self) {
        unsafe {
            ti_free_image(self.runtime.runtime(), self.image);
        }
    }
}

#[derive(Clone)]
pub struct Image {
    inner: Rc<Image_>,
}
impl Image {
    pub fn new(runtime: &Runtime, allocate_info: &TiImageAllocateInfo) -> Result<Self> {
        Ok(Image {
            inner: Rc::new(Image_::new(runtime, allocate_info)?),
        })
    }

    pub fn runtime(&self) -> TiRuntime {
        self.inner.runtime.runtime()
    }
    pub fn image(&self) -> TiImage {
        self.inner.image
    }

    pub fn dimension(&self) -> TiImageDimension {
        self.inner.allocate_info.dimension
    }
    pub fn width(&self) -> u32 {
        self.inner.allocate_info.extent.width
    }
    pub fn height(&self) -> u32 {
        self.inner.allocate_info.extent.height
    }
    pub fn depth(&self) -> u32 {
        self.inner.allocate_info.extent.depth
    }
    pub fn array_layer_count(&self) -> u32 {
        self.inner.allocate_info.extent.array_layer_count
    }
    pub fn mip_level_count(&self) -> u32 {
        self.inner.allocate_info.mip_level_count
    }
    pub fn format(&self) -> TiFormat {
        self.inner.allocate_info.format
    }
    pub fn export_sharing(&self) -> bool {
        self.inner.allocate_info.export_sharing != TI_FALSE
    }
    pub fn usage(&self) -> TiImageUsageFlags {
        self.inner.allocate_info.usage
    }
}

