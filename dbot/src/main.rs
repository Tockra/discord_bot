use std::process::Command;
use std::thread;
use chrono::prelude::*;
use std::time::Duration;

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

#[group]
#[commands(minecraft,help, hilfe)]
struct General;

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
            let duration = get_sleep_duration_until4am();
            println!("Schlafe: {:?} Sekunden", duration);
            thread::sleep(duration);

            let mut c = Command::new("docker");
            let c = c.arg("ps").arg("-a");
            let output = String::from_utf8(c.output().unwrap().stdout).unwrap();

            if output.contains("minecraft") {
                let mut c = Command::new("docker-compose");
                c.arg("-f").arg("/home/titan/minecraft-tim/docker-compose.yml").arg("down").output()
                    .expect("Fehler beim Stoppen des Minecraft-Servers!");
            }
        }
    });

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

fn get_sleep_duration_until4am() -> Duration {
    let now = Local::now();
    let tomorrow_midnight = (now + chrono::Duration::days(1)).date().and_hms(0, 0, 0);

    tomorrow_midnight.signed_duration_since(now).to_std().unwrap()
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
        c.arg("-f").arg("/home/titan/minecraft-tim/docker-compose.yml").arg("up").arg("-d").output()
            .expect("Fehler beim Starten des Minecraft-Servers!");
        println!("Minecraft-Server gestartet");
    }
    
    
    Ok(())
}

#[command]
fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    help_main(ctx,msg)
}

#[command]
fn hilfe(ctx: &mut Context, msg: &Message) -> CommandResult {
    help_main(ctx, msg)
}

fn help_main(ctx: &mut Context, msg: &Message) -> CommandResult {
    let server_ip = env::var("SERVER_IP").expect("server_ip");
    
    msg.reply(ctx, format!("Gebe !minecraft ein um den Minecraft-Server zu starten. Der Server wird jede Nacht um 4 Uhr ausgeschaltet und muss anschließend erneut gestartet werden.\n Der Server ist über die IP: {} erreichbar!", server_ip))?;
    Ok(())
}
