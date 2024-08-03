use web_sys::WebGlProgram;

use crate::colour::Colour;
use crate::matrix::Matrix4F;
use crate::primitives::{Draw, Line, Vertex};

pub struct CartesianAxis<'a> {
    program: &'a WebGlProgram,
}

impl<'a> CartesianAxis<'a> {
    pub fn new(program: &'a WebGlProgram) -> CartesianAxis<'a> {
        CartesianAxis {
            program,
        }
    }
}

impl<'a> Draw for CartesianAxis<'a> {
    fn draw(
        &self,
        context: &web_sys::WebGl2RenderingContext,
        model_matrix: Option<Matrix4F>,
    ) -> Result<(), String> {
        let o = Vertex::new(0., 0., 0.);
        let v = Vertex::new(0.5, 0.0, 0.0);
        let u = Vertex::new(0.0, 0.5, 0.0);
        let w = Vertex::new(0.0, 0.0, 0.5);

        let line = Line {
            a:       o,
            b:       v,
            colour:  Colour::RED,
            program: self.program,
        };
        line.draw(context, model_matrix)?;
        let line = Line {
            a:       o,
            b:       u,
            colour:  Colour::GREEN,
            program: self.program,
        };
        line.draw(context, model_matrix)?;

        let line = Line {
            a:       o,
            b:       w,
            colour:  Colour::BLUE,
            program: self.program,
        };
        line.draw(context, model_matrix)?;

        return Ok(());
    }
}
