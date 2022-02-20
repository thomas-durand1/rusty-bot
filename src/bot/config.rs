/// default location for assets storage
const DEFAULT_ASSETS_DIR: &str = "assets";


/// Represents / contains bot settings/configuration
/// 
/// clear_calls: 		used to check if the command calls 
/// 					like ($ping) have to be cleared after the command is executed or not
/// 
/// muted: 				the name here is pretty relevant, says whether or not the bot is muted
/// 
/// flood_delay: 		(experimental, might be deprecated in the future)
/// 					used to set a delay for the flood command, due to discord api rate limit,
/// 					this delay could be useful in order to flood people without having the phenomenon of messages that freeze by batch of 5
/// 
/// maintained: 		whether the poomp command should be maintained or not (e.g: the bot waits for another `$poomp` command call)
/// 
/// poomp_delay: 		max delay to wait before each `$poomp` call, if the delay is passed the bot leaves the channel
/// 
/// assets_directory_path: 	the assets storage location

pub struct Configuration {
	clear_calls: bool,
	muted: bool,
	flood_delay: f32,
	maintained: bool,
	poomp_delay: f32,
	assets_directory_path: String,
}

impl Configuration {
	pub fn new() -> Self {
		Self {
			clear_calls: false,
			muted: false,
			flood_delay: 0.0,
			maintained: false,
			poomp_delay: 0.0,
			assets_directory_path: String::from(DEFAULT_ASSETS_DIR),
		}
	}
	/// Returns if the commands calls are cleared
	pub fn get_clear_calls(&self) -> bool {
		self.clear_calls
	}
	/// Sets whether the commmand calls have to be cleared or not
	/// # Examples
	/// ```
	/// let mut config = Configuration::new();
	/// // default value for clear_command_calls is false;
	/// assert_eq!(false, config.get_clear_command_calls());
	/// config.set_clear_command_calls(true);
	/// assert_eq!(true, config.get_clear_command_calls());
	/// ```
	pub fn clear_calls(&mut self, new_value: bool) {
		self.clear_calls = new_value;
	}
	
	pub fn muted(&self) -> bool {
		self.muted
	}
	pub fn mute(&mut self, new_value: bool) {
		self.muted = new_value;
	}

	pub fn get_flood_delay(&self) -> f32 {
		self.flood_delay
	}
	pub fn set_flood_delay(&mut self, new_value: f32) {
		self.flood_delay = new_value;
	}

	pub fn get_maintained(&self) -> bool {
		self.maintained
	}
	pub fn set_maintained(&mut self, new_value: bool) {
		self.maintained = new_value;
	}

	pub fn get_poomp_delay(&self) -> f32 {
		self.poomp_delay
	}
	pub fn set_poomp_delay(&mut self, new_value: f32) {
		self.poomp_delay = new_value;
	}


	pub fn get_assets_dir(&self) -> String {
		self.assets_directory_path.clone()
	}
	pub fn set_assets_dir(&mut self, new_value: &str) {
		self.assets_directory_path = String::from(new_value);
	}
}

impl PartialEq for Configuration {
	fn eq(&self, other: &Self) -> bool {
		self.clear_calls == other.get_clear_calls() && 
		self.muted == other.muted() && 
		self.flood_delay == other.get_flood_delay() &&
		self.maintained == other.get_maintained() &&
		self.poomp_delay == other.get_poomp_delay() &&
		self.assets_directory_path == other.get_assets_dir()
	}
}

impl Default for Configuration {
	fn default() -> Self {
		Self::new()
	}
}

/// Builder pattern for Configuration
pub struct ConfigBuilder {
	clear_command_calls: Option<bool>,
	mute_bot: Option<bool>,
	flood_delay: Option<f32>,
	maintained: Option<bool>,
	poomp_delay: Option<f32>,
	assets_directory_path: Option<String>
}

impl ConfigBuilder {
	/// Constructor for ConfigBuilder
	pub fn new() -> Self {
		Self {
			clear_command_calls: None,
			mute_bot: None,
			flood_delay: None,
			maintained: None,
			poomp_delay: None,
			assets_directory_path: None
		}
	}
	#[allow(dead_code)]
	/// Configure whether the command calls should be clear or not
	pub fn clear_calls(&mut self, new_value: bool) -> &mut Self{
		self.clear_command_calls = Some(new_value);
		self
	}
	#[allow(dead_code)]
	/// Configure whether the bot has to be mute in voice channel
	pub fn mute_bot(&mut self, new_value: bool) -> &mut Self {
		self.mute_bot = Some(new_value);
		self
	}
	#[allow(dead_code)]
	/// Sets the delay for the flood command
	pub fn flood_delay(&mut self, new_value: Option<f32>) -> &mut Self {
		self.flood_delay = new_value;
		self
	}
	#[allow(dead_code)]
	pub fn maintained(&mut self, new_value: Option<bool>) -> &mut Self {
		self.maintained = new_value;
		self
	}
	#[allow(dead_code)]
	pub fn poomp_delay(&mut self, new_value: Option<f32>) -> &mut Self {
		self.poomp_delay = new_value;
		self
	}
	#[allow(dead_code)]
	pub fn assets_directory_path(&mut self, new_value: Option<String>) -> &mut Self {
		self.assets_directory_path = new_value;
		self
	}

	/// Build function
	pub fn build(&self) -> Configuration {
		let mut new_conf = Configuration::new();

		if let Some(clr) = self.clear_command_calls {
			new_conf.clear_calls(clr)
		}
		if let Some(mute) = self.mute_bot {
			new_conf.mute(mute);
		}
		if let Some(delay) = self.flood_delay {
			new_conf.set_flood_delay(delay);
		}
		if let Some(maintained) = self.maintained {
			new_conf.set_maintained(maintained);
		}
		if let Some(delay) = self.poomp_delay {
			new_conf.set_poomp_delay(delay)
		}
		if let Some(strpath) = &self.assets_directory_path {
			new_conf.set_assets_dir(strpath);
		}
		new_conf
	}
}

use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::RwLock;
pub struct ConfigStore;

impl TypeMapKey for ConfigStore {
    type Value = Arc<RwLock<Configuration>>;
}