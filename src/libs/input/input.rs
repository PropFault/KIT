use crate::input::input_identifier::InputIdentifier;

pub trait Input : ToString{
    fn get_value(&self) -> f32;
    fn set_deadzone(&mut self, deadzone: f32);
    fn get_deadzone(&self) -> f32;
    fn set_threshold(&mut self, threshold: f32);
    fn get_threshold(&self) -> f32;

    fn set_min(&mut self, min: f32);
    fn get_min(&self) -> f32;

    fn set_max(&mut self, max: f32);
    fn get_max(&self) -> f32;

    fn set_identifier(&mut self, id: InputIdentifier);
    fn get_identifier(&self) -> &InputIdentifier;
}

pub struct BasicInput{
    pub(crate) value : f32,
    pub(crate) deadzone : f32,
    pub(crate) threshold : f32,
    pub(crate) min : f32,
    pub(crate) max : f32,
    pub(crate) identifier: InputIdentifier
}

impl Input for BasicInput {
    fn get_value(&self) -> f32 {
        return self.value;
    }

    fn set_deadzone(&mut self, deadzone: f32) {
        self.deadzone = deadzone;
    }

    fn get_deadzone(&self) -> f32 {
        return self.deadzone;
    }

    fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold;
    }

    fn get_threshold(&self) -> f32 {
        return self.threshold;
    }

    fn set_min(&mut self, min: f32) {
        self.min = min;
    }

    fn get_min(&self) -> f32 {
        return self.min;
    }

    fn set_max(&mut self, max: f32) {
        self.max = max;
    }

    fn get_max(&self) -> f32 {
        return self.max;
    }

    fn set_identifier(&mut self, id: InputIdentifier) {
        self.identifier = id;
    }

    fn get_identifier(&self) -> &InputIdentifier {
        return &self.identifier;
    }
}

impl BasicInput{
    pub(crate) fn new(identifier: InputIdentifier) -> BasicInput {
        return BasicInput{value: 0.0, deadzone: 0.0, min: 0.0, max: 1.0, threshold: 0.5, identifier};
    }

    pub fn set_value(&mut self,value : f32){
        self.value = value;
    }
}

impl ToString for BasicInput{
    fn to_string(&self) -> String {
        return String::new() + "VALUE: " + &*self.value.to_string() + " ID: " + &*self.identifier.to_string();
    }
}