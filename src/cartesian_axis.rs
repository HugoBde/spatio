use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::boxx::Box;
use crate::colour::Colour;
use crate::matrix::Matrix4F;
use crate::primitives::Draw;

pub struct CartesianAxis<'a> {
    boxes: [Box<'a>; 3],
}

impl<'a> CartesianAxis<'a> {
    pub fn new(
        context: &WebGl2RenderingContext,
        program: &'a WebGlProgram,
    ) -> CartesianAxis<'a> {
        let width = 0.01;
        // let o = Vertex::new(0.0, 0.0, 0.0);
        // let i = Vertex::new(1.0, 0.0, 0.0);
        // let j = Vertex::new(0.0, 1.0, 0.0);
        // let k = Vertex::new(0.0, 0.0, 1.0);

        // let x_axis = Line::new(context, o, i, Colour::RED, program);
        // let y_axis = Line::new(context, o, j, Colour::GREEN, program);
        // let z_axis = Line::new(context, o, k, Colour::BLUE, program);
        let x_axis = Box::new(
            context,
            width,
            0.0,
            -width,
            -width,
            1.0,
            width,
            Colour::RED,
            program,
        );
        let y_axis = Box::new(
            context,
            1.0,
            -width,
            -width,
            0.0,
            width,
            width,
            Colour::GREEN,
            program,
        );
        let z_axis = Box::new(
            context,
            width,
            -width,
            -1.0,
            -width,
            width,
            0.0,
            Colour::BLUE,
            program,
        );

        return CartesianAxis {
            boxes: [x_axis, y_axis, z_axis],
        };
    }
}

impl<'a> Draw for CartesianAxis<'a> {
    fn draw(
        &self,
        context: &WebGl2RenderingContext,
        model_matrix: Option<Matrix4F>,
    ) -> Result<(), String> {
        for b in &self.boxes {
            b.draw(context, model_matrix)?;
        }
        return Ok(());
    }
}
