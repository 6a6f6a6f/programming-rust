use std::env;
use std::str::FromStr;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n
    }
    n
}

fn main() {
    // Ainda sendo um vetor, é necessário marca-lo como mutável!
    // Por padrão é um Vec<u64>.
    let mut numbers = Vec::new();

    // Um loop genérico, que pula o primeiro elemento do array de args(). 
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("Could not parse number from argument"));
    }

    if numbers.len() == 0 {
        eprintln!("No numbers given!");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}!", numbers, d);
}
