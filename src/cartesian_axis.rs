use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::colour::Colour;
use crate::matrix::Matrix4F;
use crate::primitives::{Draw, Line, Vertex};

pub struct CartesianAxis<'a> {
    lines: [Line<'a>; 3],
}

impl<'a> CartesianAxis<'a> {
    pub fn new(
        context: &WebGl2RenderingContext,
        program: &'a WebGlProgram,
    ) -> CartesianAxis<'a> {
        let o = Vertex::new(0.0, 0.0, 0.0);
        let i = Vertex::new(1.0, 0.0, 0.0);
        let j = Vertex::new(0.0, 1.0, 0.0);
        let k = Vertex::new(0.0, 0.0, 1.0);

        let x_axis = Line::new(context, o, i, Colour::RED, program);
        let y_axis = Line::new(context, o, j, Colour::GREEN, program);
        let z_axis = Line::new(context, o, k, Colour::BLUE, program);

        return CartesianAxis {
            lines: [x_axis, y_axis, z_axis],
        };
    }
}

impl<'a> Draw for CartesianAxis<'a> {
    fn draw(
        &self,
        context: &WebGl2RenderingContext,
        model_matrix: Option<Matrix4F>,
    ) -> Result<(), String> {
        for line in &self.lines {
            line.draw(context, model_matrix)?;
        }
        return Ok(());
    }
}
