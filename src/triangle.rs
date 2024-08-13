use web_sys::{
    WebGl2RenderingContext,
    WebGlBuffer,
    WebGlProgram,
    WebGlUniformLocation,
};

use crate::colour::Colour;
use crate::primitives::{Draw, Vertex};
use crate::Matrix4F;

pub struct Triangle<'a> {
    positions_buffer:            WebGlBuffer,
    colour:                      Colour,
    position_attribute_location: i32,
    colour_uniform_location:     WebGlUniformLocation,
    program:                     &'a WebGlProgram,
}

impl<'a> Triangle<'a> {
    pub fn new(
        context: &WebGl2RenderingContext,
        a: Vertex,
        b: Vertex,
        c: Vertex,
        colour: Colour,
        program: &'a WebGlProgram,
    ) -> Triangle<'a> {
        let vertices = [a.x, a.y, a.z, b.x, b.y, b.z, c.x, c.y, c.z];

        let positions_buffer = context.create_buffer().unwrap();

        context.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&positions_buffer),
        );

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
            context.get_attrib_location(&program, "position");

        let colour_uniform_location = context
            .get_uniform_location(&program, "colour")
            .expect("Missing \"colour\" uniform in program");


        Triangle {
            positions_buffer,
            colour,
            position_attribute_location,
            colour_uniform_location,
            program,
        }
    }
}

impl<'a> Draw for Triangle<'a> {
    fn draw(
        &self,
        context: &web_sys::WebGl2RenderingContext,
        _uniform_matrix: Option<Matrix4F>,
    ) -> Result<(), String> {
        context.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&self.positions_buffer),
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

        context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);

        return Ok(());
    }
}
