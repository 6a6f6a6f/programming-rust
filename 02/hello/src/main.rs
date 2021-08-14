fn main() {
    let result = gcd(14, 15);
    println!("GCD: {}", result);
}

/*
* Um pouco de funções, resolvendo omaior divisor comum de dois números
* utilizando o algoritmo de Euclid
*/

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

// Testezinho para a funçõo gcd:

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 4 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
