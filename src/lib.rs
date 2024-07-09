mod config;
mod discord;
mod error;
mod pluralkit;

use discord::construct_activity_text;
use discord_rich_presence::{activity::Activity, DiscordIpc, DiscordIpcClient};
use error::RichPresenceError;
use pluralkit::get_switch;
use reqwest::header::HeaderMap;
use std::{error::Error, thread::sleep, time::Duration};

const USER_AGENT: &str = concat!(
	"pluralkit_rich_presence/",
	env!("CARGO_PKG_VERSION"),
	" (@overcastwarmth or @fnige on Discord)"
);

pub fn run() -> Result<(), Box<dyn Error>> {
	let mut discord_client = DiscordIpcClient::new("1173850713743446027")?; // TODO: Grab ID from config file
	match discord_client.connect() {
		Err(error) => Err(RichPresenceError::InitialDiscordConnection(error)),
		_ => Ok(()),
	}?;

	let request_client = reqwest::blocking::Client::new();
	let mut headers = HeaderMap::new();
	headers.append("User-Agent", USER_AGENT.parse()?);
	// TODO: Add ability to authorise with a PK token

	loop {
		let switch = match get_switch(&request_client, &headers) {
			Err(error) => Err(RichPresenceError::PluralKitSwitchFetch(error)),
			Ok(switch) => Ok(switch),
		}?;

		let activity = construct_activity_text(&switch.members);

		// Match statement is a workaround for Discord seemingly refusing to display the
		// rich presence when the state is present but empty
		match discord_client.set_activity(match &activity.1 {
			None => Activity::new().details(activity.0.as_str()),
			Some(state) => Activity::new()
				.details(activity.0.as_str())
				.state(state.as_str()),
		}) {
			Ok(_) => Ok(()),
			Err(error) => Err(RichPresenceError::SendActivityData(error))
		}?;

		sleep(Duration::from_secs(10));
	}
}
