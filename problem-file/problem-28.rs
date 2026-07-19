
fn main() {
  let n: u128 = 500;
  let mut sum_total: u128 = 1;
  let mut g = 2;
  let mut current = 1;

  for i in 1..=n {
  sum_total += ( 4 * current + 10 * g);
  current += 4 * g;
  
  
  g += 2;
  }

  println!("{}", sum_total);

}
