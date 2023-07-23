#[derive(Debug, serde::Deserialize)]
pub struct User {
    pub login: String,
    pub name: Option<String>,
    pub html_url: String,
    pub avatar_url: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Team {
    pub slug: String,
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Repository {
    pub full_name: String,
    pub html_url: String,
    pub owner: User,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestAction {
    Opened,
    Edited,
    Closed,
    Reopened,
    Assigned,
    Unassigned,
    ReviewRequested,
    ReviewRequestRemoved,
    Labeled,
    Unlabeled,
    Synchronize,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestState {
    Open,
    Closed,
}

#[derive(Debug, serde::Deserialize)]
pub struct PullRequest {
    pub number: u64,
    pub html_url: String,
    pub title: String,
    pub state: PullRequestState,
    pub user: User,
    pub body: Option<String>,
    pub draft: bool,
    pub merged: Option<bool>,
    pub additions: u64,
    pub deletions: u64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "payload")]
#[allow(clippy::large_enum_variant)]
pub enum Event {
    Ping {
        hook_id: u64,
        zen: String,
    },
    PullRequest {
        action: PullRequestAction,
        sender: User,
        pull_request: PullRequest,
        requested_team: Option<Team>,
        requested_reviewer: Option<User>,
        repository: Repository,
    },
}

impl Event {
    pub fn from_payload(
        event_type: String,
        payload: serde_json::Value,
    ) -> serde_json::Result<Self> {
        let mut object = serde_json::Map::new();
        object.insert("type".to_string(), serde_json::Value::String(event_type));
        object.insert("payload".to_string(), payload);
        serde_json::from_value::<Event>(serde_json::Value::Object(object))
    }
}
