use std::path::PathBuf;

use cli::{Args, Verb};
use instance::Instance;

mod cli;
mod env;
mod instance;

fn main() {
	let args = cli::parse_args();

	if args.command.is_none() {
		infer(args);
		return;
	}

	match args.command.as_ref().unwrap() {
		Verb::Build => build(args),
		Verb::Up => up(args),
		Verb::Down => down(args),
		Verb::Shell => shell(args),
	}
}

fn get_instance() -> Instance {
	Instance::new(
		String::from("default"),
		PathBuf::from("/home/thorou/.local/share/box/build"),
	)
}

fn infer(_args: Args) {
	println!("inferring")
}

fn build(_args: Args) {
	get_instance().build();
}

fn up(_args: Args) {
	let instance = get_instance();

	if !instance.has_image() {
		instance.build();
	}

	instance.up();
}

fn down(_args: Args) {
	get_instance().down();
}

fn shell(_args: Args) {
	let instance = get_instance();

	if !instance.has_image() {
		instance.build();
	}

	if !instance.is_running() {
		instance.up();
	}

	instance.shell();
}
