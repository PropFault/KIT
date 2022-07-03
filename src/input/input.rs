use crate::input::input_identifier::InputIdentifier;

pub trait Input{
    fn get_value(&self) -> f32;
    fn set_deadzone(&mut self, deadzone: f32);
    fn get_deadzone(&self) -> f32;
    fn set_threshold(&mut self, threshold: f32);
    fn get_threshold(&self) -> f32;

    fn set_min(&mut self, min: f32);
    fn get_min(&self) -> f32;

    fn set_max(&mut self, max: f32);
    fn get_max(&self) -> f32;

    fn set_identifier(&mut self, id: &InputIdentifier);
    fn get_identifier(&self) -> &InputIdentifier;

}