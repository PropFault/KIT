use sdl2::event::Event;
use crate::Input;
use crate::libs::input::input::BasicInput;
use crate::libs::input::input_identifier::InputIdentifier;
use crate::libs::input::input_provider::InputProvider;
use crate::libs::pump::pump::Pump;

pub struct SDLInputProvider<'a, T> {
    pub event_pump: sdl2::EventPump,
    _self : &'a mut T,
    pub on_button_pressed: fn(_self : &mut T, input: &dyn Input),
    pub on_button_released: fn(_self: &mut T, input: &dyn Input)
}

impl<'a, T> InputProvider<T> for SDLInputProvider<'a, T> {
    fn set_on_button_pressed_callback(&mut self, callback: fn(&mut T, &dyn Input)) {
        self.on_button_released = callback;
    }

    fn set_on_button_released_callback(&mut self, callback: fn(&mut T, &dyn Input)) {
        self.on_button_released = callback;
    }
}

impl<'a, T> Pump for SDLInputProvider<'a, T> {
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

            match input.identifier {
                InputIdentifier::Key{..} | InputIdentifier::Button {..} => {
                    if input.value > input.threshold {
                        (self.on_button_pressed)(self._self, &input)
                    }else {
                        (self.on_button_released)(self._self, &input)
                    }
                }
                _ => {}
            }
        }
    }
}

impl<'a, T> SDLInputProvider<'a, T>{
    pub(crate) fn new(_self : &'a mut T, context : &sdl2::Sdl, on_button_pressed: fn(_self: & mut T, input: &dyn Input), on_button_released: fn(_self: & mut T, input: &dyn Input)) -> SDLInputProvider<'a, T> {
        return SDLInputProvider{ event_pump: context.event_pump().unwrap(), _self, on_button_pressed, on_button_released }
    }
}