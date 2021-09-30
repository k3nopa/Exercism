// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
    }
}
pub fn info(message: &str) -> String {
    let mut base_str = String::from("[INFO]: ");
    base_str.push_str(message);
    base_str
}
pub fn warn(message: &str) -> String {
    let mut base_str = String::from("[WARNING]: ");
    base_str.push_str(message);
    base_str
}
pub fn error(message: &str) -> String {
    let mut base_str = String::from("[ERROR]: ");
    base_str.push_str(message);
    base_str
}
