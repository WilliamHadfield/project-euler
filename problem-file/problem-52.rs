fn key(n : u32) -> Vec<char> {
  let mut d: Vec<char> = n.to_string().chars().collect();
  d.sort();
  d
}

fn main() {
let mut x: u32 = 1;
let mut container = vec![2,3,4,5,6];

loop {
  let k = key(x);

  if container.iter().all(|m| key(x * m) == k) {
    println!("{}", x);
    break;
  }
  x += 1;
}
}
