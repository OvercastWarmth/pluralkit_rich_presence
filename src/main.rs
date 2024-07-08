#![allow(dead_code)]
#![allow(unused_variables)]

/* POST MVP TODO
 * - Configuration
 * - Loop
 *
*/

use std::{error::Error, process::exit, thread::sleep, time::Duration};

use discord_rich_presence::{activity::Activity, DiscordIpc, DiscordIpcClient};

use pluralkit_rich_presence::{
	activity_handler::construct_activity_text,
	pluralkit::get_fronters,
};

use reqwest::header::HeaderMap;

const USER_AGENT: &str = concat!(
	"pluralkit_rich_presence/",
	env!("CARGO_PKG_VERSION"),
	" (@overcastwarmth or @fnige on Discord)"
);

fn main() {
	match run() {
		Ok(_) => (),
		Err(e) => {
			println!("Error! {e}");
			exit(1); // Probably not best practice but we don't care :3
		}
	}
}

fn run() -> Result<(), Box<dyn Error>> {
	let mut discord_client = DiscordIpcClient::new("1173850713743446027")?; // TODO: Grab ID from config file
	discord_client.connect()?;

	let request_client = reqwest::blocking::Client::new();
	let mut headers = HeaderMap::new();
	headers.append("User-Agent", USER_AGENT.parse()?);
	// TODO: Add ability to authorise with a PK token

	loop {
		let switch = get_fronters(&request_client, &headers)?;

		let activity = construct_activity_text(&switch.members)?;

		// Match statement is a workaround for Discord seemingly refusing to display the
		// rich presence when the state is present but empty
		discord_client.set_activity(match &activity.1 {
			None => Activity::new().details(activity.0.as_str()),
			Some(state) => Activity::new()
				.details(activity.0.as_str())
				.state(state.as_str()),
		})?;

		sleep(Duration::from_secs(10));
	};
}
