use clap::Parser;
pub mod dragon;
pub mod state;
#[derive(Debug, Parser)]
#[command(name = "")]
pub enum Command {
	Quit,
	List,
}
