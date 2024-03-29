use std::env;

fn g(q: u64, r: u64, t: u64, k: u64, n: u64, l: u64) -> f64 {
    if 4 * q + r - t < n * t {
        n as f64 / 10f64.powi(l)
    } else {
        g(
            10 * q,
            10 * (r - n * t),
            t,
            k,
            (10 * (3 * q + r)) / t - 10 * n,
            l,
        ) + g(
            q * k,
            (2 * q + r) * l,
            t * l,
            k + 1,
            (q * (7 * k + 2) + r * l) / (t * l),
            l + 2,
        )
    }
}

fn spigot(n_digits: u32) -> f64 {
    let mut sum = 0.0;
    let (mut q, mut r, mut t, mut k, mut n, mut l) = (1, 0, 1, 1, 3, 3);

    while n < n_digits {
        sum += g(q, r, t, k, n, l);
        let new_t = 3 * q + 2 * r;
        let new_r = 10 * r - 5 * k * q;
        q = t;
        r = new_r;
        t = new_t;
        k = -k;
        n += 1;
    }

    sum
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("Usage: pi_spigot <n_digits>");
    }

    let n_digits = args[1].parse::<u32>().expect("Invalid number of digits");
    let pi = spigot(n_digits);
    println!("Pi ({} digits): {}", n_digits, pi);
}

