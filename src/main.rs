extern crate core;

use std::cell::RefCell;
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, RwLock};
use sdl2::video::WindowContext;
use crate::components::quad_component::QuadComponent;
use crate::components::texture_component::TextureComponent;
use crate::libs::ecs::component_pool::{ComponentPool, ReadableComponentPool};
use crate::libs::ecs::component_pool_lifeguard::ComponentPoolLifeguard;
use crate::libs::input::input::Input;
use crate::libs::input::r#impl::sdl::sdl_input_provider::SDLInputProvider;
use crate::libs::loading::resource::FileResource;
use crate::libs::pump::pump::Pump;
use crate::libs::rendering::renderer::Renderer;
use crate::rendering::sdl2::sdl_renderer::SDLRenderer;
use crate::rendering::sdl2::sdl_resource_registry::SDLResourceRegistry;
use crate::systems::QuadRenderSystem::QuadRenderSystem;

mod rendering;
mod quad;
mod components;
mod libs;
mod systems;

struct Game{
    pressed : i32,
    released : i32
}

impl Game{
    pub fn main(&mut self){
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).unwrap();

        let mut i = 0;
        let mut pumper = SDLInputProvider::new(self, &sdl_context, Game::on_button_pressed, Game::on_button_released);
        let mut texture_pool: ComponentPoolLifeguard<TextureComponent> = ComponentPoolLifeguard::new();
        let mut texture_pool_lock = Arc::new(RwLock::new(texture_pool));

        let mut quad_pool : ComponentPoolLifeguard<QuadComponent> = ComponentPoolLifeguard::new();
        let mut quad_pool_lock = Arc::new(RwLock::new(quad_pool));

        let texture_creator = canvas.texture_creator();
        let mut resource_reg  = SDLResourceRegistry::new(&texture_creator);
        let mut renderer = RwLock::new(SDLRenderer::new(canvas, resource_reg));
        let texture_id = texture_pool_lock.write().unwrap().reserve(TextureComponent::initialize, (&mut FileResource::new(Box::from(Path::new("/home/biggest/Downloads/unnamed.png"))), &mut renderer.write().unwrap().registry));

        let mut quad_render_system = QuadRenderSystem::new(quad_pool_lock.clone(), texture_pool_lock.clone(), &mut renderer);

        while i < 10000000{
            pumper.pump();
            renderer.write().unwrap().clear();
            let texture_comp = texture_pool.checkout(texture_id);
            if let Some(texture_id) = texture_comp.as_ref().unwrap().texture_ticket{
                renderer.write().unwrap().draw_tex(texture_id, 255, 255, 100, 100);
                renderer.write().unwrap().present();
                i+=1;
            }
        }
    }
    pub fn on_button_pressed(&mut self, input: &dyn Input){
        self.pressed += 1;
        println!("{} | TOTAL PRESSES {}", input.to_string(), self.pressed)
    }
    pub fn on_button_released(&mut self, input: &dyn Input){
        self.released += 1;
        println!("{} | TOTAL RELEASES {}", input.to_string(), self.released)
    }
}

fn main() {
    let mut game = Game {pressed: 0, released: 0};
    game.main();
}

