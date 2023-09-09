use num_bigint::BigUint;
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Clone, Debug)]
pub enum Point<'a> {
    Coor(F_p<'a>, F_p<'a>),
    Identity,
}

pub struct EllipticCurve<'a> {
    a: F_p<'a>,
    b: F_p<'a>,
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
                let two_times_yp = yp.clone() * BigUint::from(2u32);
                let s = xp.clone() * xp;
                let s = s * BigUint::from(3u32);
                let s = s + &self.a;
                let s = s.clone() / &two_times_yp;

                let two_times_x = xp.clone() * BigUint::from(2u32);
                let new_x = s.clone() * &s;
                let new_x = new_x - &two_times_x;

                let new_y = xp.clone() - &new_x;
                let new_y = s * &new_y;
                let new_y = new_y - yp;

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
                if x1p == x2p && (&y1p.value + &y2p.value) == BigUint::from(0u32) {
                    return Point::Identity;
                }
                // s = (y2-y1)/(x2-x1)
                // x3 = s^2 - x1 - x2
                // y3 = s*(x1-x3) - y1

                let s = y2p.clone() - y1p;
                let x2_minus_x1 = x2p.clone() - x1p;
                let s = s / &x2_minus_x1;
                let s_square = s.clone() * &s;

                let x3p = s_square - x1p;
                let x3p = x3p - x2p;

                let y3p = s * &(x1p.clone() - &x3p);
                let y3p = y3p - y1p;

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
            let y2 = y.clone() * y;
            let x3 = x.clone() * x;
            let x3 = x3 * x;
            let ax = x.clone() * &self.a;
            y2 == x3 + &ax + &self.b
        } else {
            true
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct F_p<'a> {
    value: BigUint,
    p: &'a BigUint,
}
impl<'a> F_p<'a> {
    pub fn new(i: u32, p: &'a BigUint) -> Self {
        F_p {
            value: BigUint::from(i),
            p,
        }
    }
}

impl<'a> Add<&F_p<'a>> for F_p<'a> {
    type Output = F_p<'a>;

    fn add(self, rhs: &F_p) -> Self::Output {
        let value = (&self.value + &rhs.value).modpow(&BigUint::from(1u32), self.p);
        F_p { value, p: self.p }
    }
}

impl<'a> Sub<&F_p<'a>> for F_p<'a> {
    type Output = F_p<'a>;

    fn sub(self, rhs: &F_p) -> Self::Output {
        let value: BigUint;
        let a = &self.value;
        let b = &rhs.value;
        if a > b {
            value = a - b;
        } else {
            value = (self.p + a) - b;
        }
        F_p { value, p: &self.p }
    }
}

impl<'a> Mul<BigUint> for F_p<'a> {
    type Output = F_p<'a>;
    fn mul(self, rhs: BigUint) -> Self::Output {
        let a = &self.value;
        let value = (a * &rhs).modpow(&BigUint::from(1u32), &self.p);
        return F_p { value, p: self.p };
    }
}

impl<'a> Mul<&F_p<'a>> for F_p<'a> {
    type Output = F_p<'a>;

    fn mul(self, rhs: &F_p) -> Self::Output {
        let value = (&self.value * &rhs.value).modpow(&BigUint::from(1u32), self.p);
        F_p { value, p: self.p }
    }
}

impl<'a> Div<&F_p<'a>> for F_p<'a> {
    type Output = F_p<'a>;

    fn div(self, rhs: &F_p) -> Self::Output {
        let left = &self.value;
        let right = &rhs.value;
        let p_minus_2 = (self.p - BigUint::from(2u32)).modpow(&BigUint::from(1u32), self.p);

        let multiplicative_inverse_right = right.modpow(&p_minus_2, &self.p);
        let value = (left * &multiplicative_inverse_right).modpow(&BigUint::from(1u32), self.p);
        F_p { value, p: &self.p }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_ec_point_addition() {
        let p = BigUint::from(17u32);
        let ec = EllipticCurve {
            a: F_p::new(2, &p),
            b: F_p::new(2, &p),
        };

        // (6, 3) + (5, 1) = (10, 6);
        let p1 = Point::Coor(F_p::new(6, &p), F_p::new(3, &p));
        let p2 = Point::Coor(F_p::new(5, &p), F_p::new(1, &p));
        let r = Point::Coor(F_p::new(10, &p), F_p::new(6, &p));

        let res = ec.add(&p1, &p2);
        assert_eq!(r, res);
    }
    #[test]
    fn test_ec_point_add_identity() {
        let p = BigUint::from(17u32);
        let ec = EllipticCurve {
            a: F_p::new(2, &p),
            b: F_p::new(2, &p),
        };

        // (6, 3) + (5, 1) = (10, 6);
        let p1 = Point::Coor(F_p::new(6, &p), F_p::new(3, &p));
        let p2 = Point::Identity;
        let expect = Point::Coor(F_p::new(6, &p), F_p::new(3, &p));

        let result = ec.add(&p1, &p2);
        assert_eq!(expect, result);
    }

    #[test]
    fn test_scalar_mul() {
        let p = BigUint::from(17u32);
        let ec = EllipticCurve {
            a: F_p::new(2, &p),
            b: F_p::new(2, &p),
        };
        let q = Point::Coor(F_p::new(5, &p), F_p::new(1, &p));
        let k = BigUint::from(16u32);
        let result = ec.scalar_mul(&q, &k);

        let expected = Point::Coor(F_p::new(10, &p), F_p::new(11, &p));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_double() {
        let p = BigUint::from(17u32);
        let ec = EllipticCurve {
            a: F_p::new(2, &p),
            b: F_p::new(2, &p),
        };

        let p = Point::Coor(F_p::new(6, &p), F_p::new(3, &p));
        let double = ec.double(&p);
        let p_on_curve = ec.is_on_curve(&double);
        assert!(p_on_curve);
    }
}
