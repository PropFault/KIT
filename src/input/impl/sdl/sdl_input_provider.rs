use crate::input::input_listener::InputListener;
use crate::input::input_provider::InputProvider;
use crate::input::pump::Pump;

struct SDLInputProvider{
    listeners: Vec<dyn InputListener>,
    event_pump: sdl2::EventPump
}
impl InputProvider for SDLInputProvider{
    fn add_input_listener(&self, input_listener: &dyn InputListener) {
        self.listeners.push_back(input_listener);
    }

    fn remove_input_listener(&self, input_listener: &dyn InputListener) {
        self.listeners.erase(input_listener);
    }
}

impl Pump for SDLInputProvider {
    fn pump(&mut self) {
        for event in self.event_pump.poll_iter() {
            for listener in self.listeners{
            }
        }
    }
}

impl SDLInputProvider{
    fn new(context : sdl2::Sdl) -> SDLInputProvider{
        return SDLInputProvider{ listeners: Vec::new(), event_pump: context.event_pump().unwrap() }
    }
}