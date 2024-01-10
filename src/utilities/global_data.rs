use reqwest::Client;
use serenity::{gateway::ShardManager, prelude::TypeMapKey};
use sqlx::SqlitePool;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub struct ShardManagerContainer;
pub struct ReqwestClientContainer;
pub struct GuildSettingsContainer;
pub struct DatabaseConnectionContainer;

pub struct GuildSettings {
    pub prefix: String,
    pub owner_id: u64,
    pub mute_type: String,
    pub mute_role: u64,
    pub default_mute_duration: u64,
}

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

impl TypeMapKey for GuildSettingsContainer {
    type Value = Arc<RwLock<HashMap<u64, GuildSettings>>>;
}

impl TypeMapKey for ReqwestClientContainer {
    type Value = Arc<Client>;
}

impl TypeMapKey for DatabaseConnectionContainer {
    type Value = SqlitePool;
}
