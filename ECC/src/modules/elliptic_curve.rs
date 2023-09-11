#![allow(unused)]
#![allow(non_camel_case_types)]

// #1[]

use num_bigint::BigUint;
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Clone, Debug)]
pub enum Point<'a> {
    Coor(Field<'a>, Field<'a>),
    Identity,
}
#[derive(PartialEq, Clone, Debug)]
pub struct EllipticCurve<'a> {
    pub a: Field<'a>,
    pub b: Field<'a>,
}

impl<'a> EllipticCurve<'a> {
    pub fn double(&self, h: &Point<'a>) -> Point {
        let h_on_curve = self.is_on_curve(h);
        assert!(h_on_curve, "point h is not on the curve");
        // s = (3*x^2 + a)/(2*y)
        // x_ = s^2 - 2*x
        // y_ = s*(x - x_) - y
        match h {
            Point::Identity => Point::Identity,
            Point::Coor(xp, yp) => {
                if yp.value == BigUint::from(0u32) {
                    return Point::Identity;
                }

                let s = &(&(&(xp * xp) * BigUint::from(3u32)) + &self.a) / &(yp * BigUint::from(2u32));

                let new_x = &(&s * &s) - &(xp * BigUint::from(2u32));
                let new_y = &(&s * &(xp - &new_x)) - yp;

                Point::Coor(new_x, new_y)
            }
        }
    }

    pub fn add(&self, h: &Point<'a>, k: &Point<'a>) -> Point {
        let h_on_curve = self.is_on_curve(h);
        let k_on_curve = self.is_on_curve(k);
        assert!(*h != *k, "two points should not be the same");
        assert!(h_on_curve, "point h is not on the curve");
        assert!(k_on_curve, "point k is not on the curve");
        match (h, k) {
            (Point::Identity, _) => k.to_owned(),
            (_, Point::Identity) => h.to_owned(),
            (Point::Coor(x1p, y1p), Point::Coor(x2p, y2p)) => {
                if x1p == x2p && (y1p + y2p).value == BigUint::from(0u32) {
                    return Point::Identity;
                }
                // s = (y2-y1)/(x2-x1)
                // x3 = s^2 - x1 - x2
                // y3 = s*(x1-x3) - y1

                let s = &(y2p - y1p) / &(x2p - x1p);
                let s_square = &s * &s;

                let x3p = &s_square - x1p;
                let x3p = &x3p - x2p;

                let y3p = &s * &(x1p - &x3p);
                let y3p = &y3p - y1p;

                Point::Coor(x3p, y3p)
            }
        }
    }

    pub fn scalar_mul(&'a self, q: &Point<'a>, k: &BigUint) -> Point<'a> {
        let mut t = q.clone();
        for i in (0..(k.bits() - 1)).rev() {
            t = self.double(&t);
            if k.bit(i) {
                t = self.add(&t, q);
            }
        }
        t
    }

    fn is_on_curve(&self, point: &Point) -> bool {
        if let Point::Coor(x, y) = point {
            let y2 = y * y;
            let x3 = &(x * x) * x;
            let ax = x * &self.a;
            y2 == &(&x3 + &ax) + &self.b
        } else {
            true
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Field<'a> {
    pub value: BigUint,
    pub p: &'a BigUint,
}
impl<'a> Field<'a> {
    pub fn new(i: u32, p: &'a BigUint) -> Self {
        Field {
            value: BigUint::from(i).modpow(&BigUint::from(1u32), &p),
            p,
        }
    }
}

impl<'a> Add<&Field<'a>> for &Field<'a> {
    type Output = Field<'a>;

    fn add(self, rhs: &Field) -> Self::Output {
        let value = (&self.value + &rhs.value).modpow(&BigUint::from(1u32), self.p);
        Field { value, p: self.p }
    }
}

impl<'a> Sub<&Field<'a>> for &Field<'a> {
    type Output = Field<'a>;

    fn sub(self, rhs: &Field) -> Self::Output {
        let value: BigUint;
        let a = &self.value;
        let b = &rhs.value;
        if a > b {
            value = a - b;
        } else {
            value = (self.p + a) - b;
        }
        Field {
            value: value.modpow(&BigUint::from(1u32), self.p),
            p: &self.p,
        }
    }
}

impl<'a> Mul<BigUint> for &Field<'a> {
    type Output = Field<'a>;
    fn mul(self, rhs: BigUint) -> Self::Output {
        let a = &self.value;
        let value = (a * &rhs).modpow(&BigUint::from(1u32), &self.p);
        Field { value, p: self.p }
    }
}

impl<'a> Mul<&Field<'a>> for &Field<'a> {
    type Output = Field<'a>;

    fn mul(self, rhs: &Field) -> Self::Output {
        let value = (&self.value * &rhs.value).modpow(&BigUint::from(1u32), self.p);
        Field { value, p: self.p }
    }
}

impl<'a> Div<&Field<'a>> for &Field<'a> {
    type Output = Field<'a>;

    fn div(self, rhs: &Field) -> Self::Output {
        let left = &self.value;
        let right = &rhs.value;
        let p_minus_2 = (self.p - BigUint::from(2u32)).modpow(&BigUint::from(1u32), self.p);

        let multiplicative_inverse_right = right.modpow(&p_minus_2, &self.p);
        let value = (left * &multiplicative_inverse_right).modpow(&BigUint::from(1u32), self.p);
        Field { value, p: &self.p }
    }
}

#[cfg(test)]
mod test {
    use crate::modules::curves::{Curve, CurveConfig};

    use super::*;
    #[test]
    fn test_ec_point_addition() {
        let p = BigUint::from(17u32);
        let ec = EllipticCurve {
            a: Field::new(2, &p),
            b: Field::new(2, &p),
        };

        // (6, 3) + (5, 1) = (10, 6);
        let p1 = Point::Coor(Field::new(6, &p), Field::new(3, &p));
        let p2 = Point::Coor(Field::new(5, &p), Field::new(1, &p));
        let r = Point::Coor(Field::new(10, &p), Field::new(6, &p));

        let res = ec.add(&p1, &p2);
        assert_eq!(r, res);
    }
    #[test]
    fn test_ec_point_add_identity() {
        let p = BigUint::from(17u32);
        let ec = EllipticCurve {
            a: Field::new(2, &p),
            b: Field::new(2, &p),
        };

        // (6, 3) + (5, 1) = (10, 6);
        let p1 = Point::Coor(Field::new(6, &p), Field::new(3, &p));
        let p2 = Point::Identity;
        let expect = Point::Coor(Field::new(6, &p), Field::new(3, &p));

        let result = ec.add(&p1, &p2);
        assert_eq!(expect, result);
    }

    #[test]
    fn test_scalar_mul() {
        let p = BigUint::from(17u32);
        let ec = EllipticCurve {
            a: Field::new(2, &p),
            b: Field::new(2, &p),
        };
        let q = Point::Coor(Field::new(5, &p), Field::new(1, &p));
        let k = BigUint::from(16u32);
        let result = ec.scalar_mul(&q, &k);

        let expected = Point::Coor(Field::new(10, &p), Field::new(11, &p));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_double() {
        let p = BigUint::from(17u32);
        let ec = EllipticCurve {
            a: Field::new(2, &p),
            b: Field::new(2, &p),
        };

        let p = Point::Coor(Field::new(6, &p), Field::new(3, &p));
        let double = ec.double(&p);
        let p_on_curve = ec.is_on_curve(&double);
        assert!(p_on_curve);
    }

    fn test_y_equal_zero() {
        let p = BigUint::from(17u32);
        let ec = EllipticCurve {
            a: Field::new(2, &p),
            b: Field::new(2, &p),
        };

        let p = Point::Coor(Field::new(6, &p), Field::new(3, &p));
        let double = ec.double(&p);
        let p_on_curve = ec.is_on_curve(&double);
        assert!(p_on_curve);
    }

    #[test]
    fn test_secp256k1() {
        let p = BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16).expect("Parsing fail for p");

        let a = BigUint::parse_bytes(b"0000000000000000000000000000000000000000000000000000000000000000", 16).expect("Parsing fail for a");

        let b = BigUint::parse_bytes(b"0000000000000000000000000000000000000000000000000000000000000007", 16).expect("Parsing fail for b");

        let n = BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 16).expect("Parsing fail for n");

        let x = BigUint::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).expect("Parsing fail for x");

        let y = BigUint::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).expect("Parsing fail for y");

        let generator = Point::Coor(Field { value: x, p: &p }, Field { value: y, p: &p });

        let ec = EllipticCurve {
            a: Field { value: a, p: &p },
            b: Field { value: b, p: &p },
        };

        let result = ec.scalar_mul(&generator, &n);

        assert_eq!(Point::Identity, result);
    }
}
