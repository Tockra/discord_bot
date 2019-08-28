use discord::Discord;
use discord::model::Event;
use std::env;

fn main() {
	// Log in to Discord using a bot token from the environment
	let discord = Discord::from_bot_token(
		&env::var("TOKEN").expect("Expected token"),
	).expect("login failed");

	// Establish and use a websocket connection
	let (mut connection, _) = discord.connect().expect("connect failed");
	println!("Ready.");
	loop {
		match connection.recv_event() {
			Ok(Event::MessageCreate(message)) => {
				println!("{} says: {}", message.author.name, message.content);
				if message.content == "!minecraft" {
					discord.send_message(message.channel_id, "Der Minecraft-Server wird gestartet, bitte warte einen Augenblick", "", false).unwrap();

                    discord.send_message(message.channel_id, "Der Minecraft-Server läuft bereits. Ab 4 Uhr nachts kannst du den Befehl wieder verwenden!", "", false).unwrap();
				} 
			}
			Ok(_) => {}
			Err(discord::Error::Closed(code, body)) => {
				println!("Gateway closed on us with code {:?}: {}", code, body);
				break
			}
			Err(err) => println!("Receive error: {:?}", err)
		}
	}
}