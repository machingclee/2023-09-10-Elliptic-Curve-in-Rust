#![allow(unused)]
#![allow(non_snake_case)]

use core::panic;

use crate::modules::elliptic_curve::{EllipticCurve, Field, Point};
use crate::modules::field_utils::Futil;
use num_bigint::{BigUint, RandBigInt};
use rand::{self, Rng};
use sha256::{digest, try_digest};

struct ECDSA<'a> {
    elliptic_curve: EllipticCurve<'a>,
    generator: Point<'a>,
    order: BigUint,
}

impl<'a> ECDSA<'a> {
    pub fn generate_key_pair(&'a self) -> (BigUint, Point<'a>) {
        let priv_key = self.generate_private_key();
        let pub_key = self.generate_public_key(&priv_key);
        return (priv_key, pub_key);
    }

    pub fn generate_private_key(&self) -> BigUint {
        self.generate_random_positive_number_less_than(&self.order)
    }

    pub fn generate_random_positive_number_less_than(&self, max: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();
        rng.gen_biguint_range(&BigUint::from(1u32), &max)
    }

    pub fn generate_public_key(&'a self, priv_key: &BigUint) -> Point<'a> {
        self.elliptic_curve.scalar_mul(&self.generator, priv_key)
    }

    pub fn sign(&'a self, hash: &BigUint, priv_key: &BigUint, k_random: &BigUint) -> (BigUint, BigUint) {
        assert!(hash < &self.order, "Hash is bigger than the order of Elliptic Curve");
        assert!(
            priv_key < &self.order,
            "Private key has value bigger than the order of Elliptic Curve"
        );
        assert!(k_random < &self.order, "k_random has value bigger than the order of Elliptic Curve");
        let g = &self.generator;
        let z = hash;
        let k_pri = priv_key;
        let kg = self.elliptic_curve.scalar_mul(g, &k_random);

        match kg {
            Point::Identity => panic!("Public key should not be an identity."),
            Point::Coor(kg_x, _) => {
                let R = kg_x.value;

                let S = Futil::mul(&R, &priv_key, &self.order);
                let S = Futil::add(&z, &S, &self.order);
                let S = Futil::mul(&Futil::mul_inverse(k_random, &self.order), &S, &self.order);
                (R, S)
            }
        }
    }

    pub fn verify(&self, hash: &BigUint, pub_key: &Point, signature: &(BigUint, BigUint)) -> bool {
        assert!(hash < &self.order, "Hash is bigger than the order of the elliptic curve");
        let (R, S) = signature;
        let z = hash;
        let P = self.elliptic_curve.add(
            // &self.elliptic_curve.scalar_mul(&self.generator, (z / S)),
            &self
                .elliptic_curve
                .scalar_mul(&self.generator, &Futil::mul(&z, &Futil::mul_inverse(S, &self.order), &self.order)),
            &self
                .elliptic_curve
                .scalar_mul(pub_key, &Futil::mul(R, &Futil::mul_inverse(S, &self.order), &self.order)),
        );

        if let Point::Coor(X, Y) = &P {
            (&X.value - R).modpow(&BigUint::from(1u32), &self.order) == BigUint::from(0u32)
        } else {
            false
        }
    }

    pub fn generate_hash_less_than(&self, message: &str, max: &BigUint) -> BigUint {
        let digested = digest(message);
        let bytes = hex::decode(&digested).expect("Cannot convert to Vec<u8>");
        let one = BigUint::from(1u32);
        let hash = BigUint::from_bytes_be(&bytes).modpow(&one, &(max - &one));
        let hash = hash + one;
        hash
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sign_verify() {
        let p = BigUint::from(17u32);
        let ec = EllipticCurve {
            a: Field::new(2, &p),
            b: Field::new(2, &p),
        };
        let gp_order = BigUint::from(19u32);
        let g = Point::Coor(Field::new(5, &p), Field::new(1, &p));

        let ecdsa = ECDSA {
            elliptic_curve: ec,
            generator: g,
            order: gp_order,
        };

        let priv_key = BigUint::from(7u32);
        let pub_key = ecdsa.generate_public_key(&priv_key);

        let hash = Field::new(10, &p);
        let k_random = BigUint::from(18u32);

        let message = "Bob -> 1BTC -> Alice";
        let hash_ = ecdsa.generate_hash_less_than(message, &ecdsa.order);
        let hash = BigUint::from(hash_);
        let signature = ecdsa.sign(&hash, &priv_key, &k_random);
        let verify_result = ecdsa.verify(&hash, &pub_key, &signature);
        assert!(verify_result);
    }

    #[test]
    fn test_sign_tempered_verify() {
        let p = BigUint::from(17u32);
        let ec = EllipticCurve {
            a: Field::new(2, &p),
            b: Field::new(2, &p),
        };
        let gp_order = BigUint::from(19u32);
        let g = Point::Coor(Field::new(5, &p), Field::new(1, &p));

        let ecdsa = ECDSA {
            elliptic_curve: ec,
            generator: g,
            order: gp_order,
        };

        let priv_key = BigUint::from(7u32);
        let pub_key = ecdsa.generate_public_key(&priv_key);

        let hash = Field::new(10, &p);
        let k_random = BigUint::from(18u32);

        let message = "Bob -> 1BTC -> Alice";
        let hash_ = ecdsa.generate_hash_less_than(message, &ecdsa.order);
        let hash = BigUint::from(hash_);
        let signature = ecdsa.sign(&hash, &priv_key, &k_random);
        let (R, S) = signature;
        let R = R + BigUint::from(1u32);
        let R = Futil::power(&R, 1, &ecdsa.order);

        let tempered_signature = (R, S);
        let verify_result = ecdsa.verify(&hash, &pub_key, &tempered_signature);
        assert!(!verify_result);
    }
}
