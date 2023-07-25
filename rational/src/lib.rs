pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rational {
    p: i32,
    q: i32,
}

impl Rational {
    pub fn new(p: i32, q: i32) -> Self {
        Rational { p, q }
    }
    pub fn value(&self) -> f32 {
        self.p as f32 * (self.q as f32).recip()
    }
    pub fn simplify(&mut self) {
        (2..=((self.p.abs().min(self.q.abs()) as f32) as i32))
            .rev()
            .for_each(|i| {
                if self.p % i == 0 && self.q % i == 0 {
                    self.p /= i;
                    self.q /= i;
                }
            })
    }
}

use std::ops::{Add, Mul, Sub};
impl Add for Rational {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Rational::new(self.p * rhs.q + self.q * rhs.p, self.q * rhs.q);
        result.simplify();
        result
    }
}

impl Sub for Rational {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Rational::new(self.p * rhs.q - self.q * rhs.p, self.q * rhs.q);
        result.simplify();
        result
    }
}

impl Mul for Rational {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Rational::new(self.p * rhs.p, self.q * rhs.q);
        result.simplify();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplify_test() {
        let mut q = Rational::new(9, 18);
        assert_eq!(q.value(), 0.5f32);
        q.simplify();
        assert_eq!(q, Rational::new(1, 2));
    }
}
