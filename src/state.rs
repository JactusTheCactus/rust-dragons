use {
	crate::dragon::{Dragon as D, body::Body, head::Head, legs::Legs, tail::Tail},
	std::process::exit,
};
pub struct State {
	pub dragons: Vec<D>,
}
impl State {
	pub fn new(dragons: Vec<D>) -> Self {
		Self { dragons }
	}
	pub fn list(&self) {
		#[derive(Debug)]
		struct Dragon(Head, Body, Legs, Tail);
		for i in &self.dragons {
			println!(
				"{dragon:#?}",
				dragon = Dragon(i.head, i.body, i.legs, i.tail)
			);
		}
	}
	pub fn quit(&self) {
		exit(0);
	}
}
