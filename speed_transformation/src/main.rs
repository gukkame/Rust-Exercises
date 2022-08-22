pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    let m = km_h * 1000.0;
    return m/3600.0
}