fn main() {
let n = 2_000_000;

println!("{}", sieve(n));

}

fn sieve(n : usize) -> u128 {
  let mut sieve = vec![1 as u128; n+1];
  let mut counter: u128 = 0;
  sieve[0] = 2;
  sieve[1] = 2;

for i in 2..n+1 {
   for j in (2 * i..n+1).step_by(i) {
    sieve[j] = 2;
   }
}

for idx in 2..=n {
  if sieve[idx] == 1 {
  counter += idx as u128;
  }
}

return counter;
}
