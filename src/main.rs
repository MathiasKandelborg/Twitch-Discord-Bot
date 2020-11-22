#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
use config::{Config, File};

use serde_json::Result;

use std::time::Duration;

// Crate files
use twitch_discord_bot::{
    discord::create_discord_bot,
    twitch::{create_twitch_pubsub_ws, setup_twitch_chat_ws},
};

fn main() -> Result<()> {
    env_logger::init();

    let mut settings = Config::default();
    let mut commands = Config::default();

    settings
        .merge(File::with_name("config"))
        .expect("Couldn't read or find configuration file");

    commands
        .merge(File::with_name("commands"))
        .expect("Couldn't read or find commands file");

    // Twitch chat bot creates a connection initially
    let mut twitch_chat_bot = setup_twitch_chat_ws();
    // Twitch pubsub & Discord bot needs to call setup()
    let mut twitch_pubsub_bot = create_twitch_pubsub_ws();
    let mut discord_bot = create_discord_bot();


    twitch_chat_bot.send_listen_msg();
    twitch_pubsub_bot.send_listen_msg();
    discord_bot.setup();

    std::thread::sleep(Duration::from_millis(200));
    loop {

        discord_bot.main();

        twitch_chat_bot.main(&commands, &settings);

        twitch_pubsub_bot.main(&settings);
        // MAKE SURE THIS IS IN THE MAIN LOOP
        // YOUR PROCESSOR WILL GO BRRRRRRRRRRRRRRRRRR OTHERWISE
        std::thread::sleep(Duration::from_millis(120));
        // 👇 main loop ends
    }
}
