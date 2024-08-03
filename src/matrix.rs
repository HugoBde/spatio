pub type Matrix4F = [f32; 16];

pub fn new_rotate_x_matrix(angle: f32) -> Matrix4F {
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

pub fn new_rotate_y_matrix(angle: f32) -> Matrix4F {
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

pub fn new_rotate_z_matrix(angle: f32) -> Matrix4F {
    [
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
        0.,
        0.,
        0.,
        0.,
        1.,
    ]
}

pub fn new_translate_matrix(x: f32, y: f32, z: f32) -> Matrix4F {
    [1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., x, y, z, 1.]
}

pub fn new_scale_matrix(width: f32, height: f32, depth: f32) -> Matrix4F {
    [width, 0., 0., 0., 0., height, 0., 0., 0., 0., depth, 0., 0., 0., 0., 1.]
}

pub fn new_perspective_matrix(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Matrix4F {
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
    let mut result = [0.0; 16];

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

    // Cache only the current line of the second matrix
    let mut b0 = b[0];
    let mut b1 = b[1];
    let mut b2 = b[2];
    let mut b3 = b[3];

    result[0] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
    result[1] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
    result[2] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
    result[3] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

    b0 = b[4];
    b1 = b[5];
    b2 = b[6];
    b3 = b[7];

    result[4] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
    result[5] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
    result[6] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
    result[7] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

    b0 = b[8];
    b1 = b[9];
    b2 = b[10];
    b3 = b[11];

    result[8] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
    result[9] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
    result[10] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
    result[11] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

    b0 = b[12];
    b1 = b[13];
    b2 = b[14];
    b3 = b[15];

    result[12] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
    result[13] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
    result[14] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
    result[15] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

    return result;
}

pub fn mat_mul_many(matrices: &[Matrix4F]) -> Matrix4F {
    let mut output = ID_MATRIX;
    for matrix in matrices {
        output = mat_mul(&output, matrix);
    }
    return output;
}

pub const ID_MATRIX: Matrix4F = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];
