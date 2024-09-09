use std::{ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}, vec};

use bigint::{U256,U512};
// Define a field
// Define a field element
// Define arithmetic operations on field elements
//field_modulus is the prime number that defines the field

const FIELD_MODULUS :&str= "21888242871839275222246405745257275088696311157297823662689037894645226208583";

#[derive(Debug, Clone, Copy)]  
pub struct Field(pub U256);
impl Field{
    pub fn new(x:U256)->Field{
        Field(x)
    }
}
impl PartialEq for Field{
    fn eq(&self, other:&Field)-> bool{
        self.0==other.0
    }
}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct FieldElement(pub U256, pub Field);
impl FieldElement{
    pub fn new (x:U256,field:Field)->FieldElement{
        FieldElement(x%field.0,field)
    }

 pub fn zero(field:Field)-> FieldElement{
    FieldElement(U256::from(0),field)}
    pub fn one (field:Field)->FieldElement{
        FieldElement(U256::from(1),field)
    } 
    pub fn modulus (&self)->U256{
        self.1.0
    }
    pub fn inverse(&self)->FieldElement{
        let mut inv=U512::from(1);
        let mut base = U512::from(self.0);
        let mut exp= U512::from(self.1.0).sub(U512::from(2));
        println!("exp={:?}",exp);
        while exp>U512::from(0){
            if exp % U512::from(2)==U512::from(1){
                println!("self.1.0={:?}",self.1.0); 
                inv =(inv*base)%U512::from(self.1.0);
                println!("inv={:?}",inv);

            }
            base = (base*base)%U512::from(self.1.0);
            println!("base={:?}",base);
            exp=exp/U512::from(2);
            println!("exp ={:?}",exp);
        }
        FieldElement(U256::from(inv),self.1)
    }
   
    pub fn pow(&self, exp:u64)->FieldElement{
        let mut result =U256::from(1);
        let mut base=self.0;
        let mut exp= exp;
        while exp>0{
            if exp%2==1{
                result = (result*base)%self.1.0;
            }
            base =(base*base)%self.1.0;
            exp=exp/2;

        }
        FieldElement(result,self.1)
    }
    
    
pub fn eq(&self, other:&FieldElement)->bool{
    self.0==other.0 && self.1==other.1
}
}
    impl Add for FieldElement{
        type Output =FieldElement;
        fn add(self, other:FieldElement)->FieldElement{
            if (self.1)!=other.1{
                panic!("Fields must be same");
            }
            FieldElement((self.0+other.0)%self.1.0,self.1)
        }

    }
   impl AddAssign for FieldElement{
       fn add_assign(&mut self, other:FieldElement){
           if self.1!=other.1{
               panic!("Fields must be same");
           }
           self.0=(self.0+other.0)%self.1.0;
       }
   }

    impl Sub for FieldElement{
         type Output =FieldElement;
         fn sub(self, other:FieldElement)->FieldElement{
              if self.1!=other.1{
                panic!("Fields must be same");
              }
              if self.0<other.0{
                  FieldElement((self.0+ self.1.0-other.0)%self.1.0,self.1)} else{
                      FieldElement((self.0-other.0)%self.1.0,self.1)
                  } } }
                  impl SubAssign for FieldElement{
                      fn sub_assign(&mut self, other:FieldElement){
                          if self.1!=other.1{
                              panic!("Fields must be same");
                          }
                          if self.0<other.0{
                              self.0=(self.0+self.1.0-other.0)%self.1.0;
                          }else{
                              self.0=(self.0-other.0)%self.1.0;
                          }
                      }
                  }
                  impl Mul for FieldElement{
                      type Output =FieldElement;
                      fn mul(self, other:FieldElement)->FieldElement{
                          if self.1!=other.1{
                              panic!("Fields must be same");
                          }
                          let x=U512::from(self.0*other.0);
                          let y=x%U512::from(self.1.0);
                          FieldElement(U256::from(y),self.1)
                      }
                  }
                  impl MulAssign for FieldElement{
                      fn mul_assign(&mut self, other:FieldElement){
                          if self.1!=other.1{
                              panic!("Fields must be same");
                          }
                          self.0=(self.0*other.0)%self.1.0;
                      }
                  }
                  impl Div for FieldElement{
                      type Output =FieldElement;
                      fn div(self, other:FieldElement)->FieldElement{
                          if self.1!=other.1{
                              panic!("Fields must be same");
                          }
                          FieldElement((self.0*other.inverse().0)%self.1.0,self.1)
                      }
                  }
impl DivAssign for FieldElement{
    fn div_assign(&mut self, other:FieldElement){
        if self.1!=other.1{
            panic!("Fields must be same");
        }
        self.0=(self.0*other.inverse().0)%self.1.0;
    }
}
impl Neg for FieldElement{
    type Output =FieldElement;
    fn neg(self)->FieldElement{
        FieldElement((self.1.0-self.0)%self.1.0,self.1)
    }
}
#[cfg(test)]
mod test_field_operations {
    use super::*;

    #[test]
    fn test_field_add() {
        // let field = Field::new(U256::from(7));
        let field=Field(U256::from_dec_str(FIELD_MODULUS).expect("Invalid number"));
        let a = FieldElement::new(U256::from(1), field);
        let b = FieldElement::new(U256::from(2), field);
        let c = a + b;
        assert_eq!(c.0, U256::from(3));
    }
    #[test]
    fn test_field_sub() {
        // let field = Field::new(U256::from(7));
        let field=Field(U256::from_dec_str(FIELD_MODULUS).expect("Inval id number"));
        let a = FieldElement::new(U256::from(1), field);
        let b = FieldElement::new(U256::from(2), field);
        let c = a - b;

        // assert_eq!(c.0, U256::from());
        println!{"c.0: {}",c.0};//c.0: 21888242871839275222246405745257275088696311157297823662689037894645226208582
    }

    #[test]
     fn test_field_mul() {
        // let field = Field::new(U256::from(7));
        let field=Field(U256::from_dec_str(FIELD_MODULUS).expect("Invalid number"));
        let a = FieldElement::new(U256::from(1), field);
        let b = FieldElement::new(U256::from(2), field);
        let c = a * b;
        assert_eq!(c.0, U256::from(2));
    }

    #[test]
    fn test_field_div() {
        // let field = Field::new(U256::from(7));
        let field=Field(U256::from_dec_str(FIELD_MODULUS).expect("Invalid number"));

        let a = FieldElement::new(U256::from(1), field);
        let b = FieldElement::new(U256::from(2), field);
        let c = a / b;
        // assert_eq!(c.0, U256::from(4));
        print!("c.0: {}",c.0);
    }

    #[test]
    fn test_field_inverse() {
        // let field = Field::new(U256::from(7));
        let field=Field(U256::from_dec_str(FIELD_MODULUS).expect("Invalid number"));
        let a = FieldElement::new(U256::from(7 ), field);
        let b = a.inverse();
        // assert_eq!(b.0, U256::from(4));
        println!("b.0: {}",b.0);
    }

    #[test]
    fn test_field_pow() {
        // let field = Field::new(U256::from(7));
        let field=Field(U256::from_dec_str(FIELD_MODULUS).expect("Invalid number"));
        let a = FieldElement::new(U256::from(2), field);
        let b = a.pow(3);
        assert_eq!(b.0, U256::from(8));
        println!("b.0: {}",b.0);
    }

    #[test]
    #[should_panic]
    fn test_diff_field() {
        // let field1 = Field::new(U256::from(7));
        let field1=Field(U256::from_dec_str(FIELD_MODULUS).expect("Invalid number"));

        let field2 = Field::new(U256::from(8));
        let a = FieldElement::new(U256::from(1), field1);
        let b = FieldElement::new(U256::from(2), field2);
        let _ = a + b;
    }

    #[test]
    fn test_negative_number() {
        // let field = Field::new(U256::from(7));
        let field=Field(U256::from_dec_str(FIELD_MODULUS).expect("Invalid number"));
        let a = FieldElement::new(U256::from(2), field);
        // assert_eq!((-a).0, U256::from(5));
        println!("-a ={:?}", (-a).0)
    }

}
    