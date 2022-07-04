use std::ptr::null;
use sdl2::event::Event;
use sdl2::log::Category::Input;
use crate::input::input::BasicInput;
use crate::input::input_identifier::InputIdentifier;
use crate::input::input_listener::InputListener;
use crate::input::input_provider::InputProvider;
use crate::input::pump::Pump;

pub struct SDLInputProvider{
    pub(crate) listeners: Vec<&dyn InputListener>,
    pub(crate) event_pump: sdl2::EventPump
}
impl InputProvider for SDLInputProvider{
    fn add_input_listener(&mut self, input_listener: Weak<dyn InputListener>) {
        self.listeners.push(input_listener);
    }

    fn remove_input_listener(&mut self, input_listener: &dyn InputListener) {
        self.listeners.retain(|v| v.as_ptr() != input_listener);
    }
}

impl Pump for SDLInputProvider {
    fn pump(&mut self) {
        for event in self.event_pump.poll_iter() {

            let input= match event {
                Event::KeyDown{scancode,..} => {
                    BasicInput{
                        value: 1.0,
                        deadzone: 0.0,
                        threshold: 0.5,
                        min: 0.0,
                        max: 1.0,
                        identifier: InputIdentifier::Key { key: scancode.unwrap().name().chars().next().unwrap() }
                    }
                }
                Event::KeyUp{scancode,..} => {
                    BasicInput{
                        value: 0.0,
                        deadzone: 0.0,
                        threshold: 0.5,
                        min: 0.0,
                        max: 1.0,
                        identifier: InputIdentifier::Key { key: scancode.unwrap().name().chars().next().unwrap() }
                    }
                }
                _ => { return; }
            };

            for listener in self.listeners{
                match input.identifier {
                    InputIdentifier::Key{..} | InputIdentifier::Button {..} => {
                        if input.value > input.threshold {
                            listener.upgrade().unwrap().on_button_pressed(&input, false)
                        }else {
                            listener.upgrade().unwrap().on_button_released(&input)
                        }
                    }
                }

            }


        }
    }
}

impl SDLInputProvider{
    pub(crate) fn new(context : sdl2::Sdl) -> SDLInputProvider{
        return SDLInputProvider{ listeners: Vec::new(), event_pump: context.event_pump().unwrap() }
    }
}