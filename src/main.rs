use std::env;

#[allow(dead_code)]
fn error(_x: &str) -> ! {
    eprintln!("Couldn't convert input {} to unsigned integer", _x);
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Wrong number of inputs!");
        std::process::exit(1);
    }

    let x: u64 = args[1].parse().unwrap_or_else(|_| error(&args[1]));
    let y: u64 = args[2].parse().unwrap_or_else(|_| error(&args[2]));
    let m: u64 = args[3].parse().unwrap_or_else(|_| error(&args[3]));

    let result = modexp(x, y, m);

    println!("{}**{} % {} = {}", x, y, m, result);
}

fn modexp(x: u64, y: u64, m: u64) -> u64 {
    if x == 0 {
        return 0;
    }

    if y == 0 {
        return 1;
    }

    let mut z = modexp(x, y / 2, m);

    z = (z * z) % m;

    if y % 2 != 0 {
        z = (z * x) % m;
    }

    return z;
}

#[test]
fn test_trivia() {
    assert!(modexp(1, 2, 3) == 1);
}
