use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}


#[allow(dead_code)]
impl Colour {
    pub const BLUE: Colour = Colour {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const CYAN: Colour = Colour {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const GREEN: Colour = Colour {
        r: 0.0,
        g: 0.5,
        b: 0.0,
        a: 1.0,
    };
    pub const GREY: Colour = Colour {
        r: 0.4,
        g: 0.4,
        b: 0.4,
        a: 1.0,
    };
    pub const LIGHT_BLUE: Colour = Colour {
        r: 0.55,
        g: 0.84,
        b: 0.95,
        a: 1.0,
    };
    pub const MAGENTA: Colour = Colour {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const ORANGE: Colour = Colour {
        r: 1.0,
        g: 0.75,
        b: 0.0,
        a: 1.0,
    };
    pub const PINK: Colour = Colour {
        r: 1.0,
        g: 0.1,
        b: 0.6,
        a: 1.0,
    };
    pub const PURPLE: Colour = Colour {
        r: 0.5,
        g: 0.0,
        b: 0.5,
        a: 1.0,
    };
    pub const RED: Colour = Colour {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE: Colour = Colour {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const YELLOW: Colour = Colour {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

    pub fn from_rgb_hex(_hex: &str) -> Colour {
        Colour::PURPLE
    }

    pub fn from_rgb_u8(r: u8, g: u8, b: u8) -> Colour {
        Colour {
            r: r as f32 / 255.,
            g: g as f32 / 255.,
            b: b as f32 / 255.,
            a: 1.0,
        }
    }

    pub fn from_rgb_f32(r: f32, g: f32, b: f32) -> Colour {
        Colour {
            r,
            g,
            b,
            a: 1.0,
        }
    }

    pub fn from_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Colour {
        Colour {
            r: r as f32 / 255.,
            g: g as f32 / 255.,
            b: b as f32 / 255.,
            a: a as f32 / 255.,
        }
    }

    pub fn from_rgba_f32(r: f32, g: f32, b: f32, a: f32) -> Colour {
        Colour {
            r,
            g,
            b,
            a,
        }
    }

    pub fn uniform(
        &self,
        context: &WebGl2RenderingContext,
        colour_uniform_location: &WebGlUniformLocation,
    ) {
        context.uniform4f(
            Some(colour_uniform_location),
            self.r,
            self.g,
            self.b,
            self.a,
        );
    }
}
