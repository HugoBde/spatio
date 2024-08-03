use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::colour::Colour;
use crate::matrix::Matrix4F;

pub trait Draw {
    fn draw(
        &self,
        context: &WebGl2RenderingContext,
        model_matrix: Option<Matrix4F>,
    ) -> Result<(), String>;
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex {
            x,
            y,
            z,
        }
    }
}


pub struct Line<'a> {
    pub a:       Vertex,
    pub b:       Vertex,
    pub colour:  Colour,
    pub program: &'a WebGlProgram,
}

impl<'a> Draw for Line<'a> {
    fn draw(
        &self,
        context: &web_sys::WebGl2RenderingContext,
        model_matrix: Option<Matrix4F>,
    ) -> Result<(), String> {
        let position_attribute_location =
            context.get_attrib_location(&self.program, "position");
        let vertices =
            [self.a.x, self.a.y, self.a.z, self.b.x, self.b.y, self.b.z];

        let buffer =
            context.create_buffer().ok_or("failed to create buffer")?;
        context
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let positions_array_buf_view =
                js_sys::Float32Array::view(&vertices);

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
            .get_uniform_location(&self.program, "colour")
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
