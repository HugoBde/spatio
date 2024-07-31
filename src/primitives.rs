use web_sys::{WebGl2RenderingContext, WebGlProgram};

pub trait Draw {
    fn draw(&self, context: &WebGl2RenderingContext, program: &WebGlProgram) -> Result<(), String>;
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex {
            x,
            y,
            z,
        }
    }
}

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
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const MAGENTA: Colour = Colour {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const RED: Colour = Colour {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const YELLOW: Colour = Colour {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

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
}

pub struct Line {
    pub a:      Vertex,
    pub b:      Vertex,
    pub colour: Colour,
}

impl Draw for Line {
    fn draw(&self, context: &WebGl2RenderingContext, program: &WebGlProgram) -> Result<(), String> {
        let position_attribute_location = context.get_attrib_location(&program, "position");
        let vertices = [self.a.x, self.a.y, self.a.z, self.b.x, self.b.y, self.b.z];

        let buffer = context.create_buffer().ok_or("failed to create buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        let vao = context
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;

        context.bind_vertex_array(Some(&vao));

        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );

        context.enable_vertex_attrib_array(position_attribute_location as u32);
        context.bind_vertex_array(Some(&vao));
        context.line_width(5.0);

        let colour_uniform_location = context
            .get_uniform_location(&program, "colour")
            .expect("Missing \"colour\" uniform in program");

        context.uniform4f(
            Some(&colour_uniform_location),
            self.colour.r,
            self.colour.g,
            self.colour.b,
            self.colour.a,
        );

        context.draw_arrays(WebGl2RenderingContext::LINES, 0, 2);

        return Ok(());
    }
}
