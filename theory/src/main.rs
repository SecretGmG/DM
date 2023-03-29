#![allow(non_snake_case)]
#![allow(dead_code)]
mod number_theory;
use number_theory::*;

fn main() {
    //run_RSA(7727, 7573, 199, 42);
    //run_euklid(2828, 224);
    println!("{:?}", egcd(54, 16));
}
pub fn run_euklid(a: i128, b: i128){
    let (gcd, s, t) = egcd(a, b);
    println!("{0}*{2} + {1}*{3} = gcd({0},{1}) = {4}", a, b, s, t, gcd);
}
pub fn run_RSA(p: i128, q: i128, e: i128, msg: i128){
    let (pubk, privk) = RSA_key_gen(p, q, e).unwrap();

    let cypher = RSA_enc(pubk, msg);
    let dec = RSA_dec(privk, cypher, pubk);
    
    println!("pub-key: {:?}", pubk);
    println!("priv-key: {}", privk);
    println!("msg: {}", msg);
    println!("cypher: {}", cypher);
    println!("decripted: {}", dec);
}