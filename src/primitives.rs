use web_sys::{
    WebGl2RenderingContext,
    WebGlBuffer,
    WebGlProgram,
    WebGlUniformLocation,
};

use crate::colour::Colour;
use crate::matrix::Matrix4F;
use crate::ID_MATRIX;

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
    position_buffer:               WebGlBuffer,
    colour:                        Colour,
    position_attribute_location:   i32,
    colour_uniform_location:       WebGlUniformLocation,
    model_matrix_uniform_location: WebGlUniformLocation,
    program:                       &'a WebGlProgram,
}

impl<'a> Line<'a> {
    pub fn new(
        context: &WebGl2RenderingContext,
        a: Vertex,
        b: Vertex,
        colour: Colour,
        program: &'a WebGlProgram,
    ) -> Line<'a> {
        let vertices = [a.x, a.y, a.z, b.x, b.y, b.z];

        let buffer = context.create_buffer().unwrap();

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

        let position_attribute_location =
            context.get_attrib_location(program, "position");

        let colour_uniform_location = context
            .get_uniform_location(&program, "colour")
            .expect("Missing \"colour\" uniform in program");

        let model_matrix_uniform_location = context
            .get_uniform_location(&program, "model_matrix")
            .expect("Missing \"model_matrix\" uniform in program");

        return Line {
            position_buffer: buffer,
            colour,
            program,
            position_attribute_location,
            colour_uniform_location,
            model_matrix_uniform_location,
        };
    }
}

impl<'a> Draw for Line<'a> {
    fn draw(
        &self,
        context: &web_sys::WebGl2RenderingContext,
        model_matrix: Option<Matrix4F>,
    ) -> Result<(), String> {
        context.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&self.position_buffer),
        );

        context.enable_vertex_attrib_array(
            self.position_attribute_location as u32,
        );

        context.vertex_attrib_pointer_with_i32(
            self.position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );

        context.use_program(Some(self.program));

        context.uniform4f(
            Some(&self.colour_uniform_location),
            self.colour.r,
            self.colour.g,
            self.colour.b,
            self.colour.a,
        );

        context.uniform_matrix4fv_with_f32_array(
            Some(&self.model_matrix_uniform_location),
            false,
            &model_matrix.unwrap_or(ID_MATRIX),
        );

        context.draw_arrays(WebGl2RenderingContext::LINES, 0, 2);

        return Ok(());
    }
}
