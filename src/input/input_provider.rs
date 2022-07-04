use crate::{Input, Pump};

pub trait InputProvider<T> : Pump{
    fn set_on_button_pressed_callback(&mut self, callback: fn(_self: &mut T, input: &dyn Input));
    fn set_on_button_released_callback(&mut self, callback: fn(_self: &mut T, input: &dyn Input));
}