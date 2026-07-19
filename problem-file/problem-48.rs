use num_bigint::BigUint;
use num_traits::pow;
fn main() {

  let mut num: BigUint = BigUint::from(0u32);

  let n = 1000u32;

  for i in 1..=n {
  num += BigUint::from(i).pow(i as u32);
  }

  let container = num.to_string();

  let storage : Vec<char> = container.chars().rev().collect();

  let s: String = storage.iter().take(10).rev().collect();
  let n: u64 = s.parse().unwrap();

  println!("{}", n);


}
