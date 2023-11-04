fn celc_to_farh(celc: f64) -> f64 {
    celc * 9.0 / 5.0 + 32.0
}

fn nth_fibo(n: u32) -> u32 {
    let mut fib: u32 = 0;
    let mut prev: u32 = 1;
    for _ in 0..n {
        let temp = fib;
        fib = prev + fib;
        prev = temp;
    }
    fib
}

fn main() {
    // Converting Celsius to Fahrenheit
    let celc = 30.0;
    let farh = celc_to_farh(celc);
    println!("{}°C = {}°F", celc, farh);

    // Generating the nth Fibonacci number
    let n = 10;
    println!("The {}th Fibonacci number is {}", n, nth_fibo(n));
}
