use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct HSL {
    pub hue: f64,
    pub saturation: f64,
    pub lightness: f64
}