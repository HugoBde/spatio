use web_sys::{
    WebGl2RenderingContext,
    WebGlBuffer,
    WebGlProgram,
    WebGlUniformLocation,
};

use crate::colour::Colour;
use crate::matrix::Matrix4F;
use crate::primitives::Draw;
use crate::ID_MATRIX;

pub struct Box<'a> {
    position_buffer:               WebGlBuffer,
    indices_buffer:                WebGlBuffer,
    position_attribute_location:   i32,
    colour_uniform_location:       WebGlUniformLocation,
    colour:                        Colour,
    model_matrix_uniform_location: WebGlUniformLocation,
    program:                       &'a WebGlProgram,
}

impl<'a> Box<'a> {
    pub fn new(
        context: &WebGl2RenderingContext,
        top: f32,
        left: f32,
        front: f32,
        bottom: f32,
        right: f32,
        back: f32,
        colour: Colour,
        program: &'a WebGlProgram,
    ) -> Box<'a> {
        //     4------5
        //    /|     /|
        //   / |    / |
        //  0------1  |
        //  |  7---|--6
        //  | /    | /
        //  |/     |/
        //  3------2
        let vertices = [
            left, top, front, // 0
            right, top, front, // 1
            right, bottom, front, // 2
            left, bottom, front, // 3
            left, top, back, // 4
            right, top, back, // 5
            right, bottom, back, // 6
            left, bottom, back, // 7
        ];

        //     4------5
        //    /|     /|
        //   / |    / |
        //  0------1  |
        //  |  7---|--6
        //  | /    | /
        //  |/     |/
        //  3------2
        //
        let triangles: [u16; 36] = [
            0, 1, 2, // FRONT 1
            0, 2, 3, // FRONT 2
            4, 5, 1, // TOP 1
            4, 1, 0, // TOP 2
            5, 6, 2, // RIGHT 1
            5, 2, 1, // RIGHT 2
            6, 7, 2, // BOTTOM 1
            6, 2, 3, // BOTTOM 2
            7, 4, 0, // LEFT 1
            7, 0, 3, // LEFT 2
            5, 4, 6, // BACK 1
            5, 6, 7, // BACK 2
        ];

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
            context.get_attrib_location(program, "position");

        let colour_uniform_location = context
            .get_uniform_location(&program, "colour")
            .expect("Missing \"colour\" uniform in program");

        let model_matrix_uniform_location = context
            .get_uniform_location(&program, "model_matrix")
            .expect("Missing \"model_matrix\" uniform in program");

        return Box {
            position_buffer,
            indices_buffer,
            position_attribute_location,
            colour_uniform_location,
            model_matrix_uniform_location,
            colour,
            program,
        };
    }
}

impl<'a> Draw for Box<'a> {
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

        context.bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.indices_buffer),
        );

        context.use_program(Some(self.program));

        self.colour.uniform(context, &self.colour_uniform_location);

        context.uniform_matrix4fv_with_f32_array(
            Some(&self.model_matrix_uniform_location),
            false,
            &model_matrix.unwrap_or(ID_MATRIX),
        );


        context.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            36,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        );

        return Ok(());
    }
}
