
use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;
use super::{User, Board};
#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct Space{
    id: u32,
    #[tabled(skip)]
    boards: Option<Vec<Board>>,
    title: String,
    #[tabled(skip)]
    users: Option<Vec<User>>
}

impl Space {
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_boards_ids(&self) ->  Vec<u32> {
        self.boards.as_ref().unwrap_or(&vec![]).iter().map(|b| b.get_id()).collect()
    }
    pub fn set_users(&mut self, users: Vec<User>) {
        self.users.replace(users);
    }
    pub fn set_boards(&mut self, boards: Vec<Board>) {
        self.boards.replace(boards);
    }
}
