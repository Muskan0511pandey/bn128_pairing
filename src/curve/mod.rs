use lazy_static::lazy_static;
use num_bigint::BigUint;
use num_traits::Num;

lazy_static! {
  static ref FIELD_MODULUS: BigUint = BigUint::from_str_radix("21888242871839275222246405745257275088548364400416034343698204186575808495617", 10).unwrap();
}


