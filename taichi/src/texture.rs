use taichi_sys::{TiTexture, TiImageDimension, TiImage, TiImageExtent, TiSampler, TiFormat, TiImageUsageFlags};

use crate::{
    Result, Runtime, image::ImageBuilder, Image
};

pub struct TextureBuilder<'a> {
    image_builder: ImageBuilder<'a>,
    texture: TiTexture,
}
impl<'a> TextureBuilder<'a> {
    pub fn new(runtime: &'a Runtime) -> Self {
        let image_builder = ImageBuilder::new(runtime);
        let texture = TiTexture {
            image: TiImage::null(),
            sampler: TiSampler::null(),
            dimension: TiImageDimension::D2D,
            extent: TiImageExtent {
                width: 1,
                height: 1,
                depth: 1,
                array_layer_count: 1,
            },
            format: TiFormat::Rgba8,
        };
        TextureBuilder {
            image_builder,
            texture,
        }
    }

    pub fn dimension(&mut self, dimension: TiImageDimension) -> &mut Self {
        self.image_builder.dimension(dimension);
        self.texture.dimension = dimension;
        self
    }
    pub fn width(&mut self, width: usize) -> &mut Self {
        self.image_builder.width(width);
        self.texture.extent.width = width as u32;
        self
    }
    pub fn height(&mut self, height: usize) -> &mut Self {
        self.image_builder.height(height);
        self.texture.extent.height = height as u32;
        self
    }
    pub fn depth(&mut self, depth: usize) -> &mut Self {
        self.image_builder.depth(depth);
        self.texture.extent.depth = depth as u32;
        self
    }
    pub fn array_layer_count(&mut self, array_layer_count: usize) -> &mut Self {
        self.image_builder.array_layer_count(array_layer_count);
        self.texture.extent.array_layer_count = array_layer_count as u32;
        self
    }
    pub fn mip_level_count(&mut self, mip_level_count: usize) -> &mut Self {
        self.image_builder.mip_level_count(mip_level_count);
        self
    }
    pub fn format(&mut self, format: TiFormat) -> &mut Self {
        self.image_builder.format(format);
        self.texture.format = format;
        self
    }
    pub fn export_sharing(&mut self, export_sharing: bool) -> &mut Self {
        self.image_builder.export_sharing(export_sharing);
        self
    }
    pub fn usage(&mut self, usage: TiImageUsageFlags) -> &mut Self {
        self.image_builder.usage(usage);
        self
    }

    pub fn build(&mut self) -> Result<Texture> {
        let image = self.image_builder.build()?;
        self.texture.image = image.image();
        Texture::new(image, self.texture)
    }
}

pub struct Texture {
    image: Image,
    texture: TiTexture,
}
impl Texture {
    fn new(image: Image, texture: TiTexture) -> Result<Self> {
        debug_assert_eq!(image.image(), texture.image);
        Ok(Texture {
            image,
            texture,
        })
    }

    pub fn image(&self) -> &Image {
        &self.image
    }
    pub fn dimension(&self) -> TiImageDimension {
        self.texture.dimension
    }
    pub fn width(&self) -> usize {
        self.texture.extent.width as usize
    }
    pub fn height(&self) -> usize {
        self.texture.extent.height as usize
    }
    pub fn depth(&self) -> usize {
        self.texture.extent.depth as usize
    }
    pub fn array_layer_count(&self) -> usize {
        self.texture.extent.array_layer_count as usize
    }
    pub fn mip_level_count(&self) -> u32 {
        self.image.mip_level_count()
    }
    pub fn format(&self) -> TiFormat {
        self.texture.format
    }
    pub fn export_sharing(&self) -> bool {
        self.image.export_sharing()
    }
    pub fn usage(&self) -> TiImageUsageFlags {
        self.image.usage()
    }
    pub fn texture(&self) -> &TiTexture {
        &self.texture
    }
}
