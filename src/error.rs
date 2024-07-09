use thiserror::Error;

#[derive(Error, Debug)]
pub enum RichPresenceError<E> {
	#[error("error while connecting to your Discord client: {0}")]
	InitialDiscordConnection(E),
	#[error("error while fetching the latest switch: {0}")]
	PluralKitSwitchFetch(E),
	#[error("error while sending rich presence to Discord: {0}")]
	SendActivityData(E),
}
