fn main() {
   let n: i64 = 999;
   let mut n2: i64 = 999;
   let mut highest = 0;
    while n2 > 100 {
    for i in 100..=n {
     let container = n2 * i;
    let container2 = is_palindrome(container);
    if container2 == true {
    if container > highest {
     highest = container;
    } else {

    }

    }
    }
    n2 -= 1;
}
println!("{}", highest);

}

pub fn is_palindrome(x: i64) -> bool {
    if x < 0 {
        return false;
    }

    let mut original = x ;
    let mut reversed: i64  = 0;

    while original > 0 {
        reversed = reversed * 10 + original % 10;
        original /= 10;
    }

    
    if reversed > i64::MAX  {
        return false;
    }

    x == reversed
}   
