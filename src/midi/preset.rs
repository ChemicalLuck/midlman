use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Preset {
    pub components: PresetComponents,
}

#[derive(Debug, Deserialize)]
pub struct PresetComponents {
    pub sliders: String,
    pub buttons: String,
    pub dials: String,
}
