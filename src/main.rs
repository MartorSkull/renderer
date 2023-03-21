use std::ops::Range;

use minifb::Key;
use hsl::HSL;

mod canvas;

use crate::canvas::Canvas;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut canvas = Canvas::new(
        "Render From scratch",
        WIDTH,
        HEIGHT,
    );

    let mut r = 50.0;
    let mut t = 0.0;

    while canvas.window.is_open() && !canvas.window.is_key_down(Key::Escape) {
        let (w, h) = canvas.get_size();
        let xs: Range<isize> = -(w as isize)/2..(w as isize)/2;
        let ys: Range<isize>  = -(h as isize)/2..(h as isize)/2;
        let cs = ys.flat_map(|y| xs.clone().map(move |x| (x, y)));
        for (x,y) in cs {
            if ((x.pow(2)+y.pow(2)) as f64).sqrt() <= r {
                let h = (((x*360) as f64)/(2.0*r)+t) % 360.0;
                canvas.put_pixel(x, y, HSL{h, s: 1.0, l: 0.5});
            } else {
                canvas.put_pixel(x, y, 0);
            }
        }

        if canvas.window.is_key_down(Key::Equal) {
            r += 0.2;
        }
        if canvas.window.is_key_down(Key::Minus) {
            r -= 0.2;
        }
        t = (t + 1.0) % 360.0;

        canvas.show();
    }
}