fn main() {
let n = 1000;
let mut best = (0,0);

for p in 1..=1000 {
  let mut count = 0;
 
for a in 1..=p/3 {
 for b in a..(p-a)/2 + 1 {
  let c = p - a - b;
  if a * a + b * b == c * c { count += 1;}
 }
} 
if count > best.1 { best = (p, count); }
  }

  println!("{:?} ", best);

}
