fn main() {
let mut storage: Vec<u32> = Vec::new();
let n = 1_000_000;
let mut counter = 0;
  for i in 1..n {
if binary_converter(i) == true && char_converter(i) == true {
storage.push(i);
}
  }
  let counter: u32 = storage.iter().sum();
  println!("{}", counter);

}


fn binary_converter(mut n: u32) -> bool {
let mut s = String::new();
while n > 0 {
  s.insert(0, if n % 2 != 0 {'i'} else {'0'});
  n = n >> 1;
}
if s.chars().rev().eq(s.chars()) {
  return true;
} else {
  return false;
}
}

fn char_converter(i : u32) -> bool {

let c = i.to_string();

if c.chars().rev().eq(c.chars()) {
  return true;
} else {
  return false;
}
}
