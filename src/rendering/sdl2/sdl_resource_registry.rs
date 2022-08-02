use std::collections::HashMap;
use std::env::args;
use std::ops::DerefMut;
use rand::RngCore;
use rand::rngs::ThreadRng;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use crate::libs::loading::image_loader::{ImageRaw, PixelFormat};
use crate::libs::rendering::resource_registry::ResourceRegistry;

pub struct SDLResourceRegistry<'a, T>{
    pub texture_creator : TextureCreator<T>,
    textures: HashMap<u64, Texture<'a>>,
    rng: ThreadRng
}

impl <'a, T>ResourceRegistry for SDLResourceRegistry<'a, T> {
    fn register_texture(&mut self, image: &mut ImageRaw) -> u64 {
        let format = match image.format {
            PixelFormat::RGB => PixelFormatEnum::BGR888,
            _ => PixelFormatEnum::ABGR8888
        };
        let surface = Surface::from_data(image.data.deref_mut(), image.width, image.height, image.pitch, format).unwrap();
        let texture_creator = args.1;
        let k = self.rng.next_u64();
        self.textures.insert(k, surface.as_texture(texture_creator).unwrap());
        k
    }
}