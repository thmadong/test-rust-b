pub struct Logger { }

impl Logger {
	pub fn debug(s: &str) {
        println!("[test-rust-b:debug]: {}", s);
    }

    pub fn error(s: &str) {
        println!("[test-rust-b:error]: {}", s);
    }
}