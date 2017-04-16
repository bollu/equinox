use std::vec::Vec;
use engine::math::{Vector2, Angle};

use std::fmt::{Show, Formatter, Result};

pub struct Mat3 {
    elem: Vec<f32>,
}

impl Mat3 {
    pub fn iden() -> Mat3 {
        let mut elem: Vec<f32> = Vec::new();
        elem.push_all([1., 0., 0., 0., 1., 0., 0., 0., 1.]);

        Mat3 { elem: elem }
    }

    pub fn translate(x: f32, y: f32) -> Mat3 {
        let mut elem: Vec<f32> = Vec::new();
        elem.push_all([1., 0., x, 0., 1., y, 0., 0., 1.]);

        Mat3 { elem: elem }
    }

    pub fn scale(sx: f32, sy: f32) -> Mat3 {
        let mut elem: Vec<f32> = Vec::new();
        elem.push_all([sx, 0., 0., 0., sy, 0., 0., 0., 1.]);

        Mat3 { elem: elem }
    }

    pub fn rotate(angle: &Angle) -> Mat3 {
        let sin = angle.sin();
        let cos = angle.cos();

        let mut elem: Vec<f32> = Vec::new();
        elem.push_all([cos, -sin, 0., sin, cos, 0., 0., 0., 1.]);

        Mat3 { elem: elem }

    }

    pub fn transform(&self, rhs: &Vector2) -> Vector2 {
        let x = rhs.x;
        let y = rhs.y;

        let mat = &self.elem;

        let result_x = mat.get(0) * x + mat.get(1) * y + mat.get(2) * 1.;
        let result_y = mat.get(3) * x + mat.get(4) * y + mat.get(5) * 1.;

        Vector2 {
            x: result_x,
            y: result_y,
        }
    }

    pub fn invert(&self) -> Mat3 {
        let elem = &self.elem;

        let i00 = elem.get(4) * *elem.get(8) - elem.get(5) * *elem.get(7);
        let i01 = -1. * (elem.get(3) * *elem.get(8) - elem.get(5) * *elem.get(6));
        let i02 = elem.get(3) * *elem.get(7) - elem.get(4) * *elem.get(6);

        //2nd row, 1st column
        let i10 = -1. * (elem.get(1) * *elem.get(8) - elem.get(2) * *elem.get(7));
        let i11 = elem.get(0) * *elem.get(8) - elem.get(2) * *elem.get(6);
        let i12 = -1. * (elem.get(0) * *elem.get(7) - elem.get(1) * *elem.get(6));

        //3rd row, 1st column
        let i20 = elem.get(1) * *elem.get(5) - elem.get(2) * *elem.get(4);
        let i21 = -1. * (elem.get(0) * *elem.get(5) - elem.get(2) * *elem.get(3));
        let i22 = elem.get(0) * *elem.get(4) - elem.get(1) * *elem.get(3);


        let det = elem.get(0) * i00 + elem.get(1) * i01 + elem.get(2) * i02;

        let mut inv_elem: Vec<f32> = Vec::new();
        //(adj A) ^ transpose (I made a mistake while writing out numbers >,>)
        inv_elem.push_all([i00 / det, i10 / det, i20 / det, i01 / det, i11 / det, i21 / det,
                           i02 / det, i12 / det, i22 / det]);

        print!("\ndet = {}", det);
        print!("adj = {}", ~[i00, i10, i20, i01, i11, i21, i02, i12, i22]);
        print!("inv = {}", inv_elem);
        Mat3 { elem: inv_elem }
    }
}

impl Mul<Mat3, Mat3> for Mat3 {
    fn mul(&self, rhs_mat: &Mat3) -> Mat3 {

        let lhs = &self.elem;
        let rhs = &rhs_mat.elem;

        //1st row, 1st column
        let i00 = lhs.get(0) * *rhs.get(0) + lhs.get(1) * *rhs.get(3) + lhs.get(2) * *rhs.get(6);
        //1st row, 2nd column
        let i01 = lhs.get(0) * *rhs.get(1) + lhs.get(1) * *rhs.get(4) + lhs.get(2) * *rhs.get(7);
        //1st row, 3nd column
        let i02 = lhs.get(0) * *rhs.get(2) + lhs.get(1) * *rhs.get(5) + lhs.get(2) * *rhs.get(8);

        //2nd row, 1st column
        let i10 = lhs.get(3) * *rhs.get(0) + lhs.get(4) * *rhs.get(3) + lhs.get(5) * *rhs.get(6);
        //2nd row, 2nd column
        let i11 = lhs.get(3) * *rhs.get(1) + lhs.get(4) * *rhs.get(4) + lhs.get(5) * *rhs.get(7);
        //2nd row, 3rd column
        let i12 = lhs.get(3) * *rhs.get(2) + lhs.get(4) * *rhs.get(5) + lhs.get(5) * *rhs.get(8);


        //3rd row, 1st column
        let i20 = lhs.get(6) * *rhs.get(0) + lhs.get(7) * *rhs.get(3) + lhs.get(8) * *rhs.get(6);
        //3rd row, 2nd column
        let i21 = lhs.get(6) * *rhs.get(1) + lhs.get(7) * *rhs.get(4) + lhs.get(8) * *rhs.get(7);
        //3rd row, 3rd column
        let i22 = lhs.get(6) * *rhs.get(2) + lhs.get(7) * *rhs.get(5) + lhs.get(8) * *rhs.get(8);

        let mut elem: Vec<f32> = Vec::new();
        elem.push_all([i00, i01, i02, i10, i11, i12, i20, i21, i22]);

        Mat3 { elem: elem }
    }
}



impl Show for Mat3 {
    fn fmt(&self, formatter: &mut Formatter) -> Result {

        let elem = &self.elem;

        write!(formatter.buf,
               "\n|{:.2}, {:.2}, {:.2}|\n|{:.2}, {:.2}, {:.2}|\n|{:.2}, {:.2}, {:.2}|\n",
               elem.get(0),
               elem.get(1),
               elem.get(2),
               elem.get(3),
               elem.get(4),
               elem.get(5),
               elem.get(6),
               elem.get(7),
               elem.get(8))
    }
}