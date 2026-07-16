fn main() {
 let mut addition_sum = 0;
 let mut multiplication_sum = 0;
 let n = 100;

 for i in 0..=n {
    addition_sum += i;
    
 }

 addition_sum *= addition_sum;


 for a in 1..=n {
    multiplication_sum += a * a;

 }

 let storage = addition_sum - multiplication_sum;


 println!("{}", storage);


}
