
use std::process::Command;
use std::{time,thread};
use chrono::prelude::*;

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
    commands: [minecraft],
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
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP));

    // Stoppt den Minecraft-Server um 4 Uhr am Morgen
    thread::spawn(move || {
        loop {
            let mut c = Command::new("docker");
            let c = c.arg("ps").arg("-a");
            let output = String::from_utf8(c.output().unwrap().stdout).unwrap();
            let local: DateTime<Local> = Local::now();

            if output.contains("minecraft") && local.time().format("%H").to_string() == "04" && local.time().format("%M").to_string() == "00" {
                let mut c = Command::new("docker");
                c.arg("-f /home/titan/minecraft-tim/docker-compose.yml").arg("down").output()
                    .expect("Fehler beim Stoppen des Minecraft-Servers!");
            }
            thread::sleep(time::Duration::from_secs(30));
        }
    });

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn minecraft(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut c = Command::new("docker");
    let c = c.arg("ps").arg("-a");
    let output = String::from_utf8(c.output().unwrap().stdout).unwrap();

    if output.contains("minecraft") {
        msg.reply(ctx, "Der Minecraft-Server sollte laufen. Bitte führe dieses Kommando nur aus, falls der Server nicht läuft.")?;
    } else {
        msg.reply(ctx, "Der Minecraft-Server wird gestartet, bitte warte einen Augenblick! ")?;
        let mut c = Command::new("docker-compose");
        c.arg("-f /home/titan/minecraft-tim/docker-compose.yml").arg("up").arg("-d").output()
            .expect("Fehler beim Starten des Minecraft-Servers!");
        println!("Minecraft-Server gestartet!");
    }
    
    
    Ok(())
}
