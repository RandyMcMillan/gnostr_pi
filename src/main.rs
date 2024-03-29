use std::env;

fn spigot(n_digits: u32) -> f64 {
    let mut q = 1.0;
    let mut r = 0.0;
    let mut t = 1.0;
    let mut k = 1.0;
    let mut n = 0.0;
    let mut sum = 0.0;

    loop {
        let mut digit = (k as f64 * q + r) / t;
        sum += digit / 10.0f64.powi(n as i32);
        n += 1.0;

        if n >= n_digits as f64 {
            return sum;
        }

        let new_t = 3.0 * q + 2.0 * r;
        let new_r = 10.0 * r - 5.0 * k * q;
        q = t;
        r = new_r;
        t = new_t;
        k = -k;
    }
}

fn main() {
    let n_digits = 100.0; // Adjust this value for desired precision
    let pi = spigot(n_digits as u32);
    println!("Pi ({} digits): {}", n_digits as i32, pi);
}


//fn main() {
//    let args = env::args().collect::<Vec<String>>();
//    if args.len() < 2 {
//        panic!("Usage: pi_spigot <n_digits>");
//    }
//
//    let n_digits = args[1].parse::<u32>().expect("Invalid number of digits");
//    let pi = spigot(n_digits);
//    println!("Pi ({} digits): {}", n_digits, pi);
//}
//
