mod boxx;
mod cartesian_axis;
mod colour;
mod matrix;
mod primitives;
mod triangle;
mod utils;

use std::cell::RefCell;
use std::rc::Rc;

use cartesian_axis::CartesianAxis;
use matrix::*;
use primitives::Draw;
use utils::{compile_shader, link_program};
use web_sys::wasm_bindgen::prelude::*;
use web_sys::{
    window,
    HtmlCanvasElement,
    WebGl2RenderingContext,
    WebGlProgram,
    Window,
};


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    let window = window().unwrap();

    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = init(&canvas)?;

    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es

        in vec3 position;

        uniform mat4 model_matrix;

        void main() {
            gl_Position = model_matrix * vec4(position, 1.2);
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

    run(&window, &context, &program)?;

    return Ok(());
}

fn init(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, JsValue> {
    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    context.viewport(0, 0, 2000, 2000);
    return Ok(context);
}

fn run(
    window: &Window,
    context: &WebGl2RenderingContext,
    program: &WebGlProgram,
) -> Result<(), JsValue> {
    utils::clear_context(context);

    // let cart = CartesianAxis::new(program);
    // cart.draw(context, None)?;
    //
    // let t = Triangle::new(
    //     context,
    //     Vertex::new(-0.5, 0.0, -0.3),
    //     Vertex::new(0.0, 0.5, -0.6),
    //     Vertex::new(0.5, 0.0, -0.6),
    //     Colour::PINK,
    //     program,
    // );
    //
    //
    // let b = boxx::Box::new(
    //     context,
    //     0.5,
    //     -0.5,
    //     -0.5,
    //     -0.5,
    //     0.5,
    //     0.5,
    //     Colour::RED,
    //     program,
    // );
    // b.draw(&context, None)?;
    // t.draw(&context, None)?;
    let t = Rc::new(RefCell::new(0.0));
    let ca = Rc::new(CartesianAxis::new(&context, program.clone()));

    let draw_routine = Rc::new(RefCell::new(None));
    let draw_routine_launcher = draw_routine.clone();

    {
        let t = t.clone();
        let ca = ca.clone();
        let context = context.clone();

        *draw_routine_launcher.borrow_mut() =
            Some(Closure::<dyn FnMut()>::new(move || {
                *t.borrow_mut() += 0.01;
                context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
                context.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);

                let transforms = [
                    matrix::new_rotate_x_matrix(*t.borrow()),
                    matrix::new_rotate_z_matrix(*t.borrow()),
                ];
                let model_matrix = matrix::mat_mul_many(&transforms);
                ca.draw(&context, Some(model_matrix)).unwrap();

                utils::request_animation_frame(
                    draw_routine.borrow().as_ref().unwrap(),
                );
            }));
    }

    utils::request_animation_frame(
        draw_routine_launcher.borrow().as_ref().unwrap(),
    );


    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    context.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    let transforms = [
        matrix::new_rotate_x_matrix(*t.borrow()),
        matrix::new_rotate_z_matrix(*t.borrow()),
    ];
    let model_matrix = matrix::mat_mul_many(&transforms);
    ca.draw(&context, Some(model_matrix)).unwrap();

    return Ok(());
}
