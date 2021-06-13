# Color Picker Rust Druid Native Application

<img src="https://github.com/santokalayil/Color_Picker_Rust_Druid_Native_Application/blob/main/colorpicker.png">
<br>
<hr>
The code: <br />

```
pub struct ColorPicker {
    pub cursor_x: f64,
    pub cursor_y: f64,
    pub size: Size,
}

use crate::data::HSL;


impl Widget<HSL>  for ColorPicker {

    fn event(&mut self, ctx : &mut EventCtx, event: &Event, data : &mut HSL, _env: &Env) { // here data is mutable
        match event {
            Event::MouseDown(mouse) => {
                ctx.set_active(true);
                self.cursor_x = mouse.pos.x / self.size.width;
                self.cursor_y = mouse.pos.y / self.size.height;
                
                data.saturation = (mouse.pos.x / self.size.width).max(0.0).min(1.0);
                data.lightness = (mouse.pos.y / self.size.height).max(0.0).min(1.0);
                ctx.request_paint(); ctx.set_handled()
            },
            Event::MouseMove(mouse) => {
                if ctx.is_active() {
                    self.cursor_x = mouse.pos.x / self.size.width;
                    self.cursor_y = mouse.pos.y / self.size.height;

                    data.saturation = (mouse.pos.x / self.size.width).max(0.0).min(1.0);
                    data.lightness = (mouse.pos.y / self.size.height).max(0.0).min(1.0);
                    ctx.request_paint(); ctx.set_handled()
                }
            },
            Event::MouseUp(_) => {
                ctx.set_active(false);
            },
            _ => (),
        }
    }
    fn lifecycle(&mut self, _ctx : &mut LifeCycleCtx, _event: &LifeCycle, _data : &HSL, _env: &Env) { // here data is not mutable

    }
    fn update(&mut self, _ctx : &mut UpdateCtx, _old_data: &HSL, _data : &HSL, _env: &Env) {
        // ctx.request_update()
    }

    fn layout(&mut self, _ctx : &mut LayoutCtx, bc: &BoxConstraints, _data : &HSL, _env: &Env) -> Size { // here bc is not unused
        // if bc.is_width_bounded() | bc.is_height_bounded() {
        //     let size = Size::new(100.0, 100.0);
        //     bc.constrain(size)
        // } else {
        //     bc.max()
        // }
        self.size = bc.max();
        self.size
    }

    fn paint(&mut self, ctx : &mut PaintCtx, data : &HSL, _env: &Env) { // here ctx and data are not unused 
        use crate::functions::color_picker::*;
        let size = ctx.size(); let rect = size.to_rect();
        let rgb = hsl_to_rgb(0.5, 0.5, 0.5);
        ctx.fill(rect, &Color::rgb8(rgb.0, rgb.1, rgb.2));

        // We're generating a 256 by 256 pixels image, with a constant hue of 0.5
        let image_data = make_sl_image(256, 256, data.hue);


        let image = ctx
            .make_image(256, 256, &image_data, druid::piet::ImageFormat::RgbaSeparate)
            .unwrap();

        // When piet draws our image it will stretch it automatically.
        // We'll fix this later by giving our widget a fixed size.
        ctx.draw_image(
            &image,
            rect, // ctx.size().to_rect();
            druid::piet::InterpolationMode::Bilinear,
        );
        use druid::kurbo::{Rect,};

        // // static positioning
        // let cursor_rect = Rect::from_origin_size((100., 100.), (10., 10.));
        // ctx.stroke(cursor_rect, &Color::BLACK, 1.0);

        // Create a UnitPoint from our cursor floats
        let cursor_point = druid::piet::UnitPoint::new(self.cursor_x, self.cursor_y);
        let resolved_point = cursor_point.resolve(rect);
        let cursor_rect = Rect::from_origin_size(resolved_point, (20., 20.));
        ctx.stroke(cursor_rect, &Color::BLACK, 2.0);

        let inset_point = resolved_point + druid::kurbo::Vec2::new(2., 2.);
        let white_cursor_rect = Rect::from_origin_size(inset_point, (16., 16.));
        ctx.stroke(white_cursor_rect, &Color::rgba8(255, 255, 255, 128), 2.0);


        
        
    }

}
```


