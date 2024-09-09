use crate::field::{FieldElement, Field};
use bigint::{U256,U512};
use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::FQP::{FQP,FQ2,FQ12,FIELD_MODULUS};
const CURVE_ORDER:&str="21888242871839275222246405745257275088548364400416034343698204186575808495617";

pub fn const_pow(a:U256,b:U256, exp:u64)->U256{
  let mut result =U512::from(1);
  let mut base=U512::from(a);
  let mut exp= exp;
  while exp>0{
      if exp%2==1{
          result = (result*base)%U512::from(b);
      }
      base =(base*base)%U512::from(b);
      exp=exp/2;

  }
  U256::from(result)
}

#[derive(Debug, PartialEq, Clone)]


pub struct Curve {
    field_modulus: U256,
    b: FieldElement,
    b2: FQP,
    b12: FQ12,
    g1: (FieldElement, FieldElement),
    g2: (FQ2, FQ2),
}

impl Curve {
    pub fn new() -> Self {
        let field_modulus = U256::from_dec_str(FIELD_MODULUS).expect("Invalid field modulus");
        //Curve is y**2 = x**3 + 3
        let b = FieldElement::new(U256::from(3), Field::new(field_modulus));
        //b2 = FQ2([3, 0]) / FQ2([9, 1])
        //Twisted curve over FQ**2
        let f=  FQ2::new(
          FieldElement::new(U256::from(3), Field::new(field_modulus)),
          FieldElement::new(U256::from(0), Field::new(field_modulus))
      );
      let x=  FQ2::new(
        FieldElement::new(U256::from(9), Field::new(field_modulus)),
        FieldElement::new(U256::from(1), Field::new(field_modulus))
    );



        let b2 =x.inner.div(&f.inner);
        println!("b2={:?}",b2);
        // let b2=FQ2::new(bx.coefficients[0],bx.coefficients[1]);
        //Extension curve over FQ**12; same b value as over FQ
        //b12 = FQ12([3] + [0] * 11)

        
        let mut  b12 =FQ12::one(12);
        b12.inner.coefficients[0]=FieldElement::new(U256::from(3), Field::new(field_modulus));

        let g1 = (
            FieldElement::new(U256::from(1), Field::new(field_modulus)),
            FieldElement::new(U256::from(2), Field::new(field_modulus))
        );
        let g11:&str="10857046999023057135944570762232829481370756359578518086990519993285655852781";
        let g12:&str="11559732032986387107991004021392285783925812861821192530917403151452391805634";
        let cg11= U256::from_dec_str(g11).expect("Invalid number");
        let cg12= U256::from_dec_str(g12).expect("Invalid number");
        let g21:&str="8495653923123431417604973247489272438418190587263600148770280649306958101930";
        let g22:&str="4082367875863433681332203403145435568316851327593401208105741076214120093531";
        let cg21= U256::from_dec_str(g21).expect("Invalid number");
        let cg22= U256::from_dec_str(g22).expect("Invalid number");

        let g2 = (
            FQ2::new(
                FieldElement::new(cg11, Field::new(field_modulus)),
                FieldElement::new(cg12, Field::new(field_modulus))
            ),
            FQ2::new(
                FieldElement::new(cg21, Field::new(field_modulus)),
                FieldElement::new(cg22, Field::new(field_modulus))
            )
        );

        Curve {
            field_modulus,
            b,
            b2,
            b12,
            g1,
            g2,
        }
    }

    pub fn is_inf(&self, pt: Option<(FieldElement, FieldElement)>) -> bool {
        pt.is_none()
    }

    pub fn is_on_curve(&self, pt: Option<(FieldElement, FieldElement)>, b: FieldElement) -> bool {
        match pt {
            None => true,
            Some((x, y)) => y * y - x * x * x == b
        }
    }
    
    pub fn is_on_curve2(&self, pt: Option<(FQ2, FQ2)>, b2: FQP) -> bool {
      
  
  
      match pt {
          None => true,
          
          Some((x, y)) => (y.inner.mul(&y.inner)).sub ( &(x.inner.mul(&x.inner)).mul(&x.inner)) == b2
         
      }
    }

    // Add more methods here for double, add, multiply, etc.
}
#[cfg(test)]
mod test_curve_order{
  use super::*;
  #[test]
  fn test_curve_order(){
let  curve_order=U256::from_dec_str(CURVE_ORDER).expect("Failed to parse curve order");
let field_modulus = U256::from_dec_str(FIELD_MODULUS).expect("Invalid number");
let order =12;
let result = const_pow(field_modulus, curve_order, order);
assert_eq!(result,U256::from(1));
println!("Curve order should be a factor of field_modulus**12 - 1");
  
  

}
//g1 is on curve
#[test]
fn test_is_on_curve(){
  let curve = Curve::new();
 
  let x=curve.g1.0;
  let y=curve.g1.1;
  let pt =Some((x,y));
  assert_eq!(curve.is_on_curve(pt, curve.b.clone()), true);
} 
//g2 is on curve
#[test]
fn test2_is_on_curve(){
  let curve = Curve::new();
 
  let x=curve.g2.0.clone();
  let y=curve.g2.1.clone();
  let pt =Some((x,y));
  assert_eq!((curve.is_on_curve2(pt, curve.b2.clone())), true);
} 

}

