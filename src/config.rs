use std::collections::HashMap;

use worker::Env;

#[derive(Debug)]
pub struct GithubConfig {
    pub secret: String,
    pub team: Option<String>,
}

#[derive(Debug)]
pub struct DiscordConfig {
    pub webhook_url: String,
    pub user_ids: HashMap<String, String>,
    pub role_ids: HashMap<String, String>,
}

#[derive(Debug)]
pub struct AppConfig {
    pub github: GithubConfig,
    pub discord: DiscordConfig,
}

impl TryFrom<Env> for AppConfig {
    type Error = worker::Error;

    fn try_from(env: Env) -> worker::Result<Self> {
        let user_ids = env.var("DC_USER_IDS")?;
        let user_ids = serde_json::from_str(&user_ids.to_string())?;
        let role_ids = env.var("DC_ROLE_IDS")?;
        let role_ids = serde_json::from_str(&role_ids.to_string())?;
        Ok(Self {
            github: GithubConfig {
                secret: env.secret("GH_SECRET")?.to_string(),
                team: env.var("GH_REVIEWER_TEAM").ok().map(|t| t.to_string()),
            },
            discord: DiscordConfig {
                webhook_url: env.secret("WEBHOOK_URL")?.to_string(),
                user_ids,
                role_ids,
            },
        })
    }
}
