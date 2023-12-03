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

pub trait ComponentCallback: Fn(u7) + Send {
    fn clone_box<'a>(&self) -> Box<dyn 'a + ComponentCallback>
    where
        Self: 'a;
}

impl<F> ComponentCallback for F
where
    F: Fn(u7) + Clone + Send,
{
    fn clone_box<'a>(&self) -> Box<dyn 'a + ComponentCallback>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn 'a + ComponentCallback> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}
