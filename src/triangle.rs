use web_sys::WebGl2RenderingContext;

use crate::primitives::{Colour, Draw, Vertex};

pub struct Triangle {
    a:      Vertex,
    b:      Vertex,
    c:      Vertex,
    colour: Colour,
}

impl Triangle {
    pub fn new(a: Vertex, b: Vertex, c: Vertex, colour: Colour) -> Triangle {
        Triangle {
            a,
            b,
            c,
            colour,
        }
    }
}

impl Draw for Triangle {
    fn draw(&self, context: &web_sys::WebGl2RenderingContext, program: &web_sys::WebGlProgram) -> Result<(), String> {
        let vertices = [
            self.a.x, self.a.y, self.a.z, self.b.x, self.b.y, self.b.z, self.c.x, self.c.y, self.c.z,
        ];

        let position_attribute_location = context.get_attrib_location(&program, "position");

        let buffer = context.create_buffer().ok_or("failed to create buffer")?;

        let vao = context
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        context.bind_vertex_array(Some(&vao));


        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            4,
            0,
        );

        context.enable_vertex_attrib_array(position_attribute_location as u32);

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

        context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);

        return Ok(());
    }
}
