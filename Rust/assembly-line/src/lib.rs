// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
pub fn production_rate_per_hour(speed: u8) -> f64 {
    let base = speed as f64 * 221.0;

    match speed {
        0..=4 => base,
        5..=8 => base * 0.9,
        9..=u8::MAX => base * 0.77,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let prod = production_rate_per_hour(speed);
    (prod / 60.0) as u32
}
