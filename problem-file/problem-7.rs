


fn main() {
let k = 10001;
let n = 120000;

let d = sieve(n);

println!("{}", d);


}



fn sieve(n: usize, ) -> usize {

let mut sieve = vec![1; n+1];

sieve[0] = 2;
sieve[1] = 2;

for i in 2..n+1 {
  
    for j in (2 * i..n+1).step_by(i) {
      sieve[j] = 2;
    }
  
}

let mut count = 0;
for idx in 2..=n {
  if sieve[idx] == 1 {
    count += 1;
    if count == 10001 {
      return idx;
    }
  }
}

return 0;

}
