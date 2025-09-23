use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    id: String,
    #[serde(rename="type")]
    event_type: EventType,
    actor: Actor,
    repo: Repository,
    payload: Payload,
    public: bool,
    created_at: DateTime<Utc>
}

impl Event {
    pub fn event_message(&self) -> String {
        match self.event_type {
            EventType::Create => format!("- Create repository {}", self.repo.name),
            EventType::Push => format!("- Pushed {} commits to {}", self.payload.commits.iter().count(), self.repo.name),
            EventType::PullRequest => {
                if let (Some(action), Some(number)) = (self.payload.action.as_ref(), self.payload.number) {
                    format!("- {} pull request {} on {}", action.message(), number, self.repo.name)
                } else {
                    format!("- Unknown update occurred on {}", self.repo.name)
                }
            }
            EventType::Delete => {
                if let (Some(payload_ref), Some(ref_type)) = (self.payload.payload_ref.as_ref(), self.payload.ref_type.as_ref()) {
                    format!("- Delete {} {} on {}", ref_type, payload_ref, self.repo.name)
                } else {
                    format!("- Unknown delete event occurred on {}", self.repo.name)
                }
            },
            EventType::Issue => {
                if let Some(action) = self.payload.action.as_ref() {
                    format!("- {} issue on {}", action.message(), self.repo.name)
                } else {
                    format!("- Unknown issue event occurred on {}", self.repo.name)
                }
            }
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub enum EventType {
    #[serde(rename="PushEvent")]
    Push,
    #[serde(rename="CreateEvent")]
    Create,
    #[serde(rename="PullRequestEvent")]
    PullRequest,
    #[serde(rename="IssuesEvent")]
    Issue,
    #[serde(rename="DeleteEvent")]
    Delete

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actor {
    id: u32,
    login: String,
    display_login: String,
    gravatar_id: String,
    url: String,
    avatar_url: String
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repository {
    id: u32,
    name: String,
    url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    repository_id: Option<u32>,
    push_id: Option<u64>,
    size: Option<u32>,
    distinct_size: Option<u32>,
    #[serde(rename="ref")]
    payload_ref: Option<String>,
    ref_type: Option<String>,
    head: Option<String>,
    before: Option<String>,
    commits: Option<Vec<Commit>>,
    action: Option<Action>,
    number: Option<u32>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    #[serde(rename="opened")]
    Opened,
    #[serde(rename="closed")]
    Closed,
}

impl Action {
    pub fn message(&self) -> &str {
        match self {
            Action::Opened => "Opened",
            Action::Closed => "Closed"
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commit {
    sha: String,
    author: Author,
    message: String,
    distinct: bool,
    url: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Author {
    email: String,
    name: String
}