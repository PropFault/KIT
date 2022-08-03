use std::io::{Cursor, Read};
use image::{ColorType};
use image::io::Reader as ImageReader;
use crate::libs::loading::loader::Loader;
use crate::libs::loading::resource::Resource;

pub struct ImageLoader{
}

pub enum PixelFormat{
    RGB,
    RGBA
}
pub struct ImageRaw{
    pub data : Box<[u8]>,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub format: PixelFormat
}

impl Loader<ImageRaw> for ImageLoader{
    fn load(&mut self, resource: &dyn Resource) -> ImageRaw {
        let mut raw = Vec::new();
        resource.read().read_to_end(&mut raw).map_err(|e| e.to_string()).unwrap();
        println!("{}", raw.len());
        let cursor = Cursor::new(raw);
        let data = ImageReader::new(cursor).with_guessed_format().unwrap().decode().unwrap();
        let color = match data.color(){
            ColorType::Rgb8 | ColorType::Rgb16 => {
                PixelFormat::RGB
            }
            ColorType::Rgba8 | ColorType::Rgba16 => {
                PixelFormat::RGBA
            }
            _ => {PixelFormat::RGB}
        };
        println!("returning raw image BYTES: {}",data.color().bytes_per_pixel());
        return ImageRaw{
            data: Box::from(data.as_bytes()),
            width: data.width(),
            height: data.height(),
            pitch: data.width() * u32::from(data.color().bytes_per_pixel()),
            format: color
        }
    }
}
impl ImageLoader{
    pub fn new() -> ImageLoader{
        ImageLoader{}
    }
}