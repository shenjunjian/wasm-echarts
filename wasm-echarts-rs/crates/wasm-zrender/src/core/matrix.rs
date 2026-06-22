//! 3×2 仿射变换矩阵，布局与 zrender / DOMMatrix 一致：
//! `[a, b, c, d, e, f]` 表示 `| a c e | / | b d f |`

pub type Matrix = [f32; 6];

const EPSILON: f32 = 5e-5;

pub fn create() -> Matrix {
    [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
}

pub fn identity(m: &mut Matrix) {
    *m = create();
}

pub fn copy(out: &mut Matrix, m: &Matrix) {
    *out = *m;
}

pub fn mul(out: &mut Matrix, m1: &Matrix, m2: &Matrix) {
    let out0 = m1[0] * m2[0] + m1[2] * m2[1];
    let out1 = m1[1] * m2[0] + m1[3] * m2[1];
    let out2 = m1[0] * m2[2] + m1[2] * m2[3];
    let out3 = m1[1] * m2[2] + m1[3] * m2[3];
    let out4 = m1[0] * m2[4] + m1[2] * m2[5] + m1[4];
    let out5 = m1[1] * m2[4] + m1[3] * m2[5] + m1[5];
    out[0] = out0;
    out[1] = out1;
    out[2] = out2;
    out[3] = out3;
    out[4] = out4;
    out[5] = out5;
}

pub fn translate(out: &mut Matrix, a: &Matrix, tx: f32, ty: f32) {
    out[0] = a[0];
    out[1] = a[1];
    out[2] = a[2];
    out[3] = a[3];
    out[4] = a[4] + tx;
    out[5] = a[5] + ty;
}

pub fn rotate(out: &mut Matrix, a: &Matrix, rad: f32) {
    let cos = rad.cos();
    let sin = rad.sin();
    let out0 = a[0] * cos + a[2] * sin;
    let out1 = a[1] * cos + a[3] * sin;
    let out2 = a[0] * -sin + a[2] * cos;
    let out3 = a[1] * -sin + a[3] * cos;
    out[0] = out0;
    out[1] = out1;
    out[2] = out2;
    out[3] = out3;
    out[4] = a[4];
    out[5] = a[5];
}

pub fn scale(out: &mut Matrix, a: &Matrix, sx: f32, sy: f32) {
    out[0] = a[0] * sx;
    out[1] = a[1] * sx;
    out[2] = a[2] * sy;
    out[3] = a[3] * sy;
    out[4] = a[4];
    out[5] = a[5];
}

pub fn is_not_around_zero(val: f32) -> bool {
    val > EPSILON || val < -EPSILON
}

pub fn to_dom_matrix(m: &Matrix) -> vl_convert_canvas2d::DOMMatrix {
    vl_convert_canvas2d::DOMMatrix::new(m[0], m[1], m[2], m[3], m[4], m[5])
}
