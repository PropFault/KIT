use crate::input::input::Input;

pub trait InputListener{
    fn on_axis_changed(axis:&dyn Input){} //called ONCE when input value changes
    fn on_button_pressed(button:&dyn Input, is_initial_press : bool){} //called EVERY TICK when input goes above treshhold
    fn on_button_released(button:&dyn Input){} //called ONCE when input goes below treshhold
}