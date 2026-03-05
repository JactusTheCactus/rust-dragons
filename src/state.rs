use crate::dragon::Dragon;
pub struct State {
	pub dragons: Vec<Dragon>,
}
impl State {
	pub fn new(dragons: Vec<Dragon>) -> Self {
		Self { dragons }
	}
	pub fn list(&self) {
		for i in &self.dragons {
			println!("{i:#?}");
		}
	}
}
