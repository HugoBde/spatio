use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{
    window,
    Document,
    HtmlCanvasElement,
    HtmlInputElement,
    InputEvent,
    WebGl2RenderingContext,
    WebGlProgram,
    WebGlShader,
};

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        return Ok(shader);
    }

    return Err(context
        .get_shader_info_log(&shader)
        .unwrap_or_else(|| String::from("Unknown error creating shader")));
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create program"))?;
    context.attach_shader(&program, &vert_shader);
    context.attach_shader(&program, &frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        return Ok(program);
    }

    return Err(context.get_program_info_log(&program).unwrap_or_else(|| {
        String::from("Unknown error creating program object")
    }));
}

pub fn clear_context(context: &WebGl2RenderingContext) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    context.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

pub fn resize_canvas(
    canvas: &HtmlCanvasElement,
    context: &WebGl2RenderingContext,
) {
    let pixel_width = canvas.client_width();
    let pixel_height = canvas.client_height();

    if pixel_width as u32 != canvas.width() ||
        pixel_height as u32 != canvas.height()
    {
        canvas.set_width(pixel_width as u32);
        canvas.set_height(pixel_height as u32);
        context.viewport(0, 0, 800, 800);
    }
}

pub fn create_input_handler_f32(
    document: &Document,
    id: &str,
    handler: Box<dyn FnMut(InputEvent)>,
) {
    let js_closure = Closure::<dyn FnMut(InputEvent)>::new(handler);
    let input: HtmlInputElement =
        document.get_element_by_id(id).unwrap().dyn_into().unwrap();
    input.set_oninput(Some(js_closure.as_ref().unchecked_ref()));
    js_closure.forget();
}
