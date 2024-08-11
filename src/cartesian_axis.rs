use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::boxx::Box;
use crate::colour::Colour;
use crate::matrix::Matrix4F;
use crate::primitives::Draw;

pub struct CartesianAxis {
    boxes: [Box; 4],
}

impl<'a> CartesianAxis {
    pub fn new(
        context: &WebGl2RenderingContext,
        program: WebGlProgram,
    ) -> CartesianAxis {
        let width = 0.01;
        let origin = Box::new(
            context,
            width,
            -width,
            -width,
            -width,
            width,
            width,
            Colour::WHITE,
            program.clone(),
        );
        let x_axis = Box::new(
            context,
            width,
            width,
            -width,
            -width,
            1.0,
            width,
            Colour::RED,
            program.clone(),
        );
        let y_axis = Box::new(
            context,
            1.0,
            -width,
            -width,
            width,
            width,
            width,
            Colour::GREEN,
            program.clone(),
        );
        let z_axis = Box::new(
            context,
            width,
            -width,
            -1.0,
            -width,
            width,
            -width,
            Colour::BLUE,
            program,
        );

        return CartesianAxis {
            boxes: [origin, x_axis, y_axis, z_axis],
        };
    }
}

impl<'a> Draw for CartesianAxis {
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
