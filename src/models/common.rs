use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;
use crate::models::Column;

lazy_static! {
    pub static ref COLUMNS: Mutex<HashMap<String, Column>> = Mutex::new(HashMap::new());
    pub static ref USERS: Mutex<HashMap<String, u32>> = Mutex::new(HashMap::new());
    
}

