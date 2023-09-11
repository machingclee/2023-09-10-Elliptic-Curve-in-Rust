#![allow(unused)]
#![allow(non_snake_case)]

use crate::modules::elliptic_curve::{EllipticCurve, Field, Point};
use num_bigint::{BigUint, RandBigInt};
use rand::{self, Rng};
use sha256::{digest, try_digest};

struct ECDSA<'a> {
    elliptic_curve: EllipticCurve<'a>,
    generator: Point<'a>,
    order: BigUint,
}

impl<'a> ECDSA<'a> {
    pub fn proj_x(&self, point: &Point<'a>) -> Field<'a> {
        match point {
            Point::Identity => panic!("project cannot be done for point at infinity"),
            Point::Coor(x, y) => {
                return x.to_owned();
            }
        }
    }

    pub fn generate_key_pair(&'a self) -> (Field<'a>, Point<'a>) {
        let priv_key = self.generate_private_key();
        let pub_key = self.generate_public_key(&priv_key);
        return (priv_key, pub_key);
    }

    pub fn generate_private_key(&self) -> Field<'a> {
        self.generate_random_positive_number_less_than(&self.order);
        todo!();
    }

    pub fn generate_random_positive_number_less_than(&self, max: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();
        rng.gen_biguint_range(&BigUint::from(1u32), &max)
    }

    pub fn generate_public_key(&'a self, priv_key: &Field<'a>) -> Point<'a> {
        self.elliptic_curve.scalar_mul(&self.generator, priv_key)
    }

    pub fn sign(&'a self, hash: &Field<'a>, priv_key: &Field<'a>, k_random: &Field<'a>) -> (Field<'a>, Field<'a>) {
        let g = &self.generator;
        let k = &k_random;
        let z = hash;
        let k_pri = priv_key;
        let kg = self.elliptic_curve.scalar_mul(g, &k);
        let R = self.proj_x(&kg);
        let S = (z.clone() + &(self.proj_x(&kg) * k_pri)) / k;

        (R, S)
    }

    pub fn verify(&self, hash: &BigUint, pub_key: Point, signature: &(BigUint, BigUint)) -> bool {
        todo!();
    }
}
