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

fn fibonacci_sum(n: u64) -> u64 {
    //eprint!("{} ", n);
    let mut f = vec![0; n as usize];

    if n > 1 {
        f[0] = 1;
        //eprint!("0:{} ", f[0]);
    }

    if n > 2 {
        f[1] = 2;
        //eprint!("1:{} ", f[1]);
    }

    if n > 3 {
        for i in 2..(f.len() - 1) {
            f[i] = f[i - 2] + f[i -1];
            if f[i] > n {
                break;
            }
            //eprint!("{}:{} ", i, f[i]);
        }
    }

    //eprintln!();

    let mut sum = 0;
    for i in 0..(f.len() - 1) {
        if (f[i] % 2) == 0 {
            if f[i] > n {
                break;
            }
            sum += f[i];
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

