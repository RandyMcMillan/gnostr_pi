use num_bigint::BigInt;
use std::{env,process,str::FromStr};
fn main() {

for argument in env::args() {
    println!("{}", argument);

let count = match u64::from_str(&argument) {
        Ok(n) => n,
        Err(_) => {
            println!("Error: Could not parse string to u64");
            return; // Exit the function if parsing fails
        }
    };


    calc_pi(count as u64);
}

    //calc_pi(argument as u64);
}

fn calc_pi(count: u64) {
    let mut q = BigInt::from(1);
    let mut r = BigInt::from(0);
    let mut t = BigInt::from(1);
    let mut k = BigInt::from(1);
    let mut n = BigInt::from(3);
    let mut l = BigInt::from(3);
    let mut first = true;
    let mut count = 0u32;
    loop {
        if count >= 10 {process::exit(0);}
        if &q * 4 + &r - &t < &n * &t {
            print!("count={}\n", count);
            print!("{}\n", n);
            if first {
                print!(".");
                first = false;
            }
            let nr = (&r - &n * &t) * 10;
            n = (&q * 3 + &r) * 10 / &t - &n * 10;
            q *= 10;
            r = nr;
        } else {
            let nr = (&q * 2 + &r) * &l;
            let nn = (&q * &k * 7 + 2 + &r * &l) / (&t * &l);
            q *= &k;
            t *= &l;
            l += 2;
            k += 1;
            n = nn;
            r = nr;
        }
    count = count + 1u32;
    }
}
