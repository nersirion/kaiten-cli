use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;
use crate::models::User;

#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct Comment {
    #[tabled(skip)]
    id: u32,
    text: String,
    #[tabled(skip)]
    created: String,
    user: User
}
