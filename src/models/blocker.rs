use super::{RelatedCard, User};
use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug, Tabled, Clone)]
pub struct Blocker {
    id: u32,
    #[tabled(skip)]
    reason: Option<String>,
    #[tabled(display_with = "Self::display_user")]
    blocker: Option<User>,
    released: bool,
    created: String,
    updated: String,
    #[tabled(skip)]
    #[serde(alias = "blocked_card")]
    card: Option<RelatedCard>,
}

impl Blocker {
    pub fn get_reason(&self) -> &str {
        if let Some(reason) = &self.reason {
            reason.as_str()
        } else {
            ""
        }
    }
    pub fn get_card(&self) -> Option<RelatedCard> {
        self.card.clone()
    }

    fn display_user(u: &Option<User>) -> String {
        match u {
            Some(user) => format!("{}", user.get_username()),
            None => format!(""),
        }
    }
}
