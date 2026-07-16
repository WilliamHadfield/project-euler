fn main() {
   let n = 4000000;
   let mut previous = 1;
   let mut current = 2;
   let mut sum = 0;
   let mut storage = 0;

   loop {
    if current < n {
    if current % 2 == 0 {
        sum += current;
        let storage = current;
    current += previous;
    previous = storage;
    } else {
    let storage = current;
    current += previous;
    previous = storage;
    }
    } else {
        break;
    }
   }
   println!("{}", sum);


}
