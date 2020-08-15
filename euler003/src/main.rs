use std::io;

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("A Value");
        let $out = inner.trim().parse::<$type>().expect("Parseble");
    };
}

fn is_prime(n: u64) -> bool {
    eprint!("is_prime({}) ", n);
    let mut factor = n - 1;
    while factor > 1 {
        eprint!("{} ", factor);
        if ((n % factor) == 0) && (factor > 1) {
            return false;
        }
        factor -= 1;
    }
    eprintln!();

    return true;
}

fn largest_prime_factor(n: u64) -> u64 {
    eprint!("largest_prime_factor({}) ", n);
    let mut max_factor = n;
    let mut factor = n;

    while factor > 1 {
        eprint!("{}:{} ", max_factor, factor);
        if is_prime(factor) && ((n % factor) == 0) {
            max_factor = factor;
            break;
        }
        factor -= 1;
    }
    eprintln!();

    return max_factor;
}

fn main() {
    read!(t as u64);
    for _ in 0..t {
        read!(n as u64);
        println!("{}",largest_prime_factor(n));
    }
}

