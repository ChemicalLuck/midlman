use midly::num::u7;

pub trait Component {
    fn get_controller(&self) -> u7;
}

pub trait ComponentMut: Component {
    fn set_value(&mut self, value: u7);
}

#[derive(Clone)]
pub enum ComponentType {
    Slider,
    Button,
    Dial,
}

#[derive(Clone)]
pub struct Slider {
    controller: u7,
    value: u7,
}

impl Slider {
    pub fn new(controller: u7) -> Slider {
        Slider {
            controller,
            value: u7::from(0),
        }
    }
}

impl Component for Slider {
    fn get_controller(&self) -> u7 {
        self.controller
    }
}
impl ComponentMut for Slider {
    fn set_value(&mut self, value: u7) {
        self.value = value;
    }
}

#[derive(Clone)]
pub struct Button {
    controller: u7,
    value: u7,
}

impl Button {
    pub fn new(controller: u7) -> Button {
        Button {
            controller,
            value: u7::from(0),
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
        self.value = value;
    }
}

#[derive(Clone)]
pub struct Dial {
    controller: u7,
    value: u7,
}

impl Dial {
    pub fn new(controller: u7) -> Dial {
        Dial {
            controller,
            value: u7::from(0),
        }
    }
}

impl Component for Dial {
    fn get_controller(&self) -> u7 {
        self.controller
    }
}

impl ComponentMut for Dial {
    fn set_value(&mut self, value: u7) {
        self.value = value;
    }
}
