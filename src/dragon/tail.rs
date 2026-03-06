#[derive(Debug, Clone, Copy)]
pub enum TailLength {
	Long,
	Short,
}
#[derive(Debug, Clone, Copy)]
pub struct Tail {
	pub length: TailLength,
}
impl Tail {
	pub fn new(length: TailLength) -> Self {
		Self { length }
	}
}
