use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::colour::Colour;
use crate::primitives::{Draw, Vertex};
use crate::{matrix, Matrix4F};

pub struct Triangle<'a> {
    a:       Vertex,
    b:       Vertex,
    c:       Vertex,
    colour:  Colour,
    program: &'a WebGlProgram,
}

impl<'a> Triangle<'a> {
    pub fn new(
        a: Vertex,
        b: Vertex,
        c: Vertex,
        colour: Colour,
        program: &'a WebGlProgram,
    ) -> Triangle {
        Triangle {
            a,
            b,
            c,
            colour,
            program,
        }
    }
}

impl<'a> Draw for Triangle<'a> {
    fn draw(
        &self,
        context: &web_sys::WebGl2RenderingContext,
        _model_matrix: Option<Matrix4F>,
    ) -> Result<(), String> {
        let vertices = [
            self.a.x, self.a.y, self.a.z, self.b.x, self.b.y, self.b.z,
            self.c.x, self.c.y, self.c.z,
        ];

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

        let position_attribute_location =
            context.get_attrib_location(&self.program, "position");

        context.enable_vertex_attrib_array(position_attribute_location as u32);

        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );

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

        // let model_matrix_uniform_location = context
        //     .get_uniform_location(&self.program, "model_matrix")
        //     .expect("Missing \"model_matrix\" uniform in program");
        //
        // context.uniform_matrix4fv_with_f32_array(
        //     Some(&model_matrix_uniform_location),
        //     false,
        //     &model_matrix.unwrap_or(matrix::ID_MATRIX),
        // );

        context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);

        return Ok(());
    }
}
