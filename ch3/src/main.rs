fn celc_to_farh(celc: f64) -> f64 {
    celc * 9.0 / 5.0 + 32.0
}

fn main() {
    // Converting Celsius to Fahrenheit
    let celc = 30.0;
    let farh = celc_to_farh(celc);
    println!("{}°C = {}°F", celc, farh);

    // Generating the nth Fibonacci number
}
