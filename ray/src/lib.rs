mod rtweekend;
use rtweekend::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Default for Vec3 {
    fn default() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3(e0, e1, e2)
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
    pub fn random(min: Option<f32>, max: Option<f32>) -> Vec3 {
        Vec3(
            random_double(min, max),
            random_double(min, max),
            random_double(min, max),
        )
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Self) -> Self {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Vec3(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        self * (1.0 / other)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Self(self.0 * other, self.1 * other, self.2 * other);
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self *= 1.0 / other;
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.0 * v.0 + u.1 * v.1 + u.2 * v.2
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3(
        u.1 * v.2 - u.2 * v.1,
        u.2 * v.0 - u.0 * v.2,
        u.0 * v.1 - u.1 * v.0,
    )
}

pub fn unit_vector(u: Vec3) -> Vec3 {
    u / u.length()
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random(Some(-1.0), Some(1.0));
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    let a = random_double(Some(0.0), Some(2.0 * PI));
    let z = random_double(Some(-1.0), Some(1.0));
    let r = (1.0 - z * z).sqrt();
    Vec3(r * a.cos(), r * a.sin(), z)
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(in_unit_sphere, normal) > 0.0 {
        //和法线在同一半球
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
