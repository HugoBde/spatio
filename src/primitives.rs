#![allow(dead_code)]
use web_sys::{WebGl2RenderingContext, WebGlProgram};
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Colour {
    r: f32,
    g: f32,
    b: f32,
}

impl Colour {
    fn from_u8(r: u8, g: u8, b: u8) -> Colour {
        Colour {
            r: r as f32 / 255.,
            g: g as f32 / 255.,
            b: b as f32 / 255.,
        }
    }

    fn from_f32(r: f32, g: f32, b: f32) -> Colour {
        Colour {
            r,
            g,
            b,
        }
    }
}

pub fn draw_line(
    context: &WebGl2RenderingContext,
    program: &WebGlProgram,
    a: &Vertex,
    b: &Vertex,
) -> Result<(), String> {
    let position_attribute_location = context.get_attrib_location(&program, "position");
    let vertices = [a.x, a.y, a.z, b.x, b.y, b.z];

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

    context.draw_arrays(WebGl2RenderingContext::LINES, 0, 2);

    return Ok(());
}
