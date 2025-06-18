use github::Event;
use hmac::Mac;
use worker::*;

use crate::{
    discord::{Embed, Message},
    github::PullRequestAction,
};

mod config;
mod discord;
mod github;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}]",
        Date::now().to_string(),
        req.path()
    );
}

#[event(fetch)]
async fn main(mut req: Request, env: Env, _ctx: Context) -> Result<Response> {
    log_request(&req);

    // Configuration
    let config = match config::AppConfig::try_from(env) {
        Ok(config) => config,
        Err(e) => {
            console_log!("error loading configuration: {}", e);
            return Response::error("invalid configuration", 500);
        }
    };

    let Ok(bytes) = req.bytes().await else {
        return Response::error("invalid body", 400);
    };

    // Signature verification
    let Ok(Some(sig_header)) = req.headers().get("X-Hub-Signature-256") else {
        return Response::error("missing signature", 400);
    };
    let signature = match hex::decode(sig_header.trim_start_matches("sha256=")) {
        Ok(signature) => signature,
        Err(e) => return Response::error(format!("invalid signature: {e}"), 400),
    };
    let mut mac = hmac::Hmac::<sha2::Sha256>::new_from_slice(config.github.secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(&bytes);
    if let Err(e) = mac.verify_slice(&signature) {
        return Response::error(format!("invalid signature: {e}"), 401);
    }

    // Payload parsing
    let Ok(payload) = serde_json::from_slice(&bytes) else {
        return Response::error("invalid payload", 400);
    };
    let Ok(Some(event_type)) = req.headers().get("X-GitHub-Event") else {
        return Response::error("missing event type", 400);
    };

    // Event handling
    match Event::from_payload(event_type, payload) {
        Ok(event) => handle_event(event, &config).await,
        Err(e) => Response::ok(format!("unsupported event: {e}")),
    }
}

async fn handle_event(event: Event, config: &config::AppConfig) -> Result<Response> {
    console_log!("Received event");
    match event {
        Event::Ping { hook_id, zen } => console_log!("Received ping for {}, zen: {}", hook_id, zen),
        Event::PullRequest {
            action,
            sender,
            pull_request,
            requested_team,
            requested_reviewer,
            repository,
        } => {
            let name = sender.name.unwrap_or(format!("`{}`", sender.login));
            let message = match action {
                PullRequestAction::Opened => Message {
                    content: format!("{} opened a pull request", name),
                    embeds: vec![Embed::from_pr(pull_request, repository)],
                },
                PullRequestAction::Closed => {
                    let verb = match pull_request.merged {
                        Some(true) => "merged",
                        _ => "closed",
                    };
                    Message {
                        content: format!("{} {} a pull request", name, verb),
                        embeds: vec![Embed::from_pr(pull_request, repository)],
                    }
                }
                PullRequestAction::Reopened => Message {
                    content: format!("{} reopened a pull request", name),
                    embeds: vec![Embed::from_pr(pull_request, repository)],
                },
                PullRequestAction::ReviewRequested => {
                    if let Some(ping) = match (requested_reviewer, requested_team) {
                        (Some(reviewer), _) => config
                            .discord
                            .user_ids
                            .get(&reviewer.login)
                            .map(|i| format!("<@{}>", i))
                            .or(Some(
                                reviewer
                                    .name
                                    .map(|n| format!("{} (`{}`)", n, reviewer.login))
                                    .unwrap_or(format!("`{}`", reviewer.login)),
                            )),
                        (_, Some(team)) => config
                            .discord
                            .role_ids
                            .get(&team.slug)
                            .map(|i| format!("<@&{}>", i)),
                        _ => None,
                    } {
                        Message {
                            content: format!("{} requested review from {}", name, ping),
                            embeds: vec![Embed::from_pr(pull_request, repository)],
                        }
                    } else {
                        return Response::ok("no ping configured for requested reviewer");
                    }
                }
                _ => {
                    return Response::ok("unsupported pull request event type");
                }
            };
            if let Err(e) = discord::send_message(&config.discord.webhook_url, message).await {
                return Response::error(format!("error sending discord webhook: {}", e), 500);
            }
        }
    }
    Response::ok("event handled successfully")
}
