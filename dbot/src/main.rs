
use std::process::Command;

use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

group!({
    name: "general",
    options: {},
    commands: [ping],
});

use std::env;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    // Login with a bot token from the environment
    let token = env::var("TOKEN").expect("token");
    println!("T: {}", token);
    let mut client = Client::new(&token, Handler)
        .expect("Error creating client");


    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;
    let mut c = Command::new("docker");
    let c = c.arg("ps").arg("-a");
    let output = String::from_utf8(c.output().unwrap().stdout).unwrap();
    println!("{}", output.contains("minecraft"));
    Ok(())
}
