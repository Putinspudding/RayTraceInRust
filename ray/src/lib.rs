use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
#[derive(Debug, Eq, PartialEq)]
pub struct vec3(f32, f32, f32);

impl vec3 {
    pub fn new() -> vec3 {
        vec3(0, 0, 0)
    }
    pub fn new(e0: f32, e1: f32, e2: f32) -> vec3 {
        vec3(e0, e1, e2)
    }
    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn z(&self) -> f32 {
        self.2
    }
    pub fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
}

impl Neg for vec3 {
    type Output = Self;

    fn neg(self) -> vec3 {
        vec3(-self.0, -self.1, -self.2)
    }
}

impl Add for &vec3 {
    type Output = vec3;
    fn add(self, other: Self) -> vec3 {
        vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for &vec3 {
    type Output = vec3;

    fn sub(self, other: Self) -> vec3 {
        vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul for &vec3 {
    type Output = vec3;

    fn mul(self, other: Self) -> vec3 {
        vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f32> for &vec3 {
    type Output = vec3;

    fn mul(self, other: f32) -> vec3 {
        vec3(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Mul<&test> for f32 {
    type Output = vec3;

    fn mul(self, other: &vec3) -> vec3 {
        vec3 * self
    }
}

impl Div<f32> for &test {
    type Output = vec3;

    fn div(self, other: f32) -> vec3 {
        self * (1.0 / other)
    }
}

impl AddAssign<&vec3> for vec3 {
    fn add_assign(&mut self, other: &Self) {
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

impl MulAssign<f32> for vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Self(self.0 * other, self.1 * other, self.2 * other);
    }
}

impl DivAssign<f32> for vec3 {
    fn div_assign(&mut self, other: f32) {
        self *= 1.0 / other;
    }
}

pub fn dot(u: &vec3, v: &vec3) -> f32 {
    u.0 * v.0 + u.1 * v.1 + u.2 * v.2
}

pub fn cross(u: &vec3, v: &vec3) -> vec3 {
    vec3(
        u.1 * v.2 - u.2 * v.1,
        u.2 * v.0 - u.0 * v.2,
        u.0 * v.1 - u.1 * v.0,
    )
}

pub fn unit_vector(u: &vec3) -> vec3 {
    u / u.length()
}

pub type point3 = vec3;
pub type color = vec3;
