
// pub fn make_image_data(width: usize, height: usize) -> Vec<u8> {
//     let mut result = vec![0; width * height * 4];
//     for y in 0..height {
//         for x in 0..width {
//             let ix = (y * width + x) * 4;
//             result[ix] = x as u8;
//             result[ix + 1] = y as u8;
//             result[ix + 2] = !(x as u8);
//             result[ix + 3] = 127;
//         }
//     }
//     result
// }


pub fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
    let mut t = t;
    if t < 0. {
        t += 1.
    }
    if t > 1. {
        t -= 1.
    };
    if t < 1. / 6. {
        return p + (q - p) * 6. * t;
    }
    if t < 1. / 2. {
        return q;
    }
    if t < 2. / 3. {
        return p + (q - p) * (2. / 3. - t) * 6.;
    }
    return p;
}

pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let r;
    let g;
    let b;

    if s == 0.0 {
        r = l;
        g = l;
        b = l; // achromatic
    } else {
        let q = if l < 0.5 { l * (1. + s) } else { l + s - l * s };

        let p = 2. * l - q;
        r = hue_to_rgb(p, q, h + 1. / 3.);
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - 1. / 3.);
    }

    return (
        (r * 255.).round() as u8,
        (g * 255.).round() as u8,
        (b * 255.).round() as u8,
    );
}


pub fn make_sl_image(width: usize, height: usize, hue: f64) -> Vec<u8> {
    let mut image_data = vec![0; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let ix = (y * width + x) * 4;
            let x_ratio = x as f64 / width as f64;
            let y_ratio = y as f64 / width as f64;

            // Where the magic happens
            let color = hsl_to_rgb(hue, x_ratio, y_ratio);

            image_data[ix + 0] = color.0;
            image_data[ix + 1] = color.1;
            image_data[ix + 2] = color.2;
            image_data[ix + 3] = 255
        }
    }

    image_data
}