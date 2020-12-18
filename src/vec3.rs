use std::{
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

pub type Point3 = Vec3;

/// Returns a random real in [0.1)
pub fn random_f64() -> f64 {
    rand::random::<f64>()
}
#[inline]
pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}

/// A simple struct for storing a size 3 vector
#[derive(Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    #[inline]
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3(e0, e1, e2)
    }

    #[inline]
    pub fn reflect(v: &Vec3, n: &Vec3) -> Self {
        let dif = 2.0 * v.dot(n);
        *v - dif * (*n)
    }

    #[inline]
    pub fn random() -> Self {
        Vec3(random_f64(), random_f64(), random_f64())
    }
    #[inline]
    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3(
            random_range(min, max),
            random_range(min, max),
            random_range(min, max),
        )
    }

    #[inline]
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let x = Vec3::random_range(-1.0, 1.0);
            if x.length_squared() < 1.0 {
                return x;
            }
        }
    }

    #[inline]
    pub fn random_in_hemishpere() -> Self {
        Vec3::random()
    }

    #[inline]
    pub fn random_unit_vector() -> Self {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        let s: f64 = 1e-8;
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = {
            let c = -(uv).dot(n);
            if c < 1.0 {
                c
            } else {
                1.0
            }
        };

        let r_out_perp: Vec3 = etai_over_etat * (*uv + cos_theta * (*n));
        let r_out_parallel: Vec3 =
            -1.0 * f64::sqrt((1.0 - r_out_perp.length_squared()).abs()) * (*n);

        r_out_perp + r_out_parallel
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.0
    }
    #[inline]
    pub fn y(&self) -> f64 {
        self.1
    }
    #[inline]
    pub fn z(&self) -> f64 {
        self.2
    }

    #[inline]
    pub fn dot(&self, ref rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    #[inline]
    pub fn cross(&self, rhs: Self) -> Self {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * self.1 - self.1 * rhs.0,
        )
    }

    #[inline]
    pub fn unit_vector(&self) -> Vec3 {
        self.clone() * (1f64 / self.length())
    }
}
impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}
impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        self * (1f64 / rhs)
    }
}
impl Default for Vec3 {
    fn default() -> Vec3 {
        Vec3(1f64, 1f64, 1f64)
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
