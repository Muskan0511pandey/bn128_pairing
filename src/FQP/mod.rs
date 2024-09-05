// Module for polynomial extension fields
use crate::field::{FieldElement, Field};
// use std::str::FromStr;
// use ethereum_types::U256;
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
// // }
// lazy_static! {
//     static ref FIELD_MODULUS: BigUint = BigUint::from_str_radix("21888242871839275222246405745257275088696311157297823662689037894645226208583", 10).unwrap();
// }
const FIELD_MODULUS:u64=3221225473;

use num_traits::ToPrimitive;

pub fn prime_field_inv(a: u64) -> u64 {
    
    let mut inv = 1;
    let mut base = a;
    let mut exp = &FIELD_MODULUS -2;

    while exp > 0 {
        if &exp % 2 == 1 {
            inv = (&inv * &base) % &FIELD_MODULUS;
        }
        base = (&base * &base) % &FIELD_MODULUS;
        exp >>= 1;
    }

    inv
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
    pub coefficients: Vec<FieldElement>,
   
    pub modulus_coeff: Vec<i64>,
}
impl FQP{
    pub fn new(coefficients:Vec<FieldElement>,modulus_coeff:Vec<i64>)->FQP{
        println!("Creating new FQP:");
    println!("  coefficients: {:?}", coefficients);
    println!("  modulus_coeff: {:?}", modulus_coeff);
        
        if (coefficients.len()!=modulus_coeff.len()){
            panic!("The coefficients and modulus coefficients must have the same length"); 
        }
        else {FQP{coefficients,modulus_coeff}}
    }
    pub fn degree(&self) -> usize {
        self.modulus_coeff.len() 
    }
   
    pub fn add(&self, other: &FQP) -> FQP {
    //     println!("Performing addition:");
    // println!("  self: {:?}", self);
    // println!("  other: {:?}", other);
        assert_eq!(self.degree(), other.degree(), "Degrees must match for addition");

        
        let mut result = vec![0u64; self.degree()];
        // let mut r = vec![FieldElement::new(0,Field::new(self.coefficients[0].1.0))];
        let mut r = vec![];
        for i in 0..self.degree() {
        //  let modded = BigUint::from(self.coefficients[i].0+other.coefficients[i].0) % &*FIELD_MODULUS;
        //     result[i] = modded.to_u64().unwrap();
        result[i]=self.coefficients[i].0+other.coefficients[i].0;
             r.push(FieldElement::new(result[i],Field::new(self.coefficients[0].1.0)));
        }
        
        FQP::new(r, self.modulus_coeff.clone())
    }
    pub fn sub(&self, other: &FQP) -> FQP {
        assert_eq!(self.degree(), other.degree(), "Degrees must match for subtraction");
        
        let mut result = vec![0u64; self.degree()];
        let mut r = vec![];
        for i in 0..self.degree() {
            let modded = self.coefficients[i].0 - other.coefficients[i].0 ;
            result[i] = modded;
            r.push(FieldElement::new(result[i],Field::new(self.coefficients[0].1.0)));

        }
        
        FQP::new(r, self.modulus_coeff.clone())
    }
    
    pub fn scalar_mul(&self, scalar: FieldElement) -> Self {
        let mut result = Vec::new();

        for i in 0..self.coefficients.len() {
            result.push(self.coefficients[i] * scalar);
        }
        // self.coefficients = result.clone();
        FQP::new(result, self.modulus_coeff.clone())
    }
    pub fn is_all_zeros(a:Vec<u64>) -> bool {
        for i in 0..a.len() {
            if a[i] != 0 {
                return false;
            }
        }
        true
    }
    
    pub fn q_div(self, other:&FQP) -> (Self, Self) {
        let mut q = Vec::new();
        let field = Field::new(self.coefficients[0].modulus());
        let n = self.coefficients.len();
        let m = other.coefficients.len();
        if n < m {
            return (
                FQP::new(vec![FieldElement::new(0, field); 0], self.modulus_coeff.clone()),
                self,
            );
        }
        let mut poly1_coeff = self.clone().coefficients;
        let mut poly2_coeff = other.clone().coefficients;
        poly1_coeff.reverse();
        poly2_coeff.reverse();
        for i in 0..n - m + 1 {
            let mut other_coeff = poly2_coeff.clone();
            let mut other_FQP = FQP::new(other_coeff.clone(), self.modulus_coeff.clone());
            other_coeff.append(&mut vec![FieldElement::new(0, field); n - m - i]);
            let q_temp = poly1_coeff[0] / other_coeff[0];

            let other_poly=other_FQP.scalar_mul(q_temp.clone());
            poly1_coeff = (FQP::new(poly1_coeff,self.modulus_coeff.clone())  .sub( &other_poly.clone()))
                .coefficients[1..]
                .to_vec();
            q.push(q_temp);
        }
        q.reverse();
        poly1_coeff.reverse();
        let poly1 = FQP::new(poly1_coeff, self.modulus_coeff.clone());
        let mut x:Vec<u64>=vec![];
        for i in 0..poly1.coefficients.len(){
            x.push(poly1.coefficients[i].0);
        }
        if FQP::is_all_zeros(x) {
            return (FQP::new(q, self.modulus_coeff.clone()), FQP::new(vec![FieldElement::new(0, field); 0], self.modulus_coeff.clone())
                
            );
        } else {
            return (FQP::new(q,self.modulus_coeff.clone()), poly1);
        }
    }
    pub fn mul(&self, other: &FQP) -> FQP {
        assert_eq!(self.degree(), other.degree(), "Degrees must match for multiplication");
        
        let mut result = vec![0u64; self.degree()];
        let mut r = vec![FieldElement::new(0,Field::new(self.coefficients[0].1.0))];
        for i in 0..self.degree() {
            let modded = self.coefficients[i].0 * other.coefficients[i].0;
            result[i] = modded;
            r.push(FieldElement::new(result[i],Field::new(self.coefficients[0].1.0)));
        }
        
        FQP::new(r, self.modulus_coeff.clone())
    }
    pub fn div(&self, other: &FQP) -> FQP {
    //     assert_eq!(self.degree(), other.degree(), "Degrees must match for division");
    //     println!("Performing division:");
    // println!("  self: {:?}", self);
    // println!("  other: {:?}", other);
        
    //     let mut result = vec![0u64; self.degree()];
    //     let mut r = vec![];
    //     for i in 0..self.degree() {
    //         let modded = self.coefficients[i].0 * prime_field_inv(other.coefficients[i].0);
    //         print!("modded: {:?}", modded);
    //         result[i] = modded;
    //         r.push(FieldElement::new(result[i],Field::new(self.coefficients[0].1.0)));
    //     }
        
    //     FQP::new(r, self.modulus_coeff.clone())
    let (q, r) = self.clone().q_div(other);
    if FQP::is_all_zeros(r.coefficients.iter().map(|x| x.0).collect()) {
        return q;
    } else {
        panic!("Division error");
    }}

    pub fn inverse(&self) -> FQP {
        let mut result = vec![0u64; self.degree()];
        let mut r = vec![FieldElement::new(0,Field::new(self.coefficients[0].1.0))];
        for i in 0..self.degree() {
            let modded = self.coefficients[i].0;
            result[i] = modded;
            r.push(FieldElement::new(result[i],Field::new(self.coefficients[0].1.0)));
        }
        
        FQP::new(r, self.modulus_coeff.clone())
    }
    pub fn pow(&self, exp: u64) -> FQP {
        let mut result = vec![0u64; self.degree()];
        let mut r = vec![FieldElement::new(0,Field::new(self.coefficients[0].1.0))];
        for i in 0..self.degree() {
            let modded = self.coefficients[i].0.pow(exp as u32);
            result[i] = modded;
            r.push(FieldElement::new(result[i],Field::new(self.coefficients[0].1.0)));
        }
        
        FQP::new(r, self.modulus_coeff.clone())
    }
    // pub fn to_bytes(&self) -> Vec<u8> {
    //     let mut bytes = vec![];
    //     for coeff in &self.coefficients {
    //         bytes.extend_from_slice(&coeff.to_be_bytes());
    //     }
    //     bytes
    // // }
    // pub fn neg(&self) -> FQP {
    //     let mut result = vec![0u64; self.degree()];
    //     let mut r = vec![FieldElement::new(0,Field::new(self.coefficients[0].1.0))];
    //     for i in 0..self.degree() {
    //         let modded:i64 = self.coefficients[0].1-self.coefficients[i].0 ;
    //         result[i] = modded;
    //         r.push(FieldElement::new(result[i],Field::new(self.coefficients[0].1.0)));
    //     }
     

    //     FQP::new(r, self.modulus_coeff.clone())
    // }
    pub fn equal(&self, other: &FQP)  {
        
        for i in 0..self.degree() {
            if self.coefficients[i].0 != other.coefficients[i].0 {
                println!("The two polynomials are not equal");
                return;
            }}
       
        
            println!("The two polynomials are not equal");
        }
    
pub fn one(&self)->FQP{
    
    let mut r = vec![FieldElement::new(1,Field::new(self.coefficients[0].1.0))];
   
    FQP::new(r, self.modulus_coeff.clone())

}
pub fn zero(&self)->FQP{
    let mut r = vec![FieldElement::new(0,Field::new(self.coefficients[0].1.0))];
   
    FQP::new(r, self.modulus_coeff.clone())

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
//     pub fn neg_assign(&mut self) {
//     *self = self.neg();
// }
    pub fn equal_assign(&mut self, other: &FQP) {
        self.equal(other);
    }
    pub fn inverse_assign(&mut self) {
        *self = self.inverse();
    }
}
struct FQ2 {
    inner: FQP,
    mc_tuples: Vec<(u64,u64)>,
    degree: usize,
}

impl FQ2 {
    fn new(c0:FieldElement, c1: FieldElement) -> Self {
        FQ2 {
            inner: FQP::new(vec![c0, c1], vec![1, 0]), // x^2 - 1 = 0
            mc_tuples: vec![(1,0)],
            degree: 2,
        }
    }
}
const FQ12_MODULUS_COEFFS: [i64; 12] = [82, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0];
// let mut FQ12_MC_TUPLES: Vec<(u64, u64)> = FQ12_MODULUS_COEFFS.iter().enumerate()
//     .filter(|(_, c)| **c != 0)
//     .map(|(i, c)| (i as u64, *c as u64))
//     .collect();
// const fq12_mc_tuples: Vec<(usize, i32)> = FQ12_MODULUS_COEFFS
//     .iter()
//     .enumerate()
//     .filter_map(|(i, &c)| if c != 0 { Some((i, c as i32)) } else { None })
//     .collect();
fn get_fq12_mc_tuples() -> Vec<(usize,i64)> {
    FQ12_MODULUS_COEFFS
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c != 0)
        .map(|(i, &c)| (i, c))
        .collect()
}
struct FQ12{
    inner: FQP,
    mc_tuples: Vec<(usize,i64)>,
    degree: usize,
}

impl FQ12 {
    fn new(c0:FieldElement, c1: FieldElement, c2: FieldElement, c3: FieldElement, c4: FieldElement, c5: FieldElement,c6:FieldElement,c7:FieldElement, c8: FieldElement, c9: FieldElement, c10: FieldElement, c11: FieldElement) -> Self {
        FQ12 {
            inner: FQP::new(vec![c0, c1, c2, c3, c4, c5,c6,c7,c8,c9,c10,c11],FQ12_MODULUS_COEFFS.to_vec()), // x^6 - 1 = 0
            mc_tuples: get_fq12_mc_tuples(),
            degree: 12,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fq2_operations() {
        let field = Field::new(FIELD_MODULUS);
        
        let x = FQ2::new(
            FieldElement::new(1, field.clone()),
            FieldElement::new(1, field.clone())
        );
        let f = FQ2::new(
            FieldElement::new(1, field.clone()),
            FieldElement::new(2, field.clone())
        ); 
        let fpx = FQ2::new(
            FieldElement::new(2, field.clone()),
            FieldElement::new(3, field.clone())
        );
        let one = FQ2::new(
            FieldElement::new(1, field.clone()),
            FieldElement::new(0, field.clone())
        );

      //  Addition
    //     let add_result = x.inner.add(&f.inner);
    //     assert_eq!(add_result.coefficients[0].0, fpx.inner.coefficients[0].0);
    //     assert_eq!(add_result.coefficients[1].0, fpx.inner.coefficients[1].0);
    //    // Subtraction
    //     let sub_result = f.inner.sub(&x.inner);

    //     assert_eq!(sub_result.coefficients[0].0, 0);   
    //     assert_eq!(sub_result.coefficients[1].0, 1);

        // Division
        let div_result = f.inner.div(&f.inner);
        assert_eq!(div_result.coefficients[0].0, one.inner.coefficients[0].0);
        assert_eq!(div_result.coefficients[1].0, one.inner.coefficients[1].0);

        // // Complex operation: (1/f + x/f) == (1+x)/f
        // let left_side = one.inner.div(&f.inner).add(&x.inner.div(&f.inner));
        // let right_side = one.inner.add(&x.inner).div(&f.inner);
        // assert_eq!(left_side.coefficients[0].0, right_side.coefficients[0].0);
        // assert_eq!(left_side.coefficients[1].0, right_side.coefficients[1].0);

        // // Multiplication distributive property: f*(1+x) == f*1 + f*x
        // let left_side = f.inner.mul(&one.inner.add(&x.inner));
        // let right_side = f.inner.mul(&one.inner).add(&f.inner.mul(&x.inner));
        // assert_eq!(left_side.coefficients[0].0, right_side.coefficients[0].0);
        // assert_eq!(left_side.coefficients[1].0, right_side.coefficients[1].0);

        // // Power operation
        // let pow_result = x.inner.pow((FIELD_MODULUS.to_u64().unwrap().pow(2) - 1) as u64);
        // assert_eq!(pow_result.coefficients[0].0, one.inner.coefficients[0].0);
        // assert_eq!(pow_result.coefficients[1].0, one.inner.coefficients[1].0);
    }
}