use num_bigint::BigUint;

fn main() {
   
   let num: BigUint = BigUint::from(1u32) << 1000;


   let num_str = num.to_string();

  
   

 let mut container: Vec<u32> =  num_str.chars().map(|x| x.to_digit(10).unwrap() ).collect();

 let sum : u32 = container.iter().sum();

 println!("{}", sum);

}
