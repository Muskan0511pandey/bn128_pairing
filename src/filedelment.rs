
// use crate::field::{Field, FieldElement};










  
 
 

//     #[test]
//     fn test_field_div() {
//         let field = Field::new(7);
//         let a = FieldElement::new(1, field);
//         let b = FieldElement::new(2, field);
//         let c = a / b;
//         assert_eq!(c.0, 4);
//     }

//     #[test]
//     fn test_field_inverse() {
//         let field = Field::new(7);
//         let a = FieldElement::new(2, field);
//         let b = a.inverse();
//         assert_eq!(b.0, 4);
//     }

//     #[test]
//     fn test_field_pow() {
//         let field = Field::new(7);
//         let a = FieldElement::new(2, field);
//         let b = a.pow(3);
//         assert_eq!(b.0, 1);
//     }

//     #[test]
//     #[should_panic]
//     fn test_diff_field() {
//         let field1 = Field::new(7);
//         let field2 = Field::new(8);
//         let a = FieldElement::new(1, field1);
//         let b = FieldElement::new(2, field2);
//         let _ = a + b;
//     }

//     #[test]
//     fn test_negative_number() {
//         let field = Field::new(7);
//         let a = FieldElement::new(2, field);
//         assert_eq!((-a).0, 5);
//     }
