use midly::num::u7;

use crate::midi::components::{Component, ComponentMut};

use super::ComponentCallback;

#[derive(Clone)]
pub struct Button {
    controller: u7,
    callback: Option<Box<dyn ComponentCallback>>,
}

impl Button {
    pub fn new(controller: u7) -> Button {
        Button {
            controller,
            callback: None,
        }
    }
    pub fn set_callback(&mut self, callback: Box<dyn ComponentCallback>) {
        self.callback = Some(callback);
    }
    pub fn invoke_callback(&self, value: u7) {
        if let Some(callback) = &self.callback {
            callback(value);
        }
    }
}

impl Component for Button {
    fn get_controller(&self) -> u7 {
        self.controller
    }
}

impl ComponentMut for Button {
    fn set_value(&mut self, value: u7) {
        self.invoke_callback(value);
    }
}
