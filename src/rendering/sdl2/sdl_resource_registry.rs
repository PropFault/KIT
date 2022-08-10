use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::DerefMut;
use rand::{RngCore, thread_rng};
use rand::rngs::ThreadRng;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use crate::libs::loading::image_loader::{ImageRaw, PixelFormat};
use crate::libs::rendering::resource_registry::ResourceRegistry;

pub struct SDLResourceRegistry<'a, T>{
    pub texture_creator : &'a TextureCreator<T>,
    textures: HashMap<u64, Texture<'a>>,
    rng: ThreadRng
}

impl <'a, T>ResourceRegistry for SDLResourceRegistry<'a, T> {
    fn register_texture(& mut self, image: &mut ImageRaw) -> u64 {
        let format = match image.format {
            PixelFormat::RGB => PixelFormatEnum::BGR888,
            _ => PixelFormatEnum::ABGR8888
        };
        let surface = Surface::from_data(image.data.deref_mut(), image.width, image.height, image.pitch, format).unwrap();
        let k = self.rng.next_u64();
        self.textures.insert(k, Texture::from_surface(surface.borrow(), self.texture_creator).unwrap());
        k
    }
}

impl<'a, T> SDLResourceRegistry<'a, T> {
    pub(crate) fn new(texture_creator :&'a TextureCreator<T>) -> SDLResourceRegistry<'a, T>{
        SDLResourceRegistry{
            texture_creator,
            textures: HashMap::new(),
            rng: thread_rng()
        }
    }
    pub(crate) fn checkout_texture(&mut self, ticket: u64) -> &mut Texture<'a>{
        return self.textures.get_mut(&ticket).expect( "");
    }
}