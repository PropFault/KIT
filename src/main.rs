extern crate core;

mod rendering;
mod ecs;
mod input;
mod math;
mod quad;
mod pump;

use crate::input::input::Input;
use crate::input::input_provider::InputProvider;
use crate::input::r#impl::sdl::sdl_input_provider::SDLInputProvider;
use crate::pump::pump::Pump;
use crate::pump::pumper::Pumper;

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

        let mut i = 0;
        let input_provider: &mut dyn InputProvider<Game> = &mut SDLInputProvider::new(self, sdl_context, Game::on_button_pressed, Game::on_button_released);

        while i < 10000000{
            input_provider.pump();
            i+=1;
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

