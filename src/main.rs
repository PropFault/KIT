extern crate core;

use sdl2::rect::Rect;
use sdl2::video::WindowContext;
use crate::libs::ecs::component_pool_lifeguard::ComponentPoolLifeguard;
use crate::libs::input::r#impl::sdl::sdl_input_provider::SDLInputProvider;

mod rendering;
mod quad;
mod components;
mod libs;

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
        let mut texture_creator = canvas.texture_creator();
        let mut texture_pool: ComponentPoolLifeguard<TextureComponent<WindowContext>> = ComponentPoolLifeguard::new();
        let texture_id = texture_pool.reserve(TextureComponent::initialize, ("C:/Users/Biggest/Desktop/colortest.png".to_string(), &mut texture_creator));
        while i < 10000000{
            pumper.pump();
            canvas.clear();
            let texture_comp = texture_pool.checkout(texture_id).unwrap();
            if let Some(texture) = texture_comp.texture.borrow_mut(){
                canvas.copy(texture, None, Some(Rect::new(100, 100, 256, 256))).unwrap();
                canvas.copy_ex(
                    texture,
                    None,
                    Some(Rect::new(450, 100, 256, 256)),
                    30.0,
                    None,
                    false,
                    false,
                ).expect("TODO: panic message");
                canvas.present();
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

