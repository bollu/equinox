extern crate nalgebra;

use num::rational::Ratio;

use std::f32;
use std::f32::consts::PI;
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt::{Debug, Formatter, Result};


//hack
use sfml::system::{Vector2f, Vector2i};

pub type Coord = f32;

#[derive(Clone)]
pub struct Vector2 {
    x: Coord,
    y: Coord,
}


impl Vector2 {
    pub fn null() -> Vector2 {
        Vector2 {
            x: 0 as Coord,
            y: 0 as Coord,
        }
    }

    pub fn new(x: Coord, y: Coord) -> Vector2 {
        Vector2 { x: x, y: y }
    }

    pub fn from_sfml_f(v: &Vector2f) -> Vector2 {
        Vector2 { x: v.x, y: v.y }
    }

    /*pub fn from_sfml_i(v: &Vector2i) -> Vector2 {
		Vector2 { x: v.x, y: v.y }
	}*/

    pub fn normalize(&self) -> Vector2 {
        let len = self.len();

        match len {
            0. => Vector2::null(),
            _ => {
                Vector2 {
                    x: self.x / len,
                    y: self.y / len,
                }
            }
        }
    }

    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn polar(&self) -> Polar {
        Polar::new(Angle::rad(self.y.atan2(self.x)), self.len())
    }

    pub fn to_sfml_f(&self) -> Vector2f {
        Vector2f {
            x: self.x,
            y: self.y,
        }
    }

    /*pub fn to_sfml_i(&self) -> Vector2i {
		Vector2i { self.x as isize, self.y as isize }
	}*/
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(&self, rhs: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(&self, rhs: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Vector2 {
    type Output = Vector2;

    fn mul(&self, rhs: &f32) -> Vector2 {
        Vector2 {
            x: self.x * (*rhs),
            y: self.y * (*rhs),
        }
    }
}


impl Div for Vector2 {
    type Output = Vector2;

    fn div(&self, rhs: &f32) -> Vector2 {
        Vector2 {
            x: self.x / (*rhs),
            y: self.y / (*rhs),
        }
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(&self) -> Vector2 {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Vector2) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Vector2) -> bool {
        self.x != other.x || self.y != other.y
    }
}

impl Debug for Vector2 {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let dirStr = angle_to_dir_str(&self.polar().angle);
        write!(formatter.buf,
               "{{ {} | x: {:.2}, y:{:.2} }}",
               dirStr,
               self.x,
               self.y)
    }
}

pub fn dot(v1: Vector2, v2: Vector2) -> f32 {
    v1.x * v2.x + v1.y * v2.y
}

//tests

#[test]
fn test_vector_cons() {
    let v1 = Vector2::null();
    let v2 = Vector2::new(3.0, 4.0);

    assert!(v1.x == 0. && v1.y == 0.);
    assert!(v2.x == 3. && v2.y == 4.);
}

#[test]
fn test_vector_normalize() {
    let v1 = Vector2::null();
    let v2 = Vector2::new(3.0, 4.0);
    let v3 = Vector2::new(1.0, 0.0);

    assert!(v1.normalize() == v1);
    assert!(v3.normalize() == v3);

    assert!(v2.normalize().x == 3.0 / 5.0 && v2.normalize().y == 4.0 / 5.0)
}

#[test]
fn test_vector_len() {
    let v1 = Vector2::null();
    let v2 = Vector2::new(3.0, 4.0);
    let v3 = Vector2::new(1.0, 0.0);
    let v4 = Vector2::new(1.0, 1.0);

    assert!(v1.len() == 0. && v1.len_squared() == 0.);
    assert!(v2.len() == 5.);
    assert!(v3.len_squared() == 1. && v3.len() == 1.);
    assert!(v4.len_squared() == 2.);
}

#[test]
fn test_vector_operators() {
    let v1 = Vector2::null();
    let v2 = Vector2::new(3.0, 4.0);
    let v3 = Vector2::new(1.0, 0.0);
    let v4 = Vector2::new(1.0, 1.0);
    let v5 = Vector2::new(0.0, 1.0);

    assert!(v1 + v2 == v2);
    assert!(v1 + v1 == v1);
    assert!(v4 + v4 == v4 * 2.);
    assert!(v3 + v4 == Vector2::new(2.0, 1.0));

    assert!(v2 - v1 == v2);
    assert!(v2 - v2 == v1);
    assert!(v4 - v3 == Vector2::new(0.0, 1.0));

    assert!(v2 != v1);
    assert!(v3 != v4 && v5 != v4 && v5 != v3);
}

#[test]
fn test_cartesian_polar() {
    let v1 = Vector2::null();
    let a1 = v1.polar();

    assert!(a1.len == 0.);
    assert!(a1.cartesian() == v1);

    let a1 = Polar::new(Angle::deg(90.), 2.);

    assert!(a1.cartesian().len() == 2.);
    assert!(a1.cartesian().polar() == a1);
    assert!(a1.cartesian().y == 2. && a1.cartesian().x == 0.);

}
