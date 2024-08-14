pub type Matrix4F = [f32; 16];
pub type Vector4F = [f32; 4];

pub fn rotate_x_matrix(angle: f32) -> Matrix4F {
    [
        1.,
        0.,
        0.,
        0.,
        0.,
        angle.cos(),
        -angle.sin(),
        0.,
        0.,
        angle.sin(),
        angle.cos(),
        0.,
        0.,
        0.,
        0.,
        1.,
    ]
}

pub fn rotate_y_matrix(angle: f32) -> Matrix4F {
    [
        angle.cos(),
        0.,
        angle.sin(),
        0.,
        0.,
        1.,
        0.,
        0.,
        -angle.sin(),
        0.,
        angle.cos(),
        0.,
        0.,
        0.,
        0.,
        1.,
    ]
}

pub fn rotate_z_matrix(angle: f32) -> Matrix4F {
    [
        angle.cos(),
        angle.sin(),
        0.,
        0.,
        -angle.sin(),
        angle.cos(),
        0.,
        0.,
        0.,
        0.,
        1.,
        0.,
        0.,
        0.,
        0.,
        1.,
    ]
}

pub fn translate_matrix(x: f32, y: f32, z: f32) -> Matrix4F {
    [1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., x, y, z, 1.]
}

#[allow(dead_code)]
pub fn scale_matrix(width: f32, height: f32, depth: f32) -> Matrix4F {
    [
        width, 0., 0., 0., 0., height, 0., 0., 0., 0., depth, 0., 0., 0., 0.,
        1.,
    ]
}

pub fn perspective_matrix(
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
) -> Matrix4F {
    let f = (fov / 2.0).tan().recip();
    let range_inverse = (near - far).recip();

    return [
        f / aspect_ratio,
        0.,
        0.,
        0.,
        0.,
        f,
        0.,
        0.,
        0.,
        0.,
        (near + far) * range_inverse,
        -1.,
        0.,
        0.,
        near * far * range_inverse * 2.0,
        0.,
    ];
}

pub fn mat_mul(a: &Matrix4F, b: &Matrix4F) -> Matrix4F {
    let a00 = a[0];
    let a01 = a[1];
    let a02 = a[2];
    let a03 = a[3];
    let a10 = a[4];
    let a11 = a[5];
    let a12 = a[6];
    let a13 = a[7];
    let a20 = a[8];
    let a21 = a[9];
    let a22 = a[10];
    let a23 = a[11];
    let a30 = a[12];
    let a31 = a[13];
    let a32 = a[14];
    let a33 = a[15];

    let b00 = b[0];
    let b01 = b[1];
    let b02 = b[2];
    let b03 = b[3];
    let b10 = b[4];
    let b11 = b[5];
    let b12 = b[6];
    let b13 = b[7];
    let b20 = b[8];
    let b21 = b[9];
    let b22 = b[10];
    let b23 = b[11];
    let b30 = b[12];
    let b31 = b[13];
    let b32 = b[14];
    let b33 = b[15];

    return [
        b00 * a00 + b01 * a10 + b02 * a20 + b03 * a30,
        b00 * a01 + b01 * a11 + b02 * a21 + b03 * a31,
        b00 * a02 + b01 * a12 + b02 * a22 + b03 * a32,
        b00 * a03 + b01 * a13 + b02 * a23 + b03 * a33,
        b10 * a00 + b11 * a10 + b12 * a20 + b13 * a30,
        b10 * a01 + b11 * a11 + b12 * a21 + b13 * a31,
        b10 * a02 + b11 * a12 + b12 * a22 + b13 * a32,
        b10 * a03 + b11 * a13 + b12 * a23 + b13 * a33,
        b20 * a00 + b21 * a10 + b22 * a20 + b23 * a30,
        b20 * a01 + b21 * a11 + b22 * a21 + b23 * a31,
        b20 * a02 + b21 * a12 + b22 * a22 + b23 * a32,
        b20 * a03 + b21 * a13 + b22 * a23 + b23 * a33,
        b30 * a00 + b31 * a10 + b32 * a20 + b33 * a30,
        b30 * a01 + b31 * a11 + b32 * a21 + b33 * a31,
        b30 * a02 + b31 * a12 + b32 * a22 + b33 * a32,
        b30 * a03 + b31 * a13 + b32 * a23 + b33 * a33,
    ];
}

pub fn mat_mul_many(matrices: &[Matrix4F]) -> Matrix4F {
    let mut output = ID_MATRIX;
    for matrix in matrices {
        output = mat_mul(&output, matrix);
    }
    return output;
}

pub const ID_MATRIX: Matrix4F = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
    1.0,
];

pub fn mat_vec_mul(mat: Matrix4F, vec: Vector4F) -> Vector4F {
    [
        mat[0] * vec[0] + mat[4] * vec[1] + mat[8] * vec[2] + mat[12] * vec[3],
        mat[1] * vec[0] + mat[5] * vec[1] + mat[9] * vec[2] + mat[13] * vec[3],
        mat[2] * vec[0] + mat[6] * vec[1] + mat[10] * vec[2] + mat[14] * vec[3],
        mat[3] * vec[0] + mat[7] * vec[1] + mat[11] * vec[2] + mat[15] * vec[3],
    ]
}
