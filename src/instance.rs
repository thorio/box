use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

pub struct Instance {
	name: String,
	path: PathBuf,
}

impl Instance {
	pub fn new(name: String, path: PathBuf) -> Self {
		Instance { name, path }
	}

	pub fn build(&self) {
		self.run_compose(vec!["build"], false, false);
	}

	pub fn up(&self) {
		self.run_compose(vec!["up", "-d", "--timeout", "0", "--no-build"], false, false);
	}

	pub fn down(&self) {
		self.run_compose(vec!["down", "--remove-orphans", "--timeout", "0"], false, false);
	}

	pub fn shell(&self) {
		self.run_compose(vec!["exec", "box", "/init/entry.sh"], true, false);
	}

	pub fn has_image(&self) -> bool {
		let output = self.run_compose(vec!["images", "-q"], false, true);

		!output.stdout.is_empty()
	}

	pub fn is_running(&self) -> bool {
		let output = self.run_compose(vec!["ps", "-q"], false, true);

		!output.stdout.is_empty()
	}

	fn get_command(&self, program: &str) -> Command {
		let mut cmd = Command::new(program);

		cmd.current_dir(&self.path)
			.env("PUID", "1000")
			.env("PGID", "1000")
			.env("PUSER", "bob")
			.env("BOX_IMAGE_NAME", self.get_image_name())
			.env("BOX_CONTAINER_NAME", self.get_container_name());

		cmd
	}

	fn run_compose(&self, args: Vec<&str>, use_stdin: bool, capture_out: bool) -> Output {
		self.get_command("docker-compose")
			.args(args)
			.stderr(if capture_out { Stdio::piped() } else { Stdio::inherit() })
			.stdout(if capture_out { Stdio::piped() } else { Stdio::inherit() })
			.stdin(if use_stdin { Stdio::inherit() } else { Stdio::null() })
			.output()
			.expect("compose failed")
	}

	fn get_image_name(&self) -> String {
		let mut name = self.get_container_name();
		name.push_str(":local");
		name
	}

	fn get_container_name(&self) -> String {
		format!("box-{}", self.name)
	}
}
