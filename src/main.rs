extern crate core;

mod rendering;
mod ecs;
mod input;
mod math;
mod quad;
mod pump;

use crate::ecs::entity_component_hash_map::EntityComponentHashMap;
use rand::Rng;
use crate::ecs::component_pool::Component;
use crate::ecs::entity_component_map::EntityComponentMap;
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
        let mut pumper = SDLInputProvider::new(self, &sdl_context, Game::on_button_pressed, Game::on_button_released);
        let mut ecs = EntityComponentHashMap::new();
        let mut rng = rand::thread_rng();
        let entity1 = rng.gen::<u64>();
        let entity2 = rng.gen::<u64>();
        let component_type_1 = 1;
        let component_type_2 = 2;
        let mut components : Vec<u64> = Vec::new();
        for i in 0 .. 100{
            components.push(i);
            if i % 2 == 0 {
                ecs.add_component(entity1, component_type_1, i);
            }else {
                ecs.add_component(entity2, component_type_2, i);
            }
        }
        println!("Entity 1 type 1:");
        if let Some(comps) =  ecs.get_components(entity1, component_type_1){
            for component in comps{
                println!("{}", component);
            }
        }


        while i < 10000000{
            pumper.pump();
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

