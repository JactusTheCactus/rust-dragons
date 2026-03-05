use {
	clap_repl::{
		ClapEditor,
		reedline::{DefaultPrompt, DefaultPromptSegment, FileBackedHistory},
	},
	dragons::{
		Command::{self, List, Quit},
		dragon::{BodyColour::*, BodyTexture::*, Dragon, HeadBreath::*, HeadTop::*, TailLength::*},
		quit,
		state::State,
	},
};
fn main() {
	let state = State::new(vec![
		Dragon::new(Horns, Fire, Scales, Red, 2, Long),
		Dragon::new(Ears, Ice, Feathers, Brown, 4, Short),
	]);
	let prompt = DefaultPrompt {
		left_prompt: DefaultPromptSegment::Basic("dragons".to_owned()),
		..DefaultPrompt::default()
	};
	ClapEditor::<Command>::builder()
		.with_prompt(Box::new(prompt))
		.with_editor_hook(|reed| {
			reed.with_history(Box::new(
				FileBackedHistory::with_file(10000, "/tmp/clap-repl-dragons-history".into())
					.unwrap(),
			))
		})
		.build().repl(|command| match command {
		Quit => quit(),
		List => state.list(),
	});
}
