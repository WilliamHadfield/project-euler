fn main() {

 
  
  let g = 2_540_160;
  let mut final_check : Vec<u128> = Vec::new();

  for n in 3..g {
    let mut num: u128 = 1;
    let mut container: Vec<u128> = Vec::new();

    let digits: Vec<char> = n.to_string().chars().collect();
    let integer: Vec<u32> = digits.into_iter().map(|x| x.to_digit(10).unwrap()).collect();

   for i in integer.iter() {
   let mut dummy: u128 = num.clone();
   for k in 1..=*i {
   dummy *= k as u128;
   }
   container.push(dummy);

    }
    let check = container.iter().sum();
    if n == check {
      final_check.push(n);
    }

  }

  println!("{:?}", final_check);

}
