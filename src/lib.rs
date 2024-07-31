mod boxx;
mod primitives;
mod triangle;
mod utils;

use primitives::{Colour, Draw, Vertex};
use triangle::Triangle;
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

        in vec3 position;

        void main() {
            gl_Position = vec4(position, 1.0);
        }
        "##,
    )?;
    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es

        precision highp float;
        out vec4 outColor;

        uniform vec4 colour;

        void main() {
            outColor = colour;
        }
        "##,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    context.enable(WebGl2RenderingContext::DEPTH_TEST);
    context.depth_func(WebGl2RenderingContext::LEQUAL);

    run(&context, &program)?;

    return Ok(());
}

fn run(context: &WebGl2RenderingContext, program: &WebGlProgram) -> Result<(), JsValue> {
    utils::clear_context(context);
    // let o = Vertex::new(0., 0., 0.);
    // let v = Vertex::new(0.5, 0.0, 0.0);
    // let u = Vertex::new(0.0, 0.5, 0.0);
    // let w = Vertex::new(0.0, 0.0, 0.5);
    //
    // let line = Line {
    //     a:      o,
    //     b:      v,
    //     colour: Colour::RED,
    // };
    // line.draw(context, program)?;
    // let line = Line {
    //     a:      o,
    //     b:      u,
    //     colour: Colour::GREEN,
    // };
    // line.draw(context, program)?;
    //
    // let line = Line {
    //     a:      o,
    //     b:      w,
    //     colour: Colour::BLUE,
    // };
    // line.draw(context, program)?;

    let t = Triangle::new(
        Vertex::new(0.0, 0.5, 0.0),
        Vertex::new(-0.5, -0.5, 0.0),
        Vertex::new(0.5, -0.5, 0.0),
        Colour::RED,
    );

    t.draw(&context, &program)?;

    return Ok(());
}
