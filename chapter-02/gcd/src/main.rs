fn main() {
    let a = 54;
    let b = 24;
    let g = gcd(a, b);
    println!("Greatest common divisor of {} and {} is {}.", a, b, g);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
