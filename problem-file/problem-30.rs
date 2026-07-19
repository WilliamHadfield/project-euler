const lookup: [u32; 10] = [0, 1, 32, 243, 1024, 3125, 7776, 16807, 32768, 59049];

fn main () {

let n = 360_000; // 6 x 9^5 is roughly a bit smaller than this.
let mut final_container: Vec<u32> = Vec::new();

for g in 1..n {
let mut dummy: u32 = 0;
let digits: Vec<char> = g.to_string().chars().collect();
let integers: Vec<u32>  = digits.into_iter().map(|x| x.to_digit(10).unwrap()).collect();

for (i, t) in integers.iter().enumerate() {
dummy += lookup[*t as usize];
}
if dummy == g {
  final_container.push(g);
}
}

println!("{:?}", final_container);

}
