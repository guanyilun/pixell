use ndarray::Array1;
use std::ops::{Mul, MulAssign};

#[derive(Debug)]
struct Quat {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

impl Quat {
    fn conj(&self) -> Self {
        Quat {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[derive(Debug)]
struct QuatVec {
    w: Array1<f64>,
    x: Array1<f64>,
    y: Array1<f64>,
    z: Array1<f64>,
}

impl QuatVec {
    fn conj(&self) -> Self {
        Self {
            w: self.w.clone(),
            x: -&self.x,
            y: -&self.y,
            z: -&self.z,
        }
    }
}

// implement &Qant * &Quat
impl Mul<&Quat> for &Quat {
    type Output = Quat;

    fn mul(self, q2: &Quat) -> Quat {
        Quat {
            w: self.w * q2.w - self.x * q2.x - self.y * q2.y - self.z * q2.z,
            x: self.w * q2.x + self.x * q2.w + self.y * q2.z - self.z * q2.y,
            y: self.w * q2.y - self.x * q2.z + self.y * q2.w + self.z * q2.x,
            z: self.w * q2.z + self.x * q2.y - self.y * q2.x + self.z * q2.w,
        }
    }
}

// implement &Quat * &QuatVec
impl Mul<&QuatVec> for &Quat {
    type Output = QuatVec;

    fn mul(self, q2: &QuatVec) -> QuatVec {
        QuatVec {  // &needed on q2 because q2.w are Array1<f64>
            w: self.w * &q2.w - self.x * &q2.x - self.y * &q2.y - self.z * &q2.z,
            x: self.w * &q2.x + self.x * &q2.w + self.y * &q2.z - self.z * &q2.y,
            y: self.w * &q2.y - self.x * &q2.z + self.y * &q2.w + self.z * &q2.x,
            z: self.w * &q2.z + self.x * &q2.y - self.y * &q2.x + self.z * &q2.w,
        }
    }
}

// implement &QuatVec * &QuatVec
impl Mul<&QuatVec> for &QuatVec {
    type Output = QuatVec;

    fn mul(self, q2: &QuatVec) -> QuatVec {
        QuatVec {
            w: &self.w * &q2.w - &self.x * &q2.x - &self.y * &q2.y - &self.z * &q2.z,
            x: &self.w * &q2.x + &self.x * &q2.w + &self.y * &q2.z - &self.z * &q2.y,
            y: &self.w * &q2.y - &self.x * &q2.z + &self.y * &q2.w + &self.z * &q2.x,
            z: &self.w * &q2.z + &self.x * &q2.y - &self.y * &q2.x + &self.z * &q2.w,
        }
    }
}

// implement &QuatVec * &Quat
impl Mul<&Quat> for &QuatVec {
    type Output = QuatVec;

    fn mul(self, q2: &Quat) -> QuatVec {
        QuatVec {
            w: &self.w * q2.w - &self.x * q2.x - &self.y * q2.y - &self.z * q2.z,
            x: &self.w * q2.x + &self.x * q2.w + &self.y * q2.z - &self.z * q2.y,
            y: &self.w * q2.y - &self.x * q2.z + &self.y * q2.w + &self.z * q2.x,
            z: &self.w * q2.z + &self.x * q2.y - &self.y * q2.x + &self.z * q2.w,
        }
    }
}

// implement Quat *= &Qant
impl MulAssign<&Quat> for Quat {
    fn mul_assign(&mut self, other: &Quat) {
        self.w = self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z;
        self.x = self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y;
        self.y = self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x;
        self.z = self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w; 
    }
}

// implement QuatVec *= &Qant
impl MulAssign<&Quat> for QuatVec {
    fn mul_assign(&mut self, other: &Quat) {
        self.w = &self.w * other.w - &self.x * other.x - &self.y * other.y - &self.z * other.z;
        self.x = &self.w * other.x + &self.x * other.w + &self.y * other.z - &self.z * other.y;
        self.y = &self.w * other.y - &self.x * other.z + &self.y * other.w + &self.z * other.x;
        self.z = &self.w * other.z + &self.x * other.y - &self.y * other.x + &self.z * other.w; 
    }
}

// implement QuatVec *= &QuatVec
impl MulAssign<&QuatVec> for QuatVec {
    fn mul_assign(&mut self, other: &QuatVec) {
        self.w = &self.w * &other.w - &self.x * &other.x - &self.y * &other.y - &self.z * &other.z;
        self.x = &self.w * &other.x + &self.x * &other.w + &self.y * &other.z - &self.z * &other.y;
        self.y = &self.w * &other.y - &self.x * &other.z + &self.y * &other.w + &self.z * &other.x;
        self.z = &self.w * &other.z + &self.x * &other.y - &self.y * &other.x + &self.z * &other.w; 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quat() {
      let mut q1 = Quat{w: 1.0, x: 2.0, y: 3.0, z: 4.0};
      let q2 = Quat{w: 5.0, x: 6.0, y: 7.0, z: 8.0};

      println!("{:?}", q1);
      println!("{:?}", q1.conj());
      println!("{:?}", q2.conj());
      println!("{:?}", &q1 * &q2);
      q1 *= &q2;
      println!("{:?}", q1);

      let mut qv1 = QuatVec {
          w: ndarray::array![1.0, 2.0, 3.0],
          x: ndarray::array![4.0, 5.0, 6.0],
          y: ndarray::array![7.0, 8.0, 9.0],
          z: ndarray::array![10.0, 11.0, 12.0], 
      };
      println!("{:?}", qv1.conj());
      println!("{:?}", &qv1 * &q1);
      qv1 *= &q1;
      println!("{:?}", &qv1);
    }
}