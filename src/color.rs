use std::{
    io::Write,
    ops::{Add, Mul},
};

use crate::vec3::Vec3;

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Color::new(value.x, value.y, value.z)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self * rhs.r, self * rhs.b, self * rhs.b)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

pub fn write_color(mut out: impl Write, pixel_color: Color) {
    let rbyte = (255.999 * pixel_color.r) as usize;
    let gbyte = (255.999 * pixel_color.g) as usize;
    let bbyte = (255.999 * pixel_color.b) as usize;

    writeln!(out, "{rbyte} {gbyte} {bbyte}").unwrap();
}
