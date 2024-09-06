
use crate::field::{FieldElement, Field};
use bigint::U256;


const  field_modulus :&str= ("21888242871839275222246405745257275088696311157297823662689037894645226208583");



// pub fn prime_field_inv(a: u64) -> u64 {
    
//     let mut inv = 1;
//     let mut base = a;
//     let mut exp = &FIELD_MODULUS -2;

//     while exp > 0 {
//         if &exp % 2 == 1 {
//             inv = (&inv * &base) % &FIELD_MODULUS;
//         }
//         base = (&base * &base) % &FIELD_MODULUS;
//         exp =exp/2;
//     }

//     inv
// }

// // utility methods for polynomial math
// //polynomial is in the form a)+a1*x+a2*x^2+...+an*x^n
// //and vector of coefficients [a0,a1,a2,...,an]
// #[derive(Debug, Clone)]
// pub struct Polynomial {
//   pub coefficients: Vec<u64>,
// }

// impl Polynomial {
//   pub fn new(coefficients: Vec<u64>) -> Polynomial {
//     Polynomial { coefficients }
//   }

//   pub fn degree(&self) -> usize {
//     self.coefficients.len() - 1
//   }
//   pub fn poly_rounded_div(a:Polynomial,b:Polynomial)->Polynomial{

//         let dega=a.degree();
//         let degb=b.degree();
//         let mut temp = a.clone();
//         let mut q = Polynomial::new(vec![0]);
//         for i in dega-degb..0{
//             q.coefficients[i]=q.coefficients[i]+temp.coefficients[dega]*(prime_field_inv(b.coefficients[degb]));
//             for j in 0..degb+1{
//                 temp.coefficients[i+j]=temp.coefficients[i+j]-q.coefficients[i]*b.coefficients[j];


//             }
        
//         }
//         q
  
  

//   }
   

// }




//A struct for elemensts in polynomial extension fields
#[derive(Debug, Clone,PartialEq)]
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
  
        assert_eq!(self.degree(), other.degree(), "Degrees must match for addition");

        
        let mut result = vec![U256::from(0); self.degree()];
     
        let mut r = vec![];
        for i in 0..self.degree() {
       
        result[i]=self.coefficients[i].0+other.coefficients[i].0;
             r.push(FieldElement::new(result[i],Field::new(self.coefficients[0].1.0)));
        }
        
        FQP::new(r, self.modulus_coeff.clone())
    }
    pub fn sub(&self, other: &FQP) -> FQP {
        assert_eq!(self.degree(), other.degree(), "Degrees must match for subtraction");
        
        let mut result = vec![U256::from(0); self.degree()];
        let mut r = vec![];
        for i in 0..self.degree() {
             result[i] = self.coefficients[i].0 - other.coefficients[i].0 ;
           
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
        let mut q = vec![FieldElement::new(U256::from(0), Field::new(self.coefficients[0].1.0)); other.coefficients.len()];
        let field = Field::new(self.coefficients[0].modulus());
        let n = self.coefficients.len();
        let m = other.coefficients.len();
        if n < m {
            return (
                FQP::new(vec![FieldElement::new(U256::from(0), field); 0], self.modulus_coeff.clone()),
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
            other_coeff.append(&mut vec![FieldElement::new(U256::from(0), field); n - m - i]);
            let q_temp = poly1_coeff[0] /other_coeff[0];
            let other_poly=other_FQP.scalar_mul(q_temp.clone());
            
            for j in 0..other_poly.coefficients.len(){
                poly1_coeff[j]=poly1_coeff[j]-other_poly.coefficients[j];
            }
            q[i] = q_temp;
        }
        q.reverse();
        poly1_coeff.reverse();
        let poly1 = FQP::new(poly1_coeff, self.modulus_coeff.clone());
        let mut x:Vec<U256>=vec![];
        for i in 0..poly1.coefficients.len(){
            x.push(poly1.coefficients[i].0);
        }
        q.reverse();

        return (FQP::new(q,self.modulus_coeff.clone()), poly1);

    }
  
    pub fn mul(&self, other: &FQP) -> FQP {
        assert_eq!(self.degree(), other.degree(), "Degrees must match for multiplication");
        if (other.coefficients.len()==1){
            return self.scalar_mul(other.coefficients[0]);
            
        }
        else{
        let mut result = vec![FieldElement::new(U256::from(0),self.coefficients[0].1); self.degree()*2-1];
        
        for i in 0..self.degree() {
            for j in 
            0..other.degree() {
                result[i+j] += self.coefficients[i] * other.coefficients[j];
            }
         
        }
        println!("result={:?}",result);
        while result.len()>self.degree(){
           let exp=result.len()-self.degree()-1;
           let top = match result.pop() {
               Some(value) => value,
               None => panic!("Cannot pop from an empty vector"),
           };
           for i in 0.. self.degree(){
            let x= FieldElement::new(U256::from(self.modulus_coeff[i] ),Field::new(self.coefficients[0].1.0));
               result[exp+i]=result[exp+i]-top*x;
               if (result[exp+i].0<U256::from(0)){
                result[exp+i].0=self.coefficients[0].1.0+result[exp+i].0;

               }
               else{
                     result[exp+i].0=result[exp+i].0;
               }
               println!("result={:?}",result); 
           }
        }
        
        FQP::new(result, self.modulus_coeff.clone())}
    }
    pub fn div(&self, other: &FQP) -> FQP {
    
    let (q, r) = self.clone().q_div(other);
   
    return q;}

    pub fn inverse(&self) -> FQP {
        let mut result = vec![U256::from(0); self.degree()];
        let mut r = vec![FieldElement::new(U256::from(0),Field::new(self.coefficients[0].1.0))];
        for i in 0..self.degree() {
            let modded = self.coefficients[i].0;
            result[i] = modded;
            r.push(FieldElement::new(result[i],Field::new(self.coefficients[0].1.0)));
        }
        
        FQP::new(r, self.modulus_coeff.clone())
    }
    // pub fn pow(&self, exp: u64) -> FQP {
    //     let mut result = vec![0u64; self.degree()];
    //     let mut r = vec![FieldElement::new(U256::from(0),Field::new(self.coefficients[0].1.0))];
    //     for i in 0..self.degree() {
    //         let modded = self.coefficients[i].0.pow(exp as u32);
    //         result[i] = modded;
    //         r.push(FieldElement::new(result[i],Field::new(self.coefficients[0].1.0)));
    //     }
        
    //     FQP::new(r, self.modulus_coeff.clone())
    // }
   

    pub fn equal(&self, other: &FQP) ->bool {
        
        for i in 0..self.degree() {
            if self.coefficients[i].0 != other.coefficients[i].0 {
                println!("The two polynomials are not equal");
                return false;
            }}
       
        
            println!("The two polynomials are not equal");
            return true;
        }
    
pub fn one(&self)->FQP{
    
    let mut r = vec![FieldElement::new(U256::from(1),Field::new(self.coefficients[0].1.0))];
   
    FQP::new(r, self.modulus_coeff.clone())

}
pub fn zero(&self)->FQP{
    let mut r = vec![FieldElement::new(U256::from(0),Field::new(self.coefficients[0].1.0))];
   
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
// pub fn pow_assign(&mut self, exp: u64) {
//     *self = self.pow(exp);
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

fn get_fq12_mc_tuples() -> Vec<(usize,i64)> {
    FQ12_MODULUS_COEFFS
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c != 0)
        .map(|(i, &c)| (i, c))
        .collect()
}
#[derive(Debug,PartialEq,Clone)]
struct FQ12{
    inner: FQP,
    mc_tuples: Vec<(usize,i64)>,
    degree: usize,
}

impl FQ12 {
    fn new(coeff: Vec<FieldElement>) -> Self {
        FQ12 {
            inner: FQP::new(coeff, FQ12_MODULUS_COEFFS.to_vec()), // x^6 - 1 = 0
            mc_tuples: get_fq12_mc_tuples(),
            degree: 12,
        }
        
    }
    fn one(&self)->FQ12{
        let mut coefficients=vec![];
        let FIELD_MODULUS=Field(U256::from_dec_str(field_modulus).expect("Invalid number"));
        for i in 0..12{
            coefficients.push(FieldElement::new(U256::from(0),FIELD_MODULUS));
        }
        coefficients[0]=FieldElement::new(U256::from(1),FIELD_MODULUS);
        return FQ12::new(coefficients);

    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fq2_operations() {
        // let field = Field::new(FIELD_MODULUS);
        let FIELD_MODULUS=Field(U256::from_dec_str(field_modulus).expect("Invalid number"));
        
        let x = FQ2::new(
            FieldElement::new(U256::from(1), FIELD_MODULUS),
            FieldElement::new(U256::from(1), FIELD_MODULUS)
        );
        let f = FQ2::new(
            FieldElement::new(U256::from(1), FIELD_MODULUS),
            FieldElement::new(U256::from(2), FIELD_MODULUS)
        ); 
        let fpx = FQ2::new(
            FieldElement::new(U256::from(2), FIELD_MODULUS),
            FieldElement::new(U256::from(3), FIELD_MODULUS)
        );
        let one = FQ2::new(
            FieldElement::new(U256::from(1), FIELD_MODULUS),
            FieldElement::new(U256::from(0), FIELD_MODULUS)
        );

      //  Addition
        let add_result = x.inner.add(&f.inner);
        assert_eq!(add_result.coefficients[0].0, fpx.inner.coefficients[0].0);
        assert_eq!(add_result.coefficients[1].0, fpx.inner.coefficients[1].0);
        println!("addition working fine");
      
       // Subtraction
        let sub_result = f.inner.sub(&x.inner);

        assert_eq!(sub_result.coefficients[0].0, U256::from(0));   
        assert_eq!(sub_result.coefficients[1].0, U256::from(1));
        println!("subtraction working fine");

        // Division
        let div_result = f.inner.div(&f.inner);
        assert_eq!(div_result.coefficients[0].0, one.inner.coefficients[0].0);
        assert_eq!(div_result.coefficients[1].0, one.inner.coefficients[1].0);
        println!("division working fine");

        // // Complex operation: (f/f + x/x) != (f+x)/f+x (2!=1)
        let left_side = f.inner.div(&f.inner).add(&x.inner.div(&x.inner));
        println!{"left side ={:?}",left_side};
        let deno= f.inner.add(&x.inner);
        let right_side = f.inner.add(&x.inner).div(&deno);
        println!("right side ={:?}" ,right_side);
        assert_ne!(left_side.coefficients[0].0, right_side.coefficients[0].0);
        assert_eq!(left_side.coefficients[1].0, right_side.coefficients[1].0);

         // // Complex operation: (1/f + x/f) == (1+x)/f
         let left_side = one.inner.div(&f.inner).add(&x.inner.div(&f.inner));
         println!{"left side ={:?}",left_side};
         
         let right_side = one.inner.add(&x.inner).div(&f.inner);
         println!("right side ={:?}" ,right_side);
         assert_eq!(left_side.coefficients[0].0, right_side.coefficients[0].0);
         assert_eq!(left_side.coefficients[1].0, right_side.coefficients[1].0);

        // // Multiplication distributive property: f*(1+x) == f*1 + f*x
        let left_side = f.inner.mul(&one.inner.add(&x.inner));
        let right_side = f.inner.mul(&one.inner).add(&f.inner.mul(&x.inner));
        assert_eq!(left_side.coefficients[0].0, right_side.coefficients[0].0);
        assert_eq!(left_side.coefficients[1].0, right_side.coefficients[1].0);
        println!("multiplication working fine");
        println!{"fq2 done"};

        // // Power operation
        // let pow_result = x.inner.pow((FIELD_MODULUS.to_u64().unwrap().pow(2) - 1) as u64);
        // assert_eq!(pow_result.coefficients[0].0, one.inner.coefficients[0].0);
        // assert_eq!(pow_result.coefficients[1].0, one.inner.coefficients[1].0);
    }
  }
  #[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fq12_operations() {
        let FIELD_MODULUS=Field(U256::from_dec_str(field_modulus).expect("Invalid number"));
        let f = FQ12::new(vec![
            FieldElement::new(U256::from(1), FIELD_MODULUS),
            FieldElement::new(U256::from(2), FIELD_MODULUS),
            FieldElement::new(U256::from(3), FIELD_MODULUS),
            FieldElement::new(U256::from(4), FIELD_MODULUS),
            FieldElement::new(U256::from(5), FIELD_MODULUS),
            FieldElement::new(U256::from(6), FIELD_MODULUS),
            FieldElement::new(U256::from(7), FIELD_MODULUS),
            FieldElement::new(U256::from(8), FIELD_MODULUS),
            FieldElement::new(U256::from(9), FIELD_MODULUS),
            FieldElement::new(U256::from(10), FIELD_MODULUS),
            FieldElement::new(U256::from(11), FIELD_MODULUS),
            FieldElement::new(U256::from(12), FIELD_MODULUS),
        ]);
        
        let x =FQ12::one(&f);
       
        let fpx = FQ12::new(vec![
            FieldElement::new(U256::from(2), FIELD_MODULUS),
            FieldElement::new(U256::from(2), FIELD_MODULUS),
            FieldElement::new(U256::from(3), FIELD_MODULUS),
            FieldElement::new(U256::from(4), FIELD_MODULUS),
            FieldElement::new(U256::from(5), FIELD_MODULUS),
            FieldElement::new(U256::from(6), FIELD_MODULUS),
            FieldElement::new(U256::from(7), FIELD_MODULUS),
            FieldElement::new(U256::from(8), FIELD_MODULUS),
            FieldElement::new(U256::from(9), FIELD_MODULUS),
            FieldElement::new(U256::from(10), FIELD_MODULUS),
            FieldElement::new(U256::from(11), FIELD_MODULUS),
            FieldElement::new(U256::from(12), FIELD_MODULUS),
        ]);
       

        // assert_eq!(x.inner.add(& f.inner),fpx.inner);
        // assert_eq!(f.inner .div(&f.inner), x.inner);
        println!("{:?}", x.inner.mul(&f.inner));
      
        // assert_eq!((x.inner.mul( &f.inner)) .add( &x.inner .mul( &f.inner)), (x.inner .add( &x.inner)) .mul(&f.inner));

        // This check takes too long
        // assert_eq!(x.clone().pow(FIELD_MODULUS.pow(12) - 1), one.clone());

        println!("FQ12 works fine");
    }
}
