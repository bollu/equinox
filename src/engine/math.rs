use num::rational::Ratio;

use std::f32::{atan2, sqrt, sin, cos, tan};
use std::f32::consts::PI;
use std::fmt::{Show, Formatter, Result};

type coord = f32;
type rawAngle = f32;

#[deriving(Clone)]
pub struct vector2 {
	x: coord,
	y: coord,
}


impl vector2 {
	pub fn null() -> vector2 {
		vector2 { x: 0 as coord , y: 0 as coord } 
	}
	
	pub fn new(x: coord, y: coord) -> vector2 {
		vector2 { x: x, y: y }
	}

	pub fn normalize(&self) -> vector2 {
		let len = self.len();

		match len {
			0. => vector2::null(),
			_ => vector2 {x: self.x / len, y : self.y / len}
		}
	}

	pub fn len_squared(&self) -> f32 {
		self.x * self.x + self.y * self.y
	}

	pub fn len(&self) -> f32 {
		sqrt(self.len_squared())
	}

	pub fn polar(&self) -> Polar{
		Polar::new(Angle::rad(atan2(self.y, self.x)), self.len())
	}

}

impl Add<vector2, vector2> for vector2 {
	fn add(&self, rhs: &vector2) -> vector2 {
		vector2 { x : self.x + rhs.x,  y : self.y + rhs.y }
	}
}

impl Sub<vector2, vector2> for vector2 {
	fn sub(&self, rhs: &vector2) -> vector2 {
		vector2 { x : self.x - rhs.x,  y : self.y - rhs.y }
	}
}

impl Mul<f32, vector2> for vector2 {
	fn mul(&self, rhs: &f32) -> vector2 {
		vector2 { x : self.x * (*rhs),  y : self.y * (*rhs) }
	}
}


impl Div<f32, vector2> for vector2{
	fn div(&self, rhs: &f32) -> vector2 {
		vector2 { x : self.x / (*rhs),  y : self.y / (*rhs) }
	}
}

impl Neg<vector2> for vector2 {
	fn neg(&self) -> vector2 {
		vector2 { x: -self.x, y: -self.y }
	}
}

impl Eq for vector2{
	fn eq(&self, other: &vector2) -> bool {
		self.x == other.x && self.y == other.y
	}

	fn ne(&self, other: &vector2) -> bool {
		self.x != other.x || self.y != other.y
	}
}

impl Show for vector2 {
	fn fmt(&self, formatter: &mut Formatter) -> Result {
		let dirStr = angle_to_dir_str(&self.polar().angle);
		write!(formatter.buf, "\\{ {} | x: {:.2}, y:{:.2} \\}", dirStr, self.x, self.y)
	}
}

pub fn dot(v1: vector2, v2: vector2) -> f32 {
	v1.x * v2.x + v1.y * v2.y
}


//Angle---
#[deriving(Clone)]
pub struct Angle {
	//in radians
	theta: rawAngle,
}

impl Angle {
	
	//constructors---
	pub fn deg(deg: rawAngle) -> Angle {
		Angle { theta: Angle::clamp(deg.to_radians()) }
	}
	pub fn rad(rad: rawAngle) -> Angle{
		Angle { theta: Angle::clamp(rad) }
	}

	pub fn sin(&self) -> f32 {
		sin(self.theta)
	}

	pub fn cos(&self) -> f32 {
		cos(self.theta)
	}

	pub fn tan(&self) -> f32 {
		tan(self.theta)
	}

	fn clamp(rad: rawAngle) -> rawAngle {
		let tolerance = 2 as rawAngle;
		let limit = (PI as rawAngle) *  tolerance;

		if rad > limit || rad < -limit {
			return angle_to_principal_domain(rad);
		}
		return rad
	}
}


impl Add<Angle, Angle> for Angle {
	fn add(&self, rhs: &Angle) -> Angle {
		Angle { theta : Angle::clamp(self.theta + rhs.theta) }
	}
}

impl Sub<Angle, Angle> for Angle {
	fn sub(&self, rhs: &Angle) -> Angle {
		Angle { theta : Angle::clamp(self.theta - rhs.theta) }
	}
}

impl Mul<f32, Angle> for Angle {
	fn mul(&self, rhs: &f32) -> Angle {
		Angle { theta : Angle::clamp(self.theta * *rhs) }
	}
}


impl Div<f32, Angle> for Angle{
	fn div(&self, rhs: &f32) -> Angle {
		//clamp if divided by negative values
		Angle { theta : Angle::clamp(self.theta / *rhs) }
	}
}

impl Neg<Angle> for Angle {
	fn neg(&self) -> Angle {
		Angle { theta : Angle::clamp(-self.theta) }
	}
}


impl Show for Angle {
	fn fmt(&self, formatter: &mut Formatter) -> Result {
		
		let normalizedAngle = angle_to_principal_domain(self.theta); 
		let simplificationFactor = 6;
		let simplifiedNumerator = (normalizedAngle / PI * simplificationFactor as f32).round() as int;
		let ratio = Ratio::new(simplifiedNumerator, simplificationFactor);
		
		let n = *ratio.numer();
		let d = *ratio.denom();

		let degrees = (self.theta * 180f32 / PI);
		
		let radStr = match (n, d) {
			(0, _) => ~"0",
			(1, 1) => ~"π",
			(1, _) => format!("π/{}", d),
			(_, 1) => format!("{}π", n),
			_ => format!("{}π/{}", n, d),
		};

		let dirStr = angle_to_dir_str(self);

		write!(formatter.buf, "\\{ {} | rad: {} | {:.2}° \\}", dirStr, radStr, degrees)
		
	}
}

fn angle_to_dir_str(Angle : &Angle) -> ~str {
	let principalAngle = angle_to_principal_domain(Angle.theta);
	let degrees = (principalAngle * 180f32 / PI);

	match (degrees / 45.).floor() as int {
		0 => ~"→",
		1 => ~"↗",
		2 => ~"↑",
		3 => ~"↖",
		4 => ~"←",
		5 => ~"↙",
		6 => ~"↓",
		7 => ~"↘",
		_ => ~"⟳"
	}
} 

//polar coordinates-----------------

#[deriving(Show)]
pub struct Polar {
	angle: Angle,
	len: f32
}

impl Polar {
	pub fn new(angle: Angle, len: f32) -> Polar {
		Polar { angle: angle, len: len }
	}

	pub fn unit(angle: Angle) -> Polar {
		Polar { angle: angle, len: 1. }
	}

	pub fn unit_rad(rad: f32) -> Polar {
		Polar { angle: Angle::rad(rad), len: 1. }
	}

	pub fn unit_deg(deg: f32) -> Polar {
		Polar { angle: Angle::deg(deg), len: 1. }
	}

	//member functions-----
	pub fn cartesian(&self) -> vector2 {
		vector2::new(self.angle.cos() * self.len, self.angle.sin() * self.len)
	}

	pub fn normalize(&self) -> Polar {
		Polar { angle: self.angle, len: 1.}
	}
}

//polar by polar multiplication
impl Mul<Polar, Polar> for Polar {
	fn mul(&self, rhs: &Polar) -> Polar {
		Polar { angle: self.angle + rhs.angle, len: self.len * rhs.len }
	}
}


impl Div<Polar, Polar> for Polar{
	fn div(&self, rhs: &Polar) -> Polar {
		Polar { angle: self.angle - rhs.angle, len: self.len / rhs.len }
	}
}

impl Neg<Polar> for Polar {
	fn neg(&self) -> Polar {
		Polar { angle : -self.angle, len: self.len }
	}
}

//auxiliary helper functions--------------------------------
fn angle_to_principal_domain(rad: rawAngle) -> rawAngle{
	let twoPi = (2. * PI) as rawAngle;

	let mut clamped : rawAngle = rad % twoPi;

	if clamped < 0. {
		clamped += twoPi;
	}
	
	assert!(clamped >= 0 as rawAngle && clamped < twoPi);
	return clamped
}


//tests

#[test]
fn test_vector_cons() {
	let v1 = vector2::null();
	let v2 = vector2::new(3.0, 4.0);

	assert!(v1.x == 0. && v1.y == 0.);
	assert!(v2.x == 3. && v2.y == 4.);
}

#[test]
fn test_vector_normalize() {
	let v1 = vector2::null();
	let v2 = vector2::new(3.0, 4.0);
	let v3 = vector2::new(1.0, 0.0);

	assert!(v1.normalize() == v1);
	assert!(v3.normalize() == v3);

	assert!(v2.normalize().x == 3.0 / 5.0 && v2.normalize().y == 4.0 / 5.0)
}

#[test]
fn test_vector_len() {
	let v1 = vector2::null();
	let v2 = vector2::new(3.0, 4.0);
	let v3 = vector2::new(1.0, 0.0);
	let v4 = vector2::new(1.0, 1.0);

	assert!(v1.len() == 0. && v1.len_squared() == 0.);
	assert!(v2.len() == 5.);
	assert!(v3.len_squared() == 1. && v3.len() == 1.);
	assert!(v4.len_squared() == 2.);
}

#[test]
fn test_vector_operators() {
	let v1 = vector2::null();
	let v2 = vector2::new(3.0, 4.0);
	let v3 = vector2::new(1.0, 0.0);
	let v4 = vector2::new(1.0, 1.0);
	let v5 = vector2::new(0.0, 1.0);

	assert!(v1 + v2 == v2);
	assert!(v1 + v1 == v1);
	assert!(v4 + v4 == v4 * 2.);
	assert!(v3 + v4 == vector2::new(2.0, 1.0));

	assert!(v2 - v1 == v2);
	assert!(v2 - v2 == v1);
	assert!(v4 - v3 == vector2::new(0.0, 1.0));

	assert!(v2 != v1);
	assert!(v3 != v4 && v5 != v4 && v5 != v3);
}

#[test]
fn test_cartesian_polar() {
	let v1 = vector2::null();
	let a1 = v1.polar();

	assert!(a1.len == 0.);
	assert!(a1.cartesian() == v1);

	let a1 = Polar::new(Angle::deg(90.), 2.);

	assert!(a1.cartesian().len() == 2.);
	assert!(a1.cartesian().polar() == a1);
	assert!(a1.cartesian().y == 2. && a1.cartesian().x == 0.);

}
