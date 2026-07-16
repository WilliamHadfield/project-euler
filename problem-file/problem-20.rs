use num_bigint::BigUint;
use num_traits::FromPrimitive;

fn main() {
   
   let mut num: BigUint = BigUint::from(1u32);

   for i in 1..=100 {
    num *= BigUint::from_i32(i).unwrap();
   }
 
   

   let num_str = num.to_string();

  
   

 let mut container: Vec<u32> =  num_str.chars().map(|x| x.to_digit(10).unwrap() ).collect();

 let sum : u32 = container.iter().sum();

 println!("{}", sum);

}
