use std::io;

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("A Value");
        let $out = inner.trim().parse::<$type>().expect("Parseble");
    };
}

#[allow(unused_macros)]
macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim();
    };
}

#[allow(unused_macros)]
macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).unwrap();
        let $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
}

fn calculate_sumN(N: i64) -> i64 {
    let STEPS_3  = (N - 1)/3;
    let STEPS_5  = (N - 1)/5;
    let STEPS_15 = (N - 1)/15;

    let sum3  =  3 * ( STEPS_3 * ( STEPS_3 + 1) / 2);
    let sum5  =  5 * ( STEPS_5 * ( STEPS_5 + 1) / 2);
    let sum15 = 15 * (STEPS_15 * (STEPS_15 + 1) / 2);

    return ((sum3 + sum5) - sum15);
}

fn main() {
    read!(t as i64);
    for _ in 0..t {
        read!(n as i64);
        println!("{}",calculate_sumN(n));
    }
}

