use serde::{Deserialize, Serialize};

use crate::graphics::rendering::shaders::gl_representations::GlColor;

/// A struct that represents colors for graphics.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Color {
    /// Red value of the color
    r: u8,
    /// Green value of the color
    g: u8,
    /// Blue value of the color
    b: u8,
    /// Alpha value of the color
    a: f32,
}

impl Color {
    /// Creates a new color using RGBA values
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        assert!((0. ..=1.).contains(&a), "Alpha value must be between 0.0 and 1.0");
        Self { r, g, b, a }
    }

    /// Create a new color using RGB. Will create an RGBA with Alpha = 1.0
    pub fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Color::new(r, g, b, 1.0)
    }

    /// Replaces the current color with the given one
    pub fn replace(&mut self, color: Color) {
        self.r = color.r;
        self.g = color.g;
        self.b = color.b;
        self.a = color.a;
    }

    pub fn new_hex(hex_code:&str) -> Self {
        let hex:Vec<char> = hex_code.chars().collect();
        let red:u8 = Color::get_hex_value(hex[1]) * 16 + Color::get_hex_value(hex[2]);
        let green:u8 = Color::get_hex_value(hex[3]) * 16 + Color::get_hex_value(hex[4]);
        let blue:u8 = Color::get_hex_value(hex[5]) * 16 + Color::get_hex_value(hex[6]);
        let mut alpha:f32 = 1.0;
        if hex_code.len() > 7 // alpha value also exists in hex code
        {
            let alpha255: u8 = Color::get_hex_value(hex[7]) * 16 + Color::get_hex_value(hex[8]);
            alpha = alpha255 as f32 / 255.0;
        }
        Color::new(red, green, blue, alpha)
    }

   fn get_hex_value(mut ch: char) -> u8 {
        ch = ch.to_ascii_lowercase();
        if ch == 'a' {
            10
        }
        else if ch == 'b' {
            return 11;
        }
        else if ch == 'c' {
            return 12;
        }
        else if ch == 'd' {
            return 13;
        }
        else if ch == 'e' {
            return 14;
        }
        else if ch == 'f' {
            return 15
        }
        else {
            return ch.to_digit(10).unwrap() as u8;
        }
        
    }

    /// Red value
    pub fn red(&self) -> u8 {
        self.r
    }

    /// Green value
    pub fn green(&self) -> u8 {
        self.g
    }

    /// Blue value
    pub fn blue(&self) -> u8 {
        self.b
    }

    /// Alpha value
    pub fn alpha(&self) -> f32 {
        self.a
    }

    pub fn to_linear(&self) -> wgpu::Color{
        let (r, g, b) = ((self.red() as f32 / 255.) as f64,
                         (self.green() as f32 / 255.) as f64,
                         (self.blue() as f32 / 255.) as f64);

        let f = |x: f64| {
            if x > 0.04045 {
                ((x + 0.055) / 1.055).powf(2.4)
            } else {
                x / 12.92
            }
        };
        wgpu::Color {
            r: f(r),
            g: f(g),
            b: f(b),
            a: self.alpha() as f64,
        }
    }

    pub(crate) fn to_texture_path(&self)-> String{
        format!("color-{}-{}-{}-{}", self.red(), self.green(), self.blue(), self.alpha())
    }
}

impl From<&Color> for GlColor {
    fn from(val: &Color) -> Self {
        GlColor {
            r: val.r as f32 / 255.,
            g: val.g as f32 / 255.,
            b: val.b as f32 / 255.,
            a: val.a,
        }
    }
}

impl ToString for Color{
    fn to_string(&self) -> String {
        format!("r{:?}g{:?}b{:?}a{:?}", self.r,self.g,self.b,self.a)
    }
}
