use std::{error::Error, io::Read};

use reqwest::header::HeaderMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PluralKitMember {
	pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct PluralKitSwitch {
	pub members: Vec<PluralKitMember>,
}

pub fn get_fronters(
	request_client: &reqwest::blocking::Client,
	headers: &HeaderMap,
) -> Result<PluralKitSwitch, Box<dyn Error>> {
	let mut res = request_client
		.get("https://api.pluralkit.me/v2/systems/vgspl/fronters") // TODO: Unhardcode this later pls :3
		.headers(headers.clone())
		.send()?;
	let mut buf = String::new();
	res.read_to_string(&mut buf)?;
	Ok(serde_json::from_str(&buf)?)
}
