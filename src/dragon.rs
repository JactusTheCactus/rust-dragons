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
    texture: BodyTexture,
    colour: BodyColour,
}
impl Body {
    pub fn new(texture: BodyTexture, colour: BodyColour) -> Self {
        Self { texture, colour }
    }
}
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
#[derive(Debug, Clone, Copy)]
pub enum TailLength {
    Long,
    Short,
}
#[derive(Debug, Clone, Copy)]
pub struct Tail {
    length: TailLength,
}
impl Tail {
    pub fn new(length: TailLength) -> Self {
        Self { length }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Legs {
    count: u8,
}
impl Legs {
    pub fn new(count: u8) -> Self {
        Self { count }
    }
}
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
