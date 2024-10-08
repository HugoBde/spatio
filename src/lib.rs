mod ball;
mod boxx;
mod cartesian_axis;
mod colour;
mod matrix;
mod primitives;
mod triangle;
mod utils;

use std::cell::RefCell;
use std::f32::consts::FRAC_PI_3;
use std::rc::Rc;

use ball::Ball;
use colour::Colour;
use matrix::*;
use primitives::{Draw, Vertex};
use utils::{compile_shader, link_program};
use web_sys::wasm_bindgen::prelude::*;
use web_sys::{
    window,
    Document,
    HtmlCanvasElement,
    HtmlInputElement,
    InputEvent,
    WebGl2RenderingContext,
    WebGlProgram,
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

        in vec4 position;

        uniform mat4 u_matrix;

        void main() {
            gl_Position = u_matrix * position;
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

    run(&document, &canvas, &context, &program)?;

    return Ok(());
}

fn init(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, JsValue> {
    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    context.viewport(0, 0, 800, 800);
    return Ok(context);
}

fn run(
    document: &Document,
    canvas: &HtmlCanvasElement,
    context: &WebGl2RenderingContext,
    program: &WebGlProgram,
) -> Result<(), JsValue> {
    let x = Rc::new(RefCell::new(0.0));
    let y = Rc::new(RefCell::new(0.0));
    let z = Rc::new(RefCell::new(0.0));

    let tx = Rc::new(RefCell::new(0.0));
    let ty = Rc::new(RefCell::new(0.0));
    let tz = Rc::new(RefCell::new(-3.0));

    let fov = Rc::new(RefCell::new(FRAC_PI_3));
    let near = Rc::new(RefCell::new(1.0));
    let far = Rc::new(RefCell::new(-2000.0));

    {
        let x = x.clone();

        utils::create_input_handler_f32(
            document,
            "slider_x",
            Box::new(move |event: InputEvent| {
                let slider: HtmlInputElement =
                    event.target().unwrap().dyn_into().unwrap();
                *x.borrow_mut() = slider.value_as_number() as f32;
            }),
        );
    }

    {
        let y = y.clone();

        utils::create_input_handler_f32(
            document,
            "slider_y",
            Box::new(move |event: InputEvent| {
                let slider: HtmlInputElement =
                    event.target().unwrap().dyn_into().unwrap();
                *y.borrow_mut() = slider.value_as_number() as f32;
            }),
        );
    }

    {
        let z = z.clone();

        utils::create_input_handler_f32(
            document,
            "slider_z",
            Box::new(move |event: InputEvent| {
                let slider: HtmlInputElement =
                    event.target().unwrap().dyn_into().unwrap();
                *z.borrow_mut() = slider.value_as_number() as f32;
            }),
        );
    }

    {
        let tx = tx.clone();
        utils::create_input_handler_f32(
            document,
            "slider_tx",
            Box::new(move |event: InputEvent| {
                let slider: HtmlInputElement =
                    event.target().unwrap().dyn_into().unwrap();
                *tx.borrow_mut() = slider.value_as_number() as f32;
            }),
        );
    }

    {
        let ty = ty.clone();

        utils::create_input_handler_f32(
            document,
            "slider_ty",
            Box::new(move |event: InputEvent| {
                let slider: HtmlInputElement =
                    event.target().unwrap().dyn_into().unwrap();
                *ty.borrow_mut() = slider.value_as_number() as f32;
            }),
        );
    }

    {
        let tz = tz.clone();

        utils::create_input_handler_f32(
            document,
            "slider_tz",
            Box::new(move |event: InputEvent| {
                let slider: HtmlInputElement =
                    event.target().unwrap().dyn_into().unwrap();
                *tz.borrow_mut() = slider.value_as_number() as f32;
            }),
        );
    }

    {
        let fov = fov.clone();

        utils::create_input_handler_f32(
            document,
            "slider_fov",
            Box::new(move |event: InputEvent| {
                let slider: HtmlInputElement =
                    event.target().unwrap().dyn_into().unwrap();
                *fov.borrow_mut() = slider.value_as_number() as f32;
            }),
        );
    }

    {
        let near = near.clone();

        utils::create_input_handler_f32(
            document,
            "slider_near",
            Box::new(move |event: InputEvent| {
                let slider: HtmlInputElement =
                    event.target().unwrap().dyn_into().unwrap();
                *near.borrow_mut() = slider.value_as_number() as f32;
            }),
        );
    }

    {
        let far = far.clone();

        utils::create_input_handler_f32(
            document,
            "slider_far",
            Box::new(move |event: InputEvent| {
                let slider: HtmlInputElement =
                    event.target().unwrap().dyn_into().unwrap();
                *far.borrow_mut() = slider.value_as_number() as f32;
            }),
        );
    }

    let b = Ball::new(
        &context,
        Vertex::new(0.0, 0.0, 0.0),
        1.0,
        20,
        Colour::RED,
        program.clone(),
    );


    let draw_routine = Rc::new(RefCell::new(None));
    let draw_routine_launcher = draw_routine.clone();

    // Draw loop
    {
        let x = x.clone();
        let y = y.clone();
        let z = z.clone();

        let tx = tx.clone();
        let ty = ty.clone();
        let tz = tz.clone();

        let fov = fov.clone();
        let near = near.clone();
        let far = far.clone();

        let context = context.clone();
        let canvas = canvas.clone();

        *draw_routine_launcher.borrow_mut() =
            Some(Closure::<dyn FnMut()>::new(move || {
                let x = *x.borrow();
                let y = *y.borrow();
                let z = *z.borrow();

                let tx = *tx.borrow();
                let ty = *ty.borrow();
                let tz = *tz.borrow();

                utils::clear_context(&context);
                utils::resize_canvas(&canvas, &context);

                let transforms = [
                    matrix::perspective_matrix(
                        *fov.borrow(),
                        1.0,
                        *near.borrow(),
                        *far.borrow(),
                    ),
                    matrix::translate_matrix(tx, ty, tz),
                    matrix::rotate_x_matrix(x),
                    matrix::rotate_y_matrix(y),
                    matrix::rotate_z_matrix(z),
                ];

                let uniform_matrix = matrix::mat_mul_many(&transforms);

                b.draw(&context, Some(uniform_matrix)).unwrap();

                utils::request_animation_frame(
                    draw_routine.borrow().as_ref().unwrap(),
                );
            }));
    }

    // Launch the loop
    utils::request_animation_frame(
        draw_routine_launcher.borrow().as_ref().unwrap(),
    );

    return Ok(());
}
