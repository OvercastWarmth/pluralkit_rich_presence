#![allow(dead_code)]
#![allow(unused_variables)]

/* POST MVP TODO
 * - Configuration
 * - Loop
 *
*/

use std::{error::Error, io::Read, process::exit};

use discord_rich_presence::{activity::Activity, DiscordIpc, DiscordIpcClient};
use reqwest::header::HeaderMap;
use serde::Deserialize;

fn main() {
	match run() {
		Ok(_) => (),
		Err(e) => {
			println!("Error! {e}");
			exit(1); // Probably not best practice but we don't care :3
		}
	}
}

#[derive(Deserialize, Debug)]
struct PluralKitMember {
	name: String,
}

#[derive(Deserialize, Debug)]
struct PluralKitSwitch {
	members: Vec<PluralKitMember>,
}

fn run() -> Result<(), Box<dyn Error>> {
	let mut discord_client = DiscordIpcClient::new("1173850713743446027")?; // TODO: Grab ID from config file
	discord_client.connect()?;
	println!("Connected to Discord RPC");

	let request_client = reqwest::blocking::Client::new();
	let mut headers = HeaderMap::new();
	headers.append(
		"User-Agent",
		"pluralkit_rich_presence/1.0.0 (@overcastwarmth or @fnige on Discord)".parse()?,
	);
	// TODO: Add ability to authorise with a PK token

	let switch: PluralKitSwitch = get_fronters(request_client, headers)?;
	println!("Found new fronter list: {:?}", switch);

	// TODO: Customisation
	let activity = construct_activity_text(&switch)?;

	println!("Constructed new activity: {:?}", activity);

	discord_client.set_activity(match &activity.1 {
		None => Activity::new().details(activity.0.as_str()),
		Some(state) => Activity::new()
			.details(activity.0.as_str())
			.state(state.as_str()),
	})?;

	println!("Activity sent to Discord RPC!");

	Ok(())
}

fn get_fronters(
	request_client: reqwest::blocking::Client,
	headers: HeaderMap,
) -> Result<PluralKitSwitch, Box<dyn Error>> {
	let mut res = request_client
		.get("https://api.pluralkit.me/v2/systems/vgspl/fronters") // TODO: Unhardcode this later pls :3
		.headers(headers)
		.send()?;
	let mut buf = String::new();
	res.read_to_string(&mut buf)?;
	Ok(serde_json::from_str(&buf)?)
}

fn construct_activity_text(
	switch: &PluralKitSwitch,
) -> Result<(String, Option<String>), Box<dyn Error>> {
	let details: String;
	let state: Option<String>;

	match switch.members.len() {
		0 => {
			details = "No one is fronting!".to_owned();
			state = None
		}
		1 => {
			details = format!("{}", switch.members[0].name);
			state = None
		}
		2 => {
			details = format!("{}", switch.members[0].name);
			state = Some(format!("{}", switch.members[1].name))
		}
		3 => {
			details = format!("{}", switch.members[0].name);
			state = Some(format!(
				"{}, {}",
				switch.members[1].name, switch.members[2].name
			))
		}
		4.. => {
			details = format!("{}", switch.members[0].name);
			state = Some(format!(
				"{}, {} (+ {} others)",
				switch.members[1].name,
				switch.members[2].name,
				switch.members.len() - 3
			))
		}
	}

	Ok((details, state))
}
