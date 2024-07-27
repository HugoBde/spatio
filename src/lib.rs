mod primitives;
mod utils;

use utils::{compile_shader, link_program};
use web_sys::wasm_bindgen::prelude::*;
use web_sys::{window, WebGl2RenderingContext, WebGlProgram};


#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es

        in vec4 position;
        out vec4 var_position;

        void main() {
            var_position = position;
            gl_Position = position;
        }
        "##,
    )?;
    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es

        precision highp float;
        out vec4 outColor;
        in vec4 var_position;

        void main() {
            outColor = vec4(
                (var_position.x + 1.0) / 2.0,
                (var_position.y + 1.0) / 2.0,
                1,
                1
            );
        }
        "##,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));


    run(&context, &program)?;

    return Ok(());
}

fn run(context: &WebGl2RenderingContext, program: &WebGlProgram) -> Result<(), JsValue> {
    let point_a = primitives::Vertex {
        x: -0.5,
        y: 0.0,
        z: 0.0,
    };
    let point_b = primitives::Vertex {
        x: 0.5, y: 0.0, z: 0.0
    };
    let point_c = primitives::Vertex {
        x: 0.0, y: 0.5, z: 0.0
    };

    utils::clear_context(context);
    primitives::draw_line(context, program, &point_a, &point_b)?;
    primitives::draw_line(context, program, &point_a, &point_c)?;
    primitives::draw_line(context, program, &point_b, &point_c)?;

    return Ok(());
}
