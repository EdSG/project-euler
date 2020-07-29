use std::io;

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("A Value");
        let $out = inner.trim().parse::<$type>().expect("Parseble");
    };
}

fn largest_prime_factor(n: u64) -> u64 {
    eprint!("{} ", n);
    let mut factor = n;

    return factor;
}

fn main() {
    read!(t as u64);
    for _ in 0..t {
        read!(n as u64);
        println!("{}",largest_prime_factor(n));
    }
}

