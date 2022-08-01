use std::any::Any;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::iter::Map;
use crate::input::input::Input;
use crate::input::input_identifier::InputIdentifier;
use crate::math::floating_point::{lerp, map_range, normalise_range};

pub trait ActionTrait : Input {
    fn add_input(&mut self, input : Box<dyn Input>, map_floor: f32, map_ceil: f32, invert: bool);
    fn remove_input(&mut self, input: Box<dyn Input>);
}

struct ActionMapValue{
    map_floor: f32,
    map_ceil: f32,
    invert: bool,
    input: Box<dyn Input>
}

pub struct Action{
    input_map : HashMap<InputIdentifier, ActionMapValue>,
    deadzone : f32,
    threshold : f32,
    identifier : InputIdentifier
}


impl Input for Action {
    fn get_value(&self) -> f32 {
        let mut value = 0.0;
        for entry in self.input_map.iter(){
            let input = &entry.1.input;
            let mut normalised = normalise_range(input.get_value(), input.get_max(), input.get_min());
            if entry.1.invert {
                normalised = 1.0 - normalised;
            }
            let remapped = lerp(entry.1.map_floor, entry.1.map_ceil, normalised);
            value += remapped;
        };
        return value;
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
        todo!()
    }

    fn get_min(&self) -> f32 {
        return self.input_map.values().min_by(|a , b| a.input.get_min().total_cmp(&b.input.get_min())).unwrap().input.get_min();
    }

    fn set_max(&mut self, max: f32) {
        todo!()
    }

    fn get_max(&self) -> f32 {
        return self.input_map.values().max_by(|a, b| a.input.get_max().total_cmp(&b.input.get_max())).unwrap().input.get_max();
    }

    fn set_identifier(&mut self, id: InputIdentifier) {
        match id {
            InputIdentifier::Action {..} => {
                self.identifier = id;
            }
            _=> panic!("Can't set identifier to non-action identifier")
        }
    }

    fn get_identifier(&self) -> &InputIdentifier {
        return &self.identifier;
    }
}

impl ActionTrait for Action {
    fn add_input(&mut self, input: Box<dyn Input>, map_floor: f32, map_ceil: f32, invert: bool) {
        let action_map_value = ActionMapValue{map_floor, map_ceil, input, invert };
        self.input_map.insert((*action_map_value.input.get_identifier()).clone(), action_map_value);
    }

    fn remove_input(&mut self, input: Box<dyn Input>) {
        self.input_map.remove(input.get_identifier());
    }
}

impl Default for Action {
    fn default() -> Self {
        Self{
            input_map: HashMap::new(),
            deadzone : 0.05,
            threshold : 0.5,
            identifier : InputIdentifier::Action { id: "untitled".to_string() }
        }
    }
}

impl ToString for Action{
    fn to_string(&self) -> String {
        return self.identifier.to_string()
    }
}