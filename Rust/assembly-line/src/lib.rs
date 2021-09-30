// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
pub fn production_rate_per_hour(speed: u8) -> f64 {
    let base = speed as f64 * 221.0;

    if speed >= 5 && speed <= 8 {
        return base * 0.9;
    } else if speed >= 9 {
        return base * 0.77;
    } else {
        return base;
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let prod = production_rate_per_hour(speed);
    (prod / 60.0) as u32
}
