///Der algorithmus beruht auf folgendem lemma
///a = b*q + r => gcd(a,b) = gcd(b,r)
/// d|a and d|b => d|(a-b*q) => d|r weil r = (a-b*q)
/// d|r and d|b => d|(a-b*q) => d|a weil q = 0 sein kann
/// => gcd(a,b) = gcd(b,r)
pub fn gcd(a: i128, b: i128) -> i128 {
    let mut a = a;
    let mut b = b;
    if a < b {
        (a, b) = (b, a);
    }

    while b != 0 {
        //gcd(a,b) = gcd(b, a%b) because a=b*q+r
        (a, b) = (b, a % b);
    }
    //gcd(a, 0) = a
    a
}
///calculates the chinese remainder theorem
pub fn crt(terms: Vec<(i128, i128)>) -> Option<i128> {
    let m = terms.iter().fold(1, |m, (_, mi)| m * mi);
    let mut sum = 0;
    for (ai, mi) in terms {
        let Mi = m / mi;
        let yi = inverse(Mi, mi)?;
        sum += ai * Mi * yi;
    }
    Some(sum % m)
}
/// (gcd(a,b), s, t) such that gcd(a,b) = s*a+t*b
/// if a, b coprime then 1 = s*a mod b
/// 
/// gcd(54, 28):
/// 54 = 28 * 1 + 26 -> gcd(54, 28) = gcd(28, 26)
/// 28 = 26 * 1 + 2  -> gcd(28, 26) = gcd(26, 2)
/// 26 = 2  * 13 + 0  -> gcd(26, 2) = 2
/// 
/// 2 = 28 - (26 * 1)
/// 2 = 28 - ((54 - 28 * 1) * 1)
/// 2 = 2 * 28 - 1 * 54
/// 2 als linearkombi von 54, 28
pub fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    let mut a = a;
    let mut b = b;

    let swap = a < b;
    if swap {
        (a, b) = (b, a);
    }
    let mut s = (1, 0);
    let mut t = (0, 1);

    while b != 0 {
        let q = a / b;
        (a, b) = (b, a - q * b); //<=> (a, b) = (b, a%b)
        let (s1, s2) = s;
        let (t1, t2) = t;

        s = (s2, s1 - q * s2);
        t = (t2, t1 - q * t2);
    }
    let (s, _) = s;
    let (t, _) = t;
    if swap {
        (a, t, s)
    } else {
        (a, s, t)
    }
}







pub fn modulus(a: i128, m: i128) -> i128 {
    ((a % m) + m) % m
}
pub fn inverse(a: i128, m: i128) -> Option<i128> {
    let (gcd, val, _) = egcd(a, m);
    if gcd != 1 {
        None
    } else {
        Some(modulus(val, m))
    }
}
pub fn RSA_key_gen(p: i128, q: i128, e: i128) -> Option<((i128, i128), i128)> {
    let N = p * q;
    let phiN = (p - 1) * (q - 1); //40
    let d = inverse(e, phiN)?;
    Some(((N, e), d))
}
pub fn RSA_enc((N, e): (i128, i128), m: i128) -> i128 {
    exponetiate(m, e, N)
}
pub fn RSA_dec(d: i128, c: i128, (N, _): (i128, i128)) -> i128 {
    exponetiate(c, d, N)
}
///exponetiates a^p mod m
///runtime O(log2(p))
pub fn exponetiate(a: i128, p: i128, m: i128) -> i128 {
    let mut val = 1;
    let mut a = a;
    let mut p = p;
    while p != 0 {
        if p % 2 == 1 {
            val = val * a % m;
        }
        a = a * a % m;
        p /= 2;
    }
    return val;
}
pub fn get_primes(max: i128) -> Vec<i128>{
    let mut primes = vec![];

    for i in 2..max+1{
        let sqrt = sqrt(i) + 1;
        let mut j = 0;
        if loop{
            if j >= primes.len() || primes[j] > sqrt{
                break true;
            }
            if i % primes[j] == 0{
                break false;
            }
            j+=1;
        }{
            primes.push(i);
        }
    }
    primes
}
pub fn sqrt(val: i128) -> i128{
    (val as f64).sqrt() as i128
}
///anzahl zahlen < m welche teilerfremd zu m sind
///
///
///
pub fn euler_phi(val: i128) -> i128{
    let factors = factorise(val);
    let mut prod = 1;
    let mut prev = 0;
    for factor in factors{
        if prev == factor{
            prod *= factor;
        }
        else{
            prev = factor;
            prod *= factor -1;
        }
    }
    prod
}
pub fn factorise(val: i128) -> Vec<i128>{
    let sqrt = sqrt(val);
    let primes = get_primes(sqrt);
    let mut val = val;
    let mut factors = vec![];
    
    for prime in &primes{
        while val % prime == 0{
            val /= prime;
            factors.push(*prime);
        }
    }
    if val != 1{
        factors.push(val);
    }
    
    return factors;

}