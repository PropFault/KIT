use crate::input::input::Input;

pub trait InputListener{
    fn on_axis_changed(&self, axis:&dyn Input){} //called ONCE when input value changes
    fn on_button_pressed(&self, button:&dyn Input, is_initial_press : bool){} //called EVERY TICK when input goes above treshhold
    fn on_button_released(&self, button:&dyn Input){} //called ONCE when input goes below treshhold
}