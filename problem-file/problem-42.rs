use std::fs;


fn main() {
let content = fs::read_to_string("project_euler_42.txt").unwrap();

let words: Vec<&str> = content.split(',').map(|x| x.trim().trim_matches('"')).collect();


let container : Vec<String> = words.iter().map(|x| x.to_string()).collect();
let mut count = 0;
for i in container {
  let score: u32 = i.chars().map(|x| x as u32 - 'A' as u32 + 1).sum();
  let d = score * 8 + 1;
  let c = d as f64;
  let r = c.sqrt();
  let g = r as u32;
  if g * g == d {
    count += 1;
  }

  }

  println!("{}", count);

}


