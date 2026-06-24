//! Point 值对象（对齐 zrender export.ts）

use rust_zrender::core::point::Point2;
use wasm_bindgen::prelude::*;

use super::geometry_util::parse_matrix;

#[wasm_bindgen]
pub struct Point {
    inner: Point2,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: Option<f64>, y: Option<f64>) -> Point {
        Point {
            inner: Point2::new(x.unwrap_or(0.0), y.unwrap_or(0.0)),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.inner.x
    }

    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, x: f64) {
        self.inner.x = x;
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.inner.y
    }

    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, y: f64) {
        self.inner.y = y;
    }

    pub fn copy(&mut self, other: &Point) -> Point {
        self.inner.copy_from(&other.inner);
        self.clone()
    }

    pub fn clone(&self) -> Point {
        Point {
            inner: self.inner,
        }
    }

    pub fn set(&mut self, x: f64, y: f64) -> Point {
        self.inner.set(x, y);
        self.clone()
    }

    pub fn equal(&self, other: &Point) -> bool {
        self.inner == other.inner
    }

    pub fn add(&mut self, other: &Point) -> Point {
        self.inner.add(&other.inner);
        self.clone()
    }

    pub fn sub(&mut self, other: &Point) -> Point {
        self.inner.sub(&other.inner);
        self.clone()
    }

    pub fn dot(&self, other: &Point) -> f64 {
        self.inner.dot(&other.inner)
    }

    pub fn len(&self) -> f64 {
        self.inner.len()
    }

    #[wasm_bindgen(js_name = lenSquare)]
    pub fn len_square(&self) -> f64 {
        self.inner.len_square()
    }

    pub fn normalize(&mut self) -> Point {
        self.inner.normalize();
        self.clone()
    }

    pub fn distance(&self, other: &Point) -> f64 {
        self.inner.distance(&other.inner)
    }

    #[wasm_bindgen(js_name = distanceSquare)]
    pub fn distance_square(&self, other: &Point) -> f64 {
        self.inner.distance_square(&other.inner)
    }

    pub fn negate(&mut self) -> Point {
        self.inner.negate();
        self.clone()
    }

    pub fn scale(&mut self, scalar: f64) {
        self.inner.scale(scalar);
    }

    #[wasm_bindgen(js_name = scaleAndAdd)]
    pub fn scale_and_add(&mut self, other: &Point, scalar: f64) {
        self.inner.scale_and_add(&other.inner, scalar);
    }

    pub fn transform(&mut self, matrix: JsValue) -> Point {
        if let Some(m) = parse_matrix(&matrix) {
            self.inner.transform(&m);
        }
        self.clone()
    }

    pub fn to_array(&self, out: &mut [f64]) -> Vec<f64> {
        if out.len() >= 2 {
            out[0] = self.inner.x;
            out[1] = self.inner.y;
            out.to_vec()
        } else {
            vec![self.inner.x, self.inner.y]
        }
    }

    pub fn from_array(&mut self, input: &[f64]) {
        if input.len() >= 2 {
            self.inner.set(input[0], input[1]);
        }
    }
}
