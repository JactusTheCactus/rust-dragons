#[derive(Debug, Clone, Copy)]
pub enum BodyTexture {
	Scales,
	Feathers,
}
#[derive(Debug, Clone, Copy)]
pub enum BodyColour {
	Red,
	Orange,
	Yellow,
	Green,
	Purple,
	Pink,
	Brown,
	Black,
	Grey,
	White,
	Gold,
	Silver,
	Bronze,
}
#[derive(Debug, Copy, Clone)]
pub struct Body {
	pub texture: BodyTexture,
	pub colour: BodyColour,
}
impl Body {
	pub fn new(texture: BodyTexture, colour: BodyColour) -> Self {
		Self { texture, colour }
	}
}
