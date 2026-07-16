fn main() {
let mut n: i64 = 600851475143;

let mut factors = Vec::new();

let mut d = 2;
while d * d <= n {
    while n % d == 0 {
        factors.push(d);
        n /= d;
    }
    d += 1;
}
if n > 1 {
    factors.push(n);
}

factors.sort();
factors.reverse();

println!("{}", factors[0]);

}

