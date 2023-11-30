use std::collections::HashMap;

use midly::num::u7;

use crate::midi::components::{Button, Component, ComponentMut, ComponentType, Dial, Slider};

#[derive(Clone)]
pub struct Controller {
    bank: u7,
    controllers: HashMap<u7, ComponentType>,
    sliders: Vec<Vec<Slider>>,
    buttons: Vec<Vec<Button>>,
    dials: Vec<Vec<Dial>>,
}

impl Controller {
    pub fn new() -> Self {
        let mut controllers = HashMap::new();

        // create sliders with controller numbers 3-11, inclusive
        let slider_controllers = vec![].into_iter().chain(3..=11).collect::<Vec<_>>();
        slider_controllers.iter().for_each(|i| {
            controllers.insert(u7::from(*i), ComponentType::Slider);
        });
        let sliders = slider_controllers
            .into_iter()
            .map(|i| Slider::new(u7::from(i)))
            .collect::<Vec<_>>();

        // 1, 2, 23-31, 44-49, 64, 67
        let button_controllers = vec![1, 2]
            .into_iter()
            .chain(23..=31)
            .chain(44..=49)
            .chain(vec![64, 67].into_iter())
            .collect::<Vec<_>>();
        button_controllers.iter().for_each(|i| {
            controllers.insert(u7::from(*i), ComponentType::Button);
        });
        let buttons = button_controllers
            .into_iter()
            .map(|i| Button::new(u7::from(i)))
            .collect::<Vec<_>>();

        // 12-21 inclusive
        let dial_controllers = vec![].into_iter().chain(12..=22).collect::<Vec<_>>();
        dial_controllers.iter().for_each(|i| {
            controllers.insert(u7::from(*i), ComponentType::Dial);
        });
        let dials = dial_controllers
            .into_iter()
            .map(|i| Dial::new(u7::from(i)))
            .collect::<Vec<_>>();

        Controller {
            bank: u7::from(0),
            controllers,
            sliders: (0..=8).map(|_| sliders.clone()).collect(),
            buttons: (0..=8).map(|_| buttons.clone()).collect(),
            dials: (0..=8).map(|_| dials.clone()).collect(),
        }
    }
    pub fn set_bank(&mut self, bank: u7) {
        self.bank = bank;
    }
    fn set_slider(&mut self, controller: u7, value: u7) {
        self.sliders[self.bank.as_int() as usize]
            .iter_mut()
            .find(|s| s.get_controller() == controller)
            .map(|s| s.set_value(value));
    }
    fn set_button(&mut self, controller: u7, value: u7) {
        self.buttons[self.bank.as_int() as usize]
            .iter_mut()
            .find(|b| b.get_controller() == controller)
            .map(|b| b.set_value(value));
    }
    fn set_dial(&mut self, controller: u7, value: u7) {
        self.dials[self.bank.as_int() as usize]
            .iter_mut()
            .find(|d| d.get_controller() == controller)
            .map(|d| d.set_value(value));
    }
    pub fn set_component(&mut self, controller: u7, value: u7) {
        if let Some(c) = self.controllers.get(&controller) {
            match c {
                ComponentType::Slider => self.set_slider(controller, value),
                ComponentType::Button => self.set_button(controller, value),
                ComponentType::Dial => self.set_dial(controller, value),
            }
        }
    }
}
