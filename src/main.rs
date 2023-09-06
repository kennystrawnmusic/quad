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
    if let Some(a) = v.get(1) {
        if let Some(b) = v.get(2) {
            if let Some(c) = v.get(3) {
                let a = a.parse::<i64>().unwrap_or_else(|_| {
                    eprintln!("Not a number");
                    exit(1)
                });
                let b = b.parse::<i64>().unwrap_or_else(|_| {
                    eprintln!("Not a number");
                    exit(1)
                });
                let c = c.parse::<i64>().unwrap_or_else(|_| {
                    eprintln!("Not a number");
                    exit(1)
                });
                println!("{:?}", quad(a, b, c));
            } else {
                eprintln!("Usage: quad a b c where\na is the first coefficient in the equation you're trying to solve,\nb is the second coefficient in the equation you're trying to solve,\nand c is the third coefficient in the equation you're trying to solve");
                exit(1);
            }
        } else {
            eprintln!("Usage: quad a b c where\na is the first coefficient in the equation you're trying to solve,\nb is the second coefficient in the equation you're trying to solve,\nand c is the third coefficient in the equation you're trying to solve");
            exit(1);
        }
    } else {
        eprintln!("Usage: quad a b c where\na is the first coefficient in the equation you're trying to solve,\nb is the second coefficient in the equation you're trying to solve,\nand c is the third coefficient in the equation you're trying to solve");
        exit(1);
    }
}
