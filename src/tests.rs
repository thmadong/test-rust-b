#[test]
fn it_works() {
	assert_eq!(2 + 2, 4);
}

#[test]
fn test_print() {
	use crate::utils::Logger;

	let s: &str = "123456";
	Logger::debug(s);
	Logger::error(s);
}