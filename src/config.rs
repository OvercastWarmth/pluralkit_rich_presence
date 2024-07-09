#![allow(dead_code)]
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
	pluralkit: PluralKit,
	advanced: Advanced,
}

#[derive(Deserialize)]
struct PluralKit {
	system_id: Option<String>,
	token: Option<String>,
} 

#[derive(Deserialize)]
struct Advanced {
	discord_application_id: Option<String>,
}
