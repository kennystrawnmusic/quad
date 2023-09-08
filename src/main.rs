use std::env::args;
use std::process::exit;

fn quadf(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discrim = b.powi(2) - (4.0 * a * c);

    let pos = (-b + discrim.sqrt()) / (2.0 * a);
    let neg = (-b - discrim.sqrt()) / (2.0 * a);

    (pos, neg)
}

fn _quadi(a: i64, b: i64, c: i64) -> (f64, f64) {
    let a = a as f64;
    let b = b as f64;
    let c = c as f64;

    let discrim = b.powi(2) - (4.0 * a * c);

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

    let a = if let Ok(a) = raw_a.parse::<f64>() {
        a
    } else if let Ok(a) = raw_a.parse::<i64>() {
        a as f64
    } else {
        eprintln!("Not a number");
        exit(1)
    };

    let b = if let Ok(b) = raw_b.parse::<f64>() {
        b
    } else if let Ok(b) = raw_b.parse::<i64>() {
        b as f64
    } else {
        eprintln!("Not a number");
        exit(1)
    };

    let c = if let Ok(c) = raw_c.parse::<f64>() {
        c
    } else if let Ok(c) = raw_c.parse::<i64>() {
        c as f64
    } else {
        eprintln!("Not a number");
        exit(1)
    };

    println!("{:?}", quadf(a, b, c));
}
