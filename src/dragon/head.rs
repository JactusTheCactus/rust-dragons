#[derive(Debug, Copy, Clone)]
pub enum HeadTop {
	Horns,
	Ears,
}
#[derive(Debug, Copy, Clone)]
pub enum HeadBreath {
	None,
	Fire,
	Ice,
	Toxic,
	Wind,
}
#[derive(Debug, Clone, Copy)]
pub struct Head {
	pub top: HeadTop,
	pub breath: HeadBreath,
}
impl Head {
	pub fn new(top: HeadTop, breath: HeadBreath) -> Self {
		Self { top, breath }
	}
}
