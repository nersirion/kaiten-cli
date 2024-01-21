use super::{Board, User};
use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;
#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct Space {
    id: u32,
    #[tabled(skip)]
    boards: Option<Vec<Board>>,
    title: String,
    #[tabled(skip)]
    users: Option<Vec<User>>,
}

impl Space {
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_boards_ids(&self) -> Vec<u32> {
        self.boards
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .map(|b| b.get_id())
            .collect()
    }
    pub fn set_users(&mut self, users: Vec<User>) {
        self.users.replace(users);
    }
    pub fn get_users(&self) -> Vec<User> {
        self.users.as_ref().cloned().unwrap_or(Vec::new())
    }
}
