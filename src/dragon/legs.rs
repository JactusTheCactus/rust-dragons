#[derive(Debug, Clone, Copy)]
pub struct Legs {
	pub count: u8,
}
impl Legs {
	pub fn new(count: u8) -> Self {
		Self { count }
	}
}
