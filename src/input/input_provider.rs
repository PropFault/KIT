use crate::input::input_listener::InputListener;

pub trait InputProvider{
    fn add_input_listener(&self, input_listener : &dyn InputListener);
    fn remove_input_listener(&self, input_listener : &dyn InputListener);
}