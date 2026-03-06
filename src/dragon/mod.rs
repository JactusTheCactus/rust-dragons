use crate::dragon::{
	body::{Body, BodyColour, BodyTexture},
	head::{Head, HeadBreath, HeadTop},
	legs::Legs,
	tail::{Tail, TailLength},
};
pub mod body;
pub mod head;
pub mod legs;
pub mod tail;
#[derive(Debug, Clone, Copy)]
pub struct Dragon {
	pub head: Head,
	pub body: Body,
	pub legs: Legs,
	pub tail: Tail,
}
impl Dragon {
	pub fn new(
		top: HeadTop,
		breath: HeadBreath,
		texture: BodyTexture,
		colour: BodyColour,
		count: u8,
		length: TailLength,
	) -> Self {
		Self {
			head: Head::new(top, breath),
			body: Body::new(texture, colour),
			legs: Legs::new(count),
			tail: Tail::new(length),
		}
	}
}
