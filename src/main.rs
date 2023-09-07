use std::env::args;
use std::process::exit;

fn quad(a: i64, b: i64, c: i64) -> (f64, f64) {
    let a = a as f64;
    let b = b as f64;
    let c = c as f64;

    let discrim = b.powi(2) - 4.0 * a * c;

    let pos = (-b + discrim.sqrt()) / 2.0 * a;
    let neg = (-b - discrim.sqrt()) / 2.0 * a;

    (pos, neg)
}

fn main() {
    let v = args().collect::<Vec<_>>();

    let raw_a = v.get(1).unwrap_or_else(|| {
        eprintln!("Usage: quad a b c where\na is the first coefficient in the equation you're trying to solve,\nb is the second coefficient in the equation you're trying to solve,\nand c is the third coefficient in the equation you're trying to solve");
        exit(1)
    });

    let raw_b = v.get(2).unwrap_or_else(|| {
        eprintln!("Usage: quad a b c where\na is the first coefficient in the equation you're trying to solve,\nb is the second coefficient in the equation you're trying to solve,\nand c is the third coefficient in the equation you're trying to solve");
        exit(1)
    });

    let raw_c = v.get(3).unwrap_or_else(|| {
        eprintln!("Usage: quad a b c where\na is the first coefficient in the equation you're trying to solve,\nb is the second coefficient in the equation you're trying to solve,\nand c is the third coefficient in the equation you're trying to solve");
        exit(1)
    });

    let a = raw_a.parse::<i64>().unwrap_or_else(|_| {
        eprintln!("Not a number");
        exit(1)
    });

    let b = raw_b.parse::<i64>().unwrap_or_else(|_| {
        eprintln!("Not a number");
        exit(1)
    });

    let c = raw_c.parse::<i64>().unwrap_or_else(|_| {
        eprintln!("Not a number");
        exit(1)
    });

    println!("{:?}", quad(a, b, c));
}
