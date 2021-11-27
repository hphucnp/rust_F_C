fn main() {
    println!("{}", f2c(32.0));
    println!("{}", c2f(32.0));
}

fn f2c(f: f64) -> f64 {
    (f - 32.0) * 5.0 /9.0
}

fn c2f(c: f64) -> f64 {
    c / 5.0 * 9.0 + 32.0
}