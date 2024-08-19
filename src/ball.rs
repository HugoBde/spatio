use std::f32::consts::{FRAC_PI_2, PI};

use web_sys::{
    WebGl2RenderingContext,
    WebGlBuffer,
    WebGlProgram,
    WebGlUniformLocation,
};

use crate::colour::Colour;
use crate::primitives::{Draw, Vertex};
use crate::ID_MATRIX;

pub struct Ball {
    position_buffer:             WebGlBuffer,
    indices_buffer:              WebGlBuffer,
    indices_count:               usize,
    position_attribute_location: i32,
    colour_uniform_location:     WebGlUniformLocation,
    colour:                      Colour,
    uniform_matrix_location:     WebGlUniformLocation,
    program:                     WebGlProgram,
}

impl Ball {
    pub fn new(
        context: &WebGl2RenderingContext,
        center: Vertex,
        radius: f32,
        precision: u16,
        colour: Colour,
        program: WebGlProgram,
    ) -> Ball {
        let mut vertices: Vec<f32> = vec![];

        let step = PI / precision as f32;

        let mut y_angle = FRAC_PI_2;

        let mut xz_angle: f32 = 0.0;

        for _ in 0..precision {
            let y_coord = center.y + radius * y_angle.sin();
            let radius = radius * y_angle.cos();

            for _ in 0..precision {
                vertices.push(center.x + radius * xz_angle.cos());
                vertices.push(y_coord);
                vertices.push(center.z + radius * xz_angle.sin());

                vertices.push(center.x - radius * xz_angle.cos());
                vertices.push(y_coord);
                vertices.push(center.z - radius * xz_angle.sin());

                xz_angle += step;
            }

            y_angle += step;
        }

        let mut triangles: Vec<u16> = vec![];

        for y_stripe in 0..(precision - 1) {
            for square in 0..(precision - 1) {
                let top_left = y_stripe * precision * 2 + square;
                let top_right = y_stripe * precision * 2 + square + 2; // + 2 because + 1 would be
                                                                       // the center symmetric point
                let bot_left = (y_stripe + 1) * precision * 2 + square;
                let bot_right = (y_stripe + 1) * precision * 2 + square + 2;

                // Triangle 1
                triangles.push(top_left);
                triangles.push(top_right);
                triangles.push(bot_left);

                // Triangle 2
                triangles.push(top_right);
                triangles.push(bot_left);
                triangles.push(bot_right);

                // Symmetrics
                // Triangle 1
                triangles.push(top_left + 1);
                triangles.push(top_right + 1);
                triangles.push(bot_left + 1);

                // Triangle 2
                triangles.push(top_right + 1);
                triangles.push(bot_left + 1);
                triangles.push(bot_right + 1);
            }
        }

        let position_buffer = context.create_buffer().unwrap();

        context.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&position_buffer),
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

        let indices_buffer = context.create_buffer().unwrap();

        context.bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&indices_buffer),
        );

        unsafe {
            let indices_array_buf_view = js_sys::Uint16Array::view(&triangles);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &indices_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        let position_attribute_location =
            context.get_attrib_location(&program, "position");


        let colour_uniform_location = context
            .get_uniform_location(&program, "colour")
            .expect("Missing \"colour\" uniform in program");

        let uniform_matrix_location = context
            .get_uniform_location(&program, "u_matrix")
            .expect("Missing \"u_matrix\" uniform in program");

        return Ball {
            position_buffer,
            indices_buffer,
            indices_count: triangles.len(),
            position_attribute_location,
            colour_uniform_location,
            uniform_matrix_location,
            colour,
            program,
        };
    }
}

impl Draw for Ball {
    fn draw(
        &self,
        context: &WebGl2RenderingContext,
        uniform_matrix: Option<crate::Matrix4F>,
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

        context.bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.indices_buffer),
        );

        context.use_program(Some(&self.program));

        self.colour.uniform(context, &self.colour_uniform_location);

        context.uniform_matrix4fv_with_f32_array(
            Some(&self.uniform_matrix_location),
            false,
            &uniform_matrix.unwrap_or(ID_MATRIX),
        );


        context.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            self.indices_count as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        );

        return Ok(());
    }
}
