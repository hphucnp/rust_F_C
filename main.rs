fn main() {
    println!("{}", f2c(32.0));
    println!("{}", c2f(32.0));
    println!("{}, {}", fibonacci(5), fibonacci(8))
}

fn f2c(f: f64) -> f64 {
    (f - 32.0) * 5.0 /9.0
}

fn c2f(c: f64) -> f64 {
    c / 5.0 * 9.0 + 32.0
}

fn fibonacci(n: usize) -> usize {
    if n == 0 {
        return 0
    }
    else if n == 1 {
        return 1
    }
    else {
        let mut v: Vec<usize> = Vec::new();
        v.push(0);
        v.push(1);
        for num in 2..(n+1) {
            v.push(v[num - 1] + v[num - 2]);
        }
        return v[n];
    }

}