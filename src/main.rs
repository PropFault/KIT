extern crate core;

mod rendering;
mod ecs;
mod input;
mod math;
mod quad;

use std::rc::Weak;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::printf;
use sdl2::log::Category::Render;
use sdl2::pixels::Color;
use sdl2::Sdl;
use crate::input::input::Input;
use crate::input::input_listener::InputListener;
use crate::input::input_provider::InputProvider;
use crate::input::pump::Pump;
use crate::input::r#impl::sdl::sdl_input_provider::SDLInputProvider;

struct Game{

}
impl InputListener for Game{
    fn on_button_pressed(&self, button:&dyn Input, is_initial_press : bool){
        println!("VALUE: {}", button.get_value());
    }
    fn on_button_released(&self, button:&dyn Input){
        println!("VALUE: {}", button.get_value());
    }

}
impl Game{
    pub fn main(&self){
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut i = 0;
        let mut input_provider = SDLInputProvider::new(sdl_context);
        input_provider.add_input_listener(self);
        while i < 100000{
            input_provider.pump();
            i+=1;
        }
    }
}

fn main() {
    let game = Game {};
    game.main();
}

