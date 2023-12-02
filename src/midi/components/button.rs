use midly::num::u7;

use crate::midi::components::{Component, ComponentMut};
use crate::volume::Session;

#[derive(Clone)]
pub struct Button {
    controller: u7,
    audio_interface: Option<Box<dyn Session>>,
}

impl Button {
    pub fn new(controller: u7) -> Button {
        Button {
            controller,
            audio_interface: None,
        }
    }
    pub fn set_audio_interface(&mut self, audio_interface: &Box<dyn Session>) {
        self.audio_interface = Some(audio_interface.to_owned());
    }
}

impl Component for Button {
    fn get_controller(&self) -> u7 {
        self.controller
    }
}

impl ComponentMut for Button {
    fn set_value(&mut self, value: u7) {
        if let Some(audio_interface) = &mut self.audio_interface {
            unsafe {
                audio_interface.set_volume(value.as_int() as f32 / 127.0);
            }
        }
    }
}
