pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0 + 0.000000000000001
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    fn eql(a: f64, b: f64) -> bool {
        (b - a).abs() < EPSILON
    }
    #[test]
    fn test_f_to_c() {
        let temp_f = 20.0;
        println!("{}°F = {}°C", temp_f, fahrenheit_to_celsius(temp_f));
        assert!(eql(fahrenheit_to_celsius(temp_f), -6.666666666666666));
        let temp_f = 83.0;
        println!("{}°F = {}°C", temp_f, fahrenheit_to_celsius(temp_f));
        assert!(eql(fahrenheit_to_celsius(temp_f), 28.333333333333332));
    }
}
