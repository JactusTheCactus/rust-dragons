use {
	clap_repl::{
		ClapEditor,
		reedline::{DefaultPrompt, DefaultPromptSegment::Basic},
	},
	dragons::{
		Command::{self, List, Quit},
		dragon::{
			Dragon,
			body::{
				BodyColour::{Brown, Red},
				BodyTexture::{Feathers, Scales},
			},
			head::{
				HeadBreath::{Fire, Ice},
				HeadTop::{Ears, Horns},
			},
			tail::TailLength::{Long, Short},
		},
		state::State,
	},
};
fn main() {
	let state = State::new(vec![
		Dragon::new(Horns, Fire, Scales, Red, 2, Long),
		Dragon::new(Ears, Ice, Feathers, Brown, 4, Short),
	]);
	ClapEditor::<Command>::builder()
		.with_prompt(Box::new(DefaultPrompt {
			left_prompt: Basic("dragons".to_owned()),
			..DefaultPrompt::default()
		}))
		.build()
		.repl(|command| match command {
			Quit => state.quit(),
			List => state.list(),
		});
}
