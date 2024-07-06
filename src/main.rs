#![allow(dead_code)]
#![allow(unused_variables)]

/* POST MVP TODO
 * - Configuration
 * - Loop
 *
*/

use std::{error::Error, io::Read, process::exit};

use discord_rich_presence::{
	activity::Activity,
	DiscordIpc, DiscordIpcClient,
};
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

	let fronters: PluralKitSwitch = get_fronters(request_client, headers)?;
	println!("Found new fronter list: {:?}", fronters);

	// TODO: Customisation
	let activity = construct_activity(&fronters)?;

	println!("Constructed new activity: {:?}", activity);

	match fronters.members.len() {
		0 | 1 => discord_client.set_activity(
			Activity::new()
				.details(activity.0)
		)?,

		// 2 and above
		_ => discord_client.set_activity(
			Activity::new()
				.details(activity.0)
				.state(activity.1.as_str()),
		)?,
	}

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

fn construct_activity(fronters: &PluralKitSwitch) -> Result<(&str, String), Box<dyn Error>> {
	let mut iter = fronters.members.iter();
	let details: &str;
	let mut state = String::new();
	match iter.next() {
		Some(fronter) => details = fronter.name.as_str(),
		None => details = "There's no one fronting.",
	};
	match iter.next() {
		Some(fronter) => {
			state += fronter.name.as_str();
		}
		None => (),
	};
	match iter.next() {
		Some(fronter) => {
			state = state + ", " + fronter.name.as_str();
		}
		None => (),
	};
	match iter.next() {
		Some(fronter) => {
			state = state + " (+" + { iter.count() + 1 }.to_string().as_str() + " more)";
		}
		None => (),
	};

	Ok((details, state))
}
