#![allow(unused)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use num_bigint::BigUint;

use super::{
    ecdsa::ECDSA,
    elliptic_curve::{EllipticCurve, Field, Point},
};

pub struct CurveConfig {
    pub a: BigUint,
    pub b: BigUint,
    pub p: BigUint,
    pub order: BigUint,
    pub generator: (BigUint, BigUint),
}

pub struct Curve {}

impl Curve {
    pub fn get_Secp256k1_config() -> CurveConfig {
        let p = BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16).expect("Parsing fail for p");
        let a = BigUint::parse_bytes(b"0000000000000000000000000000000000000000000000000000000000000000", 16).expect("Parsing fail for a");
        let b = BigUint::parse_bytes(b"0000000000000000000000000000000000000000000000000000000000000007", 16).expect("Parsing fail for b");
        let n = BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 16).expect("Parsing fail for n");
        let x = BigUint::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).expect("Parsing fail for x");
        let y = BigUint::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).expect("Parsing fail for y");
        CurveConfig {
            p,
            a,
            b,
            order: n,
            generator: (x, y),
        }
    }

    pub fn get_elliptic_cuve<'a>(p: &'a BigUint, a: &BigUint, b: &BigUint) -> EllipticCurve<'a> {
        EllipticCurve {
            a: Field { value: a.clone(), p: &p },
            b: Field { value: b.clone(), p: &p },
        }
    }

    pub fn get_ecdsa<'a>(ec: &EllipticCurve<'a>, generator: &Point<'a>, order: &BigUint) -> ECDSA<'a> {
        ECDSA {
            elliptic_curve: ec.clone(),
            generator: generator.clone(),
            order: order.clone(),
        }
    }
}
