use crate::github::{PullRequest, PullRequestState, Repository, User};

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
    pub fields: Vec<Field>,
    pub footer: Option<Footer>,
}

#[derive(Debug, serde::Serialize)]
pub struct Message {
    pub content: String,
    pub embeds: Vec<Embed>,
}

fn user_name(user: User) -> String {
    if let Some(name) = &user.name {
        format!("{} ([@{}]({}))", name, user.login, user.html_url)
    } else {
        format!("[@{}]({})", user.login, user.html_url)
    }
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
            fields: vec![
                Field {
                    name: "Author".to_string(),
                    value: user_name(pr.user),
                    inline: false,
                },
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
