// Module for polynomial extension fields
use crate::field::{FieldElement, Field};
use std::str::FromStr;
use ethereum_types::U256;
use lazy_static::lazy_static;
use num_bigint::BigUint;
use num_traits::Num;


// static FIELD_MODULUS: Lazy<U256> = Lazy::new(|| U256::from_dec_str("21888242871839275222246405745257275088696311157297823662689037894645226208583").unwrap());
// pub fn prime_field_inv(a:u64,n:U256)->u64{
//     let mut inv=1;
//     let mut base = a;
//     let mut exp= n-2;
//     while exp>0{
//         inv =(inv*base)%n;
//         base = (base*base)%n;
//     }
//     inv
// }
lazy_static! {
    static ref FIELD_MODULUS: BigUint = BigUint::from_str_radix("21888242871839275222246405745257275088696311157297823662689037894645226208583", 10).unwrap();
}

use num_traits::ToPrimitive;

pub fn prime_field_inv(a: u64) -> u64 {
    let a_biguint = BigUint::from(a);
    let mut inv = BigUint::from(1u32);
    let mut base = a_biguint;
    let mut exp = &*FIELD_MODULUS - BigUint::from(2u32);

    while exp > BigUint::from(0u32) {
        if &exp % 2u32 == BigUint::from(1u32) {
            inv = (&inv * &base) % &*FIELD_MODULUS;
        }
        base = (&base * &base) % &*FIELD_MODULUS;
        exp >>= 1;
    }

    inv.to_u64().unwrap()
}

// utility methods for polynomial math
//polynomial is in the form a)+a1*x+a2*x^2+...+an*x^n
//and vector of coefficients [a0,a1,a2,...,an]
#[derive(Debug, Clone)]
pub struct Polynomial {
  pub coefficients: Vec<u64>,
}

impl Polynomial {
  pub fn new(coefficients: Vec<u64>) -> Polynomial {
    Polynomial { coefficients }
  }

  pub fn degree(&self) -> usize {
    self.coefficients.len() - 1
  }
  pub fn poly_rounded_div(a:Polynomial,b:Polynomial)->Polynomial{
        // let mut a = a;
        // let mut b = b;
        // let mut q:Polynomial = Polynomial::new(vec![FieldElement::zero(a.coefficients[0].1);0])
        // ;
        // let mut r = a;
        // while r.coefficients.len()>=b.coefficients.len(){
        //     let t = r.coefficients[r.coefficients.len()-1]/b.coefficients[b.coefficients.len()-1];
        //     let mut t_vec = vec![FieldElement::zero(a.coefficients[0].1.);r.coefficients.len()-b.coefficients.len()];
        //     t_vec.push(t);
        //     let t_poly = Polynomial::new(t_vec);
        //     q = q.add(&t_poly);
        //     r = r.sub(&t_poly.mul(&b));
        // }
        // Polynomial::new(q.coefficients)
        let dega=a.degree();
        let degb=b.degree();
        let mut temp = a.clone();
        let mut q = Polynomial::new(vec![0]);
        for i in dega-degb..0{
            q.coefficients[i]=q.coefficients[i]+temp.coefficients[dega]*(prime_field_inv(b.coefficients[degb]));
            for j in 0..degb+1{
                temp.coefficients[i+j]=temp.coefficients[i+j]-q.coefficients[i]*b.coefficients[j];


            }
        
        }
        q
  
  

  }
   

}

//A struct for elemensts in polynomial extension fields
#[derive(Debug, Clone)]
pub struct FQP {
    pub coefficients: Vec<u64>,
    pub modulus_coeff: Vec<u64>,
}
impl FQP{
    pub fn new(coefficients:Vec<u64>,modulus_coeff:Vec<u64>)->FQP{
        
        if (coefficients.len()!=modulus_coeff.len()){
            panic!("The coefficients and modulus coefficients must have the same length");
        }
        else {FQP{coefficients,modulus_coeff}}
    }
    pub fn degree(&self) -> usize {
        self.modulus_coeff.len() 
    }
   
    pub fn add(&self, other: &FQP) -> FQP {
        assert_eq!(self.degree(), other.degree(), "Degrees must match for addition");
        
        let mut result = vec![0u64; self.degree()];
        for i in 0..self.degree() {
         let modded = BigUint::from(self.coefficients[i]+self.coefficients[i]) % &*FIELD_MODULUS;
            result[i] = modded.to_u64().unwrap();
        }
        
        FQP::new(result, self.modulus_coeff.clone())
    }
    pub fn sub(&self, other: &FQP) -> FQP {
        assert_eq!(self.degree(), other.degree(), "Degrees must match for subtraction");
        
        let mut result = vec![0u64; self.degree()];
        for i in 0..self.degree() {
            let modded = BigUint::from(self.coefficients[i] - other.coefficients[i]) % &*FIELD_MODULUS;
            result[i] = modded.to_u64().unwrap();
        }
        
        FQP::new(result, self.modulus_coeff.clone())
    }
    pub fn mul(&self, other: &FQP) -> FQP {
        assert_eq!(self.degree(), other.degree(), "Degrees must match for multiplication");
        
        let mut result = vec![0u64; self.degree()];
        for i in 0..self.degree() {
            let modded = BigUint::from(self.coefficients[i] * other.coefficients[i]) % &*FIELD_MODULUS;
            result[i] = modded.to_u64().unwrap();
        }
        
        FQP::new(result, self.modulus_coeff.clone())
    }
    pub fn div(&self, other: &FQP) -> FQP {
        assert_eq!(self.degree(), other.degree(), "Degrees must match for division");
        
        let mut result = vec![0u64; self.degree()];
        for i in 0..self.degree() {
            let modded = BigUint::from(self.coefficients[i] * prime_field_inv(other.coefficients[i])) % &*FIELD_MODULUS;
            result[i] = modded.to_u64().unwrap();
        }
        
        FQP::new(result, self.modulus_coeff.clone())
    }
    pub fn inverse(&self) -> FQP {
        let mut result = vec![0u64; self.degree()];
        for i in 0..self.degree() {
            let modded = BigUint::from(prime_field_inv(self.coefficients[i])) % &*FIELD_MODULUS;
            result[i] = modded.to_u64().unwrap();
        }
        
        FQP::new(result, self.modulus_coeff.clone())
    }
    pub fn pow(&self, exp: u64) -> FQP {
        let mut result = vec![0u64; self.degree()];
        for i in 0..self.degree() {
            let modded = BigUint::from(self.coefficients[i].pow(exp as u32)) % &*FIELD_MODULUS;
            result[i] = modded.to_u64().unwrap();
        }
        
        FQP::new(result, self.modulus_coeff.clone())
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        for coeff in &self.coefficients {
            bytes.extend_from_slice(&coeff.to_be_bytes());
        }
        bytes
    }
    pub fn neg(&self) -> FQP {
        let mut result = vec![0u64; self.degree()];
        for i in 0..self.degree() {
            let modded = &*FIELD_MODULUS-BigUint::from(self.coefficients[i]) ;
            result[i] = modded.to_u64().unwrap();
        }
     

        FQP::new(result, self.modulus_coeff.clone())
    }
    pub fn equal(&self, other: &FQP)  {
        if
        self.coefficients == other.coefficients
        {
            println!("The two polynomials are equal");
        }
        else{
            println!("The two polynomials are not equal");
        }
    }
pub fn one(&self)->FQP{
    let mut result = vec![1u64; self.degree()];
   
    FQP::new(result, self.modulus_coeff.clone())

}
pub fn zero(&self)->FQP{
    let mut result = vec![0u64; self.degree()];
    FQP::new(result, self.modulus_coeff.clone())

}
pub fn mul_assign(&mut self, other: &FQP) {
    *self = self.mul(other);
}
pub fn add_assign(&mut self, other: &FQP) {
    *self = self.add(other);
}
pub fn sub_assign(&mut self, other: &FQP) {
    *self = self.sub(other);
}
pub fn div_assign(&mut self, other: &FQP) {
    *self = self.div(other);
}
pub fn pow_assign(&mut self, exp: u64) {
    *self = self.pow(exp);
}
    pub fn neg_assign(&mut self) {
    *self = self.neg();
}
    pub fn equal_assign(&mut self, other: &FQP) {
        self.equal(other);
    }
    pub fn inverse_assign(&mut self) {
        *self = self.inverse();
    }
}