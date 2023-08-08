use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
	#[clap(subcommand)]
	pub command: Option<Verb>,
}

#[derive(Subcommand)]
pub enum Verb {
	Build,
	Up,
	Down,
	Shell,
}

pub fn parse_args() -> Args {
	Args::parse()
}
