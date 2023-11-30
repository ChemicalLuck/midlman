mod button;
mod dial;
mod slider;

use midly::num::u7;

pub use button::Button;
pub use dial::Dial;
pub use slider::Slider;

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
