pub struct Config {
	pub cc: String,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			cc: "gcc".to_owned(),
		}
	}
}
