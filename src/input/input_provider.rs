use std::rc::Weak;
use crate::input::input_listener::InputListener;

pub trait InputProvider{
    fn add_input_listener(&mut self, input_listener : Weak<dyn InputListener>);
    fn remove_input_listener(&mut self, input_listener : &dyn InputListener);
}