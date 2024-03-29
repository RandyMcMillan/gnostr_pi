use num_bigint::BigInt;
use std::{env, process, str::FromStr};
fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("Number of arguments: {}", args.len() - 1);
    let limit = u64::from_str(&args[1]).unwrap();
    if limit <= 5 {
        println!("limit={}", limit);
        //TODO print usage
    }
    if (args.len() - 1) == 1 {
        if &args[1] == "-h" || &args[1] == "--help" {}
        let depth = u64::from_str(&args[1]).unwrap();
        println!("depth={}\n", depth);
        calc_pi(depth as u64);
        process::exit(0);
    }
    if (args.len() - 1) == 2 {
        let depth = u64::from_str(&args[1]).unwrap();
        println!("depth={}\n", depth);
        let offset = u64::from_str(&args[2]).unwrap();
        println!("offset={}\n", offset);
        calc_pi_with_offset(depth as u64, offset as u64);
        process::exit(0);
    }
}
fn calc_pi_with_offset(depth: u64, offset: u64) {
    //println!("depth={}", depth);
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
        if count == depth + offset {
            //println!("limit={}", limit);
            println!("count={}", count);
            process::exit(0);
        }
        println!("count={}\n", count);
        if &q * 4 + &r - &t < &n * &t {
            //print!("count={}\n", count);
            if first {
                //we only print pi mantissa
                //print!("3.");
                first = false;
            } else {
                //detect limit and offset
                //dont print for offset number of digits
                //augment depth to limit + offset

                print!("\n{}\n", n);
                //print!("{}", n);
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
            } else {
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
