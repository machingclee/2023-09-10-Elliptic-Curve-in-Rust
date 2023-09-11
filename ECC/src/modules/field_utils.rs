use num_bigint::BigUint;

pub struct Futil {}

impl Futil {
    pub fn power(u: &BigUint, i: u32, p: &BigUint) -> BigUint {
        u.modpow(&BigUint::from(i), &p)
    }

    pub fn add(u: &BigUint, v: &BigUint, p: &BigUint) -> BigUint {
        (u + v).modpow(&BigUint::from(1u32), &p)
    }
    pub fn mul(u: &BigUint, v: &BigUint, p: &BigUint) -> BigUint {
        (u * v).modpow(&BigUint::from(1u32), &p)
    }
    pub fn add_inverse(u: &BigUint, p: &BigUint) -> BigUint {
        assert!(u < &p, "{}", format!("{} >= {} should not happen", u, &p));
        p - u
    }
    pub fn mul_inverse(u: &BigUint, p: &BigUint) -> BigUint {
        if p < &BigUint::from(2u32) {
            BigUint::from(1u32)
        } else {
            let two = BigUint::from(2u32);
            let power = Futil::add(&p, &Futil::add_inverse(&two, &p), &p);
            u.modpow(&power, &p)
        }
    }
}
