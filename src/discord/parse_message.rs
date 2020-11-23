use crate::discord::DiscordBot;
use log::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;
pub struct Disconnected;
pub type Result<T> = std::result::Result<T, Disconnected>;

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordRolesObj {
    id: String,
    name: String,
    color: i64,
    hoist: bool,
    position: i64,
    permissions: String,
    managed: bool,
    mentionable: bool,
}

impl Default for DiscordUserObj {
    fn default() -> Self {
        Self {
            username: "".to_string(),
            discriminator: "".to_string(),
            id: "".to_string(),
            avatar: Some("".to_string()),
            bot: false,
            system: false,
            mfa_enabled: false,
            locale: "".to_string(),
            verified: false,
            email: Some("".to_string()),
            flags: 0,
            premium_type: 0,
            public_flags: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordGuildMember {
    #[serde(default)]
    user: DiscordUserObj,
    nick: Option<String>,
    roles: Vec<String>,
    joined_at: String,
    premium_since: Option<String>,
    deaf: bool,
    mute: bool,
}

impl Default for DiscordGuildMember {
    fn default() -> Self {
        Self {
            user: DiscordUserObj::default(),
            nick: Some("".to_string()),
            roles: vec!["".to_string()],
            joined_at: "".to_string(),
            premium_since: Some("".to_string()),
            deaf: false,
            mute: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordUserObj {
    username: String,
    discriminator: String,
    id: String,
    avatar: Option<String>,
    #[serde(default)]
    bot: bool,
    #[serde(default)]
    system: bool,
    #[serde(default)]
    mfa_enabled: bool,
    #[serde(default)]
    locale: String,
    #[serde(default)]
    verified: bool,
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    flags: i64,
    #[serde(default)]
    premium_type: i64,
    #[serde(default)]
    public_flags: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordVoiceState {
    #[serde(default)]
    guild_id: String,
    channel_id: Option<String>,
    user_id: String,
    #[serde(default)]
    member: DiscordGuildMember,
    session_id: String,
    deaf: bool,
    mute: bool,
    self_deaf: bool,
    self_mute: bool,
    #[serde(default)]
    self_stream: bool,
    self_video: bool,
    #[serde(default)]
    supress: bool,
}

impl Default for DiscordVoiceState {
    fn default() -> Self {
        Self {
            guild_id: "".to_string(),
            channel_id: Some("".to_string()),
            user_id: "".to_string(),
            mute: false,
            deaf: false,
            supress: false,
            self_video: false,
            self_stream: false,
            self_mute: false,
            self_deaf: false,
            session_id: "".to_string(),
            member: DiscordGuildMember::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGuildEmoji {
    id: String,
    name: String,
    #[serde(default)]
    roles: Option<Vec<String>>,
    #[serde(default)]
    user: DiscordUserObj,
    #[serde(default)]
    require_colons: bool,
    #[serde(default)]
    managed: bool,
    #[serde(default)]
    animated: bool,
    available: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGuild {
    op: i64,
    s: i64,
    d: CreateGuildData,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGuildChannelPermissionOverwrites {
    allow: String,
    deny: String,
    id: String,
    #[serde(rename = "type")]
    t: i64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGuildChannels {
    id: String,
    #[serde(default)]
    #[serde(rename = "type")]
    channel_type: i64,
    #[serde(default)]
    guild_id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    position: i64,
    #[serde(default)]
    permission_overwrites: Vec<CreateGuildChannelPermissionOverwrites>,
    #[serde(default)]
    rate_limit_per_user: i64,
    nsfw: bool,
    #[serde(default)]
    topic: Option<String>,
    #[serde(default)]
    last_message_id: Option<String>,
    #[serde(default)]
    user_limit: i64,
    #[serde(default)]
    parent_id: Option<String>,
    #[serde(default)]
    bitrate: i64,
    #[serde(default)]
    recipients: Vec<DiscordUserObj>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGuildData {
    id: String,
    name: String,
    icon: Option<String>,
    #[serde(default)]
    icon_hash: Option<String>,
    splash: Option<String>,
    discovery_splash: Option<String>,
    #[serde(default)]
    owner: bool,
    #[serde(default)]
    owner_id: String,
    region: String,
    #[serde(default)]
    afk_channel_id: Option<String>,
    afk_timeout: i64,
    #[serde(default)]
    widget_enabled: bool,
    #[serde(default)]
    widget_channel_id: Option<String>,
    verification_level: i64,
    default_message_notifications: i64,
    explicit_content_filter: i64,
    roles: Vec<DiscordRolesObj>,
    emojis: Vec<CreateGuildEmoji>,
    features: Vec<String>,
    mfa_level: i64,
    #[serde(default)]
    application_id: Option<String>,
    #[serde(default)]
    system_channel_id: Option<String>,
    system_channel_flags: i64,
    #[serde(default)]
    rules_channel_id: Option<String>,
    #[serde(default)]
    joined_at: String,
    large: bool,
    unavailable: bool,
    member_count: i64,
    #[serde(default)]
    voice_states: Vec<DiscordVoiceState>,
    #[serde(default)]
    members: Vec<DiscordGuildMember>,
    #[serde(default)]
    max_presences: Option<i64>,
    #[serde(default)]
    max_members: i64,
    vanity_url_code: Option<String>,
    #[serde(default)]
    description: Option<String>,
    banner: Option<String>,
    premium_tier: i64,
    premium_subscription_count: i64,
    preferred_locale: String,
    public_updates_channel_id: Option<String>,
    #[serde(default)]
    max_video_channel_users: i64,
    #[serde(default)]
    approximate_member_count: i64,
    #[serde(default)]
    approximate_presence_count: i64,
}

// pub struct DiscordMetaMsg {
//    op: i64,
//    s: i64,
// }

#[derive(Deserialize, Serialize, Debug)]
pub struct HeatBeatMessage {
    pub op: u16,
    pub d: Option<i16>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HeartBeatData {
    pub heartbeat_interval: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DiscordHeatBeat {
    pub op: u8,
    pub d: HeartBeatData,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DiscordReadyUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    #[serde(default)]
    pub bot: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DiscordReadyGuilds {
    pub unavailable: bool,
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DiscordReadyData {
    v: i64,
    user: DiscordReadyUser,
    private_channels: Vec<String>,
    guilds: Vec<DiscordReadyGuilds>,
    session_id: String,
    #[serde(default)]
    shard: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DiscordReadyRes {
    t: String,
    d: DiscordReadyData,
}

#[derive(Deserialize, Serialize, Debug)]
struct InvalidSession {
    op: i64,
    d: bool,
}

pub struct DiscordMsgParser {}
impl DiscordMsgParser {
    pub fn parse(bot: &mut DiscordBot, msg: &str) -> Result<()> {
        if msg.contains(r#""op":9"#) {
            let invalid: InvalidSession = serde_json::from_str(msg)
                .expect("Could not deserialize Discord invalid session message");
            if invalid.d {
                info!("Dicord session is resumable");
                return bot.resume();
            } else {
                return Err(Disconnected);
            }
        }

        if msg.contains(r#""t":"READY""#) {
            let ready_msg: DiscordReadyRes =
                serde_json::from_str(msg).expect("Could not deserialize Discord ready response");

            // println!("{:#?}", ready_msg.d);

            bot.session_id = ready_msg.d.session_id;
            return Ok(());
        }

        if msg.contains(r#""t":"GUILD_CREATE""#) {
            let create_guild: CreateGuild =
                serde_json::from_str(msg).expect("Could not deserialize Discord Guild Create msg");

            // println!("{:#?}", create_guild);
            bot.last_sequence = Some(create_guild.s);
            return Ok(());
        };

        if msg.contains(r#""t":"MESSAGE_CREATE""#) {
           info!("{:#?}", msg);

            return Ok(());
        };

        if msg.contains(r#""op":11"#) {
            info!("Discord's heart beats");

            return Ok(());
        };

        if msg.contains(r#""op":10"#) {
            let hb_msg: DiscordHeatBeat = serde_json::from_str(msg.trim())
                .expect("Discord Heartbeat could not be deserialized");

            bot.heartbeat_interval = Duration::from_millis(hb_msg.d.heartbeat_interval);

            info!("Sending heartbeat after receiving heartbeart");

            return bot.send_heartbeat();
        }

        println!("I am not parsing a Discord message: {}", msg);
        return Err(Disconnected);
    }
    // pub fn create_message(msg: &str) {
    //     //      let create_message: CreateMessage = serde_json::from_str(msg).expect("Could not deserialize Create Message msg");
    // }
}
