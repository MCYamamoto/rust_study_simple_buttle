pub struct Logger;
impl Logger {
    pub fn log_println(message: &str) {
        println!("{}", message)
    }
}