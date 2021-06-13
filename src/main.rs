pub mod widgets;
pub mod functions;
pub mod data;

use druid::{WindowDesc, AppLauncher};
use druid::widget::{Flex, Label,Slider, LensWrap};

use crate::widgets::color_picker::ColorPicker;
use crate::data::HSL;

fn ui_builder() -> impl druid::Widget<HSL> {
    let color_picker = ColorPicker {cursor_x: 0.2, cursor_y: 0.9, size: druid::Size::default()};
    let slider = LensWrap::new(Slider::new(), HSL::hue);
    let hue_label = Label::dynamic(|data: &HSL, _env| {format!("Hue: {0:.2}", data.hue)});
    let sat_label = Label::dynamic(|data: &HSL, _env| {format!("Saturation: {0:.2}", data.saturation)});
    let light_label = Label::dynamic(|data: &HSL, _env| {format!("Lightness: {0:.2}", data.lightness)});
    
    let col = Flex::column()
        .with_flex_child(color_picker, 1.0)
        .with_spacer(20.)
        .with_child(slider)
        .with_child(hue_label)
        .with_child(sat_label)
        .with_child(light_label)
        .with_spacer(20.);

    col
}

fn main() {
    let window = WindowDesc::new(ui_builder()).window_size(druid::Size::from((400., 500.))).title("Color Picker");
    AppLauncher::with_window(window)
        .log_to_console()
        .launch(HSL { hue: 0.4, saturation: 0.4, lightness: 0.4 })
        .expect("Launch Failed");
}