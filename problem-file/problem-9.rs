
fn main() {
   let mut vec: Vec<u32> = Vec::new();

    for c in 2..=500 {
        for a in 1..=c {
       let b = 1000 - c - a;
       if a < b && a * a + b * b == c * c {
        vec.push(b);
        vec.push(a);
        vec.push(c);
       }

        }
    }

    let b_1 = vec[0];
    let a_1 = vec[1];
    let c_1 = vec[2];
    println!("{} {}, {}", b_1,a_1,c_1 );

}
