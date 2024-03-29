use num_bigint::BigInt;
use std::{env, process, str::FromStr};
fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("Number of arguments: {}", args.len() - 1);
    let limit = u64::from_str(&args[1]).unwrap();

    calc_pi(limit as u64);
}

fn calc_pi(limit: u64) {
    //println!("limit={}", limit);
    let mut q = BigInt::from(1);
    let mut r = BigInt::from(0);
    let mut t = BigInt::from(1);
    let mut k = BigInt::from(1);
    let mut n = BigInt::from(3);
    let mut l = BigInt::from(3);
    let mut first = true;
    let mut count = 0u64;
    //println!("count={}", count);
    loop {
        if count == limit {
            //println!("limit={}", limit);
            //println!("count={}", count);
            process::exit(0);
        }
        if &q * 4 + &r - &t < &n * &t {
            //print!("count={}\n", count);
            if first {
                //print!("3.");
                first = false;
            }else{
            print!("{}", n);

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
        count = count + 1u64;
    }
}
