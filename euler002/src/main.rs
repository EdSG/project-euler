use std::io;

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("A Value");
        let $out = inner.trim().parse::<$type>().expect("Parseble");
    };
}

fn fibonacci_sum(n: u64) -> u64 {
    //eprint!("{} ", n);
    let mut sum = 0;

    if n > 3 {
        let mut f0 = 0;
        let mut f1 = 1;

        loop {
            let f2 = f0 + f1;
            if f2 > n {
                break;
            } else if (f2 % 2) == 0 {
                sum += f2;
            }
            let _f0 = f0;
            f0 = f1;
            f1 = _f0 + f1;
            //eprint!("{}:{} ", i, f[i]);
        }
    }

    return sum;
}

fn main() {
    read!(t as u64);
    for _ in 0..t {
        read!(n as u64);
        println!("{}",fibonacci_sum(n));
    }
}

