use std::error::Error;

use crate::pluralkit::PluralKitMember;

pub fn construct_activity_text(
	members: &Vec<PluralKitMember>,
) -> Result<(String, Option<String>), Box<dyn Error>> {
	// TODO: Customisation
	let details: String;
	let state: Option<String>;

	match members.len() {
		0 => {
			details = "No one is fronting!".to_owned();
			state = None
		}
		1 => {
			details = format!("{}", members[0].name);
			state = None
		}
		2 => {
			details = format!("{}", members[0].name);
			state = Some(format!("{}", members[1].name))
		}
		3 => {
			details = format!("{}", members[0].name);
			state = Some(format!("{}, {}", members[1].name, members[2].name))
		}
		4.. => {
			details = format!("{}", members[0].name);
			state = Some(format!(
				"{}, {} (+ {} others)",
				members[1].name,
				members[2].name,
				members.len() - 3
			))
		}
	}

	Ok((details, state))
}