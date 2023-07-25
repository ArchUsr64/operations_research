const EPSILON: f32 = 1e-4;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Clone, Copy, Debug)]
pub struct Rational {
    p: i32,
    q: i32,
}

impl Rational {
    pub fn new(p: i32, q: i32) -> Self {
        assert!(q != 0, "Denominator can not be 0");
        Rational { p, q }
    }
    pub fn value(&self) -> f32 {
        self.p as f32 * (self.q as f32).recip()
    }
    pub fn from_integer(value: i32) {
        Rational { p: value, q: 1 };
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

use std::cmp::{Ordering, PartialEq, PartialOrd};
impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        (self.value() - other.value()).abs() < EPSILON
    }
    fn ne(&self, other: &Self) -> bool {
        (self.value() - other.value()).abs() > EPSILON
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self == other {
            Ordering::Equal
        } else if self.value() > other.value() {
            Ordering::Greater
        } else {
            Ordering::Less
        })
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

    #[test]
    fn operations() {
        let q1 = Rational::new(1, 6);
        let q2 = Rational::new(1, 2);
        assert_eq!(q1 + q2, Rational::new(2, 3));
        assert_eq!(q1 - q2, Rational::new(-1, 3));
        assert_eq!(q1 * q2, Rational::new(1, 12));
    }

    #[test]
    fn comparisons() {
        let q1 = Rational::new(1, 6);
        let q2 = Rational::new(1, 2);
        assert!(q2 > q1);
        assert!(q1 < q2);
        let q1 = Rational::new(1, 6);
        let q2 = Rational::new(9, 54);
        assert!(q2 == q1);
    }
}
