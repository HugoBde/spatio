use web_sys::WebGlProgram;

use crate::colour::Colour;
use crate::matrix::Matrix4F;
use crate::primitives::{Draw, Vertex};
use crate::triangle::Triangle;

pub struct Box<'a> {
    triangles: [Triangle<'a>; 12],
}

impl<'a> Box<'a> {
    pub fn new(
        top: f32,
        left: f32,
        front: f32,
        bottom: f32,
        right: f32,
        back: f32,
        program: &'a WebGlProgram,
    ) -> Box {
        //     4------5
        //    /|     /|
        //   / |    / |
        //  0------1  |
        //  |  7---|--6
        //  | /    | /
        //  |/     |/
        //  3------2
        let vertices = [
            // 0
            Vertex {
                x: left,
                y: top,
                z: front,
            },
            // 1
            Vertex {
                x: right,
                y: top,
                z: front,
            },
            // 2
            Vertex {
                x: right,
                y: bottom,
                z: front,
            },
            // 3
            Vertex {
                x: left,
                y: bottom,
                z: front,
            },
            // 4
            Vertex {
                x: left,
                y: top,
                z: back,
            },
            // 5
            Vertex {
                x: right,
                y: top,
                z: back,
            },
            // 6
            Vertex {
                x: right,
                y: bottom,
                z: back,
            },
            // 7
            Vertex {
                x: left,
                y: bottom,
                z: back,
            },
        ];

        //     4------5
        //    /|     /|
        //   / |    / |
        //  0------1  |
        //  |  7---|--6
        //  | /    | /
        //  |/     |/
        //  3------2
        //
        let triangles: [Triangle; 12] = [
            Triangle::new(
                vertices[0],
                vertices[1],
                vertices[3],
                Colour::RED,
                program,
            ),
            Triangle::new(
                vertices[1],
                vertices[2],
                vertices[3],
                Colour::PURPLE,
                program,
            ),
            Triangle::new(
                vertices[4],
                vertices[5],
                vertices[0],
                Colour::YELLOW,
                program,
            ),
            Triangle::new(
                vertices[5],
                vertices[1],
                vertices[0],
                Colour::GREEN,
                program,
            ),
            Triangle::new(
                vertices[5],
                vertices[6],
                vertices[1],
                Colour::CYAN,
                program,
            ),
            Triangle::new(
                vertices[6],
                vertices[2],
                vertices[1],
                Colour::BLUE,
                program,
            ),
            Triangle::new(
                vertices[6],
                vertices[7],
                vertices[2],
                Colour::MAGENTA,
                program,
            ),
            Triangle::new(
                vertices[7],
                vertices[3],
                vertices[2],
                Colour::PINK,
                program,
            ),
            Triangle::new(
                vertices[7],
                vertices[4],
                vertices[3],
                Colour::WHITE,
                program,
            ),
            Triangle::new(
                vertices[4],
                vertices[0],
                vertices[3],
                Colour::GREY,
                program,
            ),
            Triangle::new(
                vertices[5],
                vertices[4],
                vertices[6],
                Colour::PURPLE,
                program,
            ),
            Triangle::new(
                vertices[4],
                vertices[6],
                vertices[7],
                Colour::LIGHT_BLUE,
                program,
            ),
        ];

        return Box {
            triangles,
        };
    }
}

impl<'a> Draw for Box<'a> {
    fn draw(
        &self,
        context: &web_sys::WebGl2RenderingContext,
        model_matrix: Option<Matrix4F>,
    ) -> Result<(), String> {
        for triangle in &self.triangles {
            triangle.draw(context, model_matrix)?;
        }

        return Ok(());
    }
}
