use crate::github::{PullRequest, PullRequestState, Repository};

#[derive(Debug, serde::Serialize)]
pub struct Author {
    pub name: String,
    pub url: String,
    pub icon_url: String,
}

#[derive(Debug, serde::Serialize)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Debug, serde::Serialize)]
pub struct Footer {
    pub text: String,
    pub icon_url: String,
}

#[derive(Debug, serde::Serialize)]
pub struct Embed {
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub color: u32,
    pub author: Author,
    pub fields: Vec<Field>,
    pub footer: Option<Footer>,
}

#[derive(Debug, serde::Serialize)]
pub struct Message {
    pub content: String,
    pub embeds: Vec<Embed>,
}

impl Embed {
    pub fn from_pr(pr: PullRequest, repo: Repository) -> Self {
        let color = match (pr.state, pr.merged) {
            (_, Some(true)) => 0x8957e5,
            (PullRequestState::Open, _) if pr.draft => 0x6e7681,
            (PullRequestState::Open, _) => 0x238636,
            (PullRequestState::Closed, _) => 0xda3633,
        };
        Self {
            title: format!("{} #{}", pr.title, pr.number),
            description: pr.body,
            url: pr.html_url,
            color,
            author: Author {
                name: pr
                    .user
                    .name
                    .map(|n| format!("{} ({})", n, pr.user.login))
                    .unwrap_or(pr.user.login),
                url: pr.user.html_url,
                icon_url: pr.user.avatar_url,
            },
            fields: vec![
                Field {
                    name: "Additions".to_string(),
                    value: format!("**`+{}`**", pr.additions),
                    inline: true,
                },
                Field {
                    name: "Deletions".to_string(),
                    value: format!("**`-{}`**", pr.deletions),
                    inline: true,
                },
            ],
            footer: Some(Footer {
                text: repo.full_name,
                icon_url: repo.owner.avatar_url,
            }),
        }
    }
}

pub async fn send_message(url: &str, message: Message) -> reqwest::Result<()> {
    reqwest::Client::new()
        .post(url)
        .json(&message)
        .send()
        .await?;
    Ok(())
}
