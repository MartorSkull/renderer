mod color;
use minifb::{Window, WindowOptions};

use crate::canvas::color::Color;

pub struct Canvas{
    pub window: Window,
    buffer: Vec<u32>,
    updated: bool,
}

impl Canvas {
    pub fn new(name: &str, w: usize, h: usize) -> Self {
        let buffer: Vec<u32> = vec![0; w * h];

        let mut window = Window::new(
            name,
            w,
            h,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Canvas{ window, buffer, updated: true }
    }

    pub fn put_pixel<C: Color>(&mut self, x: isize, y: isize, color: C) {
        if self.updated {
            self.updated = false;
        }
        let w = self.get_width();
        let h = self.get_height();
        assert!(y.abs() <= h/2 && x.abs() <= w/2);
        let x = x + w/2;
        let y = y + h/2;
        let i = (y*w + x) as usize;
        self.buffer[i] = color.to_color();
    }

    pub fn show(&mut self) {
        if !self.updated {
            let (w, h) = self.window.get_size();
            self.window
                .update_with_buffer(&self.buffer, w, h)
                .unwrap();
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.window.get_size()
    }

    fn get_width(&self) -> isize {
        self.window.get_size().0 as isize
    }

    fn get_height(&self) -> isize {
        self.window.get_size().1 as isize
    }
}