#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
use config::{Config as Conf, File as ConfFile};

use log::*;
use serde_json::Result;
use simplelog::*;

use std::{time::Duration, rc::Rc};
use std::fs::File;

// Crate files
use twitch_discord_bot::{
    discord::create_discord_bot,
    twitch::{create_twitch_pubsub_ws, setup_twitch_chat_ws},
};

fn main() -> Result<()> {
    // Initialize logger here
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("twitch-discord-bot.log").unwrap()),
        ]
    ).unwrap();

    let mut settings = Conf::default();
    let mut commands = Conf::default();

    settings
        .merge(ConfFile::with_name("config"))
        .expect("Couldn't read or find configuration file");
    // recreating immutable Rc for settings
    let settings = Rc::new(settings);

    commands
        .merge(ConfFile::with_name("commands"))
        .expect("Couldn't read or find commands file");

    // Twitch chat bot creates a connection initially
    let mut twitch_chat_bot = setup_twitch_chat_ws();
    // Twitch pubsub & Discord bot needs to call setup()
    let mut twitch_pubsub_bot = create_twitch_pubsub_ws(settings);
    let mut discord_bot = create_discord_bot();


    twitch_chat_bot.send_listen_msg();
    twitch_pubsub_bot.send_listen_msg();
    discord_bot.setup();

    info!("Starting bot ...");
    std::thread::sleep(Duration::from_millis(200));
    loop {

        discord_bot.main();

        twitch_chat_bot.main(&commands);

        twitch_pubsub_bot.main();
        // MAKE SURE THIS IS IN THE MAIN LOOP
        // YOUR PROCESSOR WILL GO BRRRRRRRRRRRRRRRRRR OTHERWISE
        std::thread::sleep(Duration::from_millis(120));
        // 👇 main loop ends
    }
}
