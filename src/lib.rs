use {clap::Parser, std::process::exit};
pub mod dragon;
pub mod state;
#[derive(Debug, Parser)]
#[command(name = "")]
pub enum Command {
	Quit,
	List,
}
pub fn quit() {
	exit(0)
}
