use crate::primitives::{Colour, Draw, Vertex};
use crate::triangle::Triangle;

pub struct Box {
    triangles: [Triangle; 12],
}

impl Default for Box {
    fn default() -> Box {
        Box::new(0.5, -0.5, -0.5, -0.5, 0.5, 0.5)
    }
}

impl Box {
    pub fn new(top: f32, left: f32, front: f32, bottom: f32, right: f32, back: f32) -> Box {
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
            Triangle::new(vertices[0], vertices[1], vertices[3], Colour::RED),
            Triangle::new(vertices[1], vertices[2], vertices[3], Colour::RED),
            Triangle::new(vertices[4], vertices[5], vertices[0], Colour::BLUE),
            Triangle::new(vertices[5], vertices[1], vertices[0], Colour::BLUE),
            Triangle::new(vertices[5], vertices[6], vertices[1], Colour::GREEN),
            Triangle::new(vertices[6], vertices[2], vertices[1], Colour::GREEN),
            Triangle::new(vertices[6], vertices[7], vertices[2], Colour::CYAN),
            Triangle::new(vertices[7], vertices[3], vertices[2], Colour::CYAN),
            Triangle::new(vertices[7], vertices[4], vertices[3], Colour::MAGENTA),
            Triangle::new(vertices[4], vertices[0], vertices[3], Colour::MAGENTA),
            Triangle::new(vertices[5], vertices[4], vertices[6], Colour::YELLOW),
            Triangle::new(vertices[4], vertices[6], vertices[7], Colour::YELLOW),
        ];

        return Box {
            triangles,
        };
    }
}

impl Draw for Box {
    fn draw(&self, context: &web_sys::WebGl2RenderingContext, program: &web_sys::WebGlProgram) -> Result<(), String> {
        for triangle in &self.triangles {
            triangle.draw(context, program)?;
        }

        return Ok(());
    }
}
