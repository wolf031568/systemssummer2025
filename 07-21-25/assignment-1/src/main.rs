fn main() {
    let original_temperature : f64 = 25.75;
    println!("Fahrenheit to Celsius: {}", fahrenheit_to_celsius(original_temperature));
    println!("Celsius to Fahrenheit {}", celsius_to_fahrenheit(original_temperature))
}
fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - 32.0) * (5.0/9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    (c * (9.0/5.0)) + 32.0
}
