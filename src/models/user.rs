use serde::{Deserialize, Deserializer as De, Serialize, Serializer as Se};
use serde_derive::{Serialize,Deserialize};
use tabled::Tabled;
#[derive(Serialize, Deserialize, Debug, Tabled, Clone)]
pub struct User{
    pub id: u32,
    pub username: String,
    #[tabled(skip)]
    #[serde(default = "Role::default", deserialize_with = "User::deserialize_type", serialize_with = "User::serialize_type")]
    r#type: Role
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Role {
    Empty,
    Responsible(bool)
}

impl Role {
    fn default() -> Self {
        Role::Empty
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.username)
    }
}
impl User {

    fn deserialize_type<'de, D>(deserializer: D) -> Result<Role, D::Error>
    where
        D: De<'de>,
    {
        let value: Option<u8> = Deserialize::deserialize(deserializer)?;
        match value {
            Some(1) => Ok(Role::Responsible(false)),
            Some(2) => Ok(Role::Responsible(true)),
            None => Ok(Role::Empty),
            _ => Err(serde::de::Error::custom("Invalid value for 'type'")),
        }
    }

    fn serialize_type<S>(role: &Role, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Se,
    {
        let value = match role {
            Role::Empty => None,
            Role::Responsible(flag) => {
                if *flag {
                    Some(2)
                } else {
                    Some(1)
                }
            }

        };
        value.serialize(serializer)
    }

    pub fn is_responsible(&self) -> bool {
        match self.r#type {
            Role::Responsible(flag) => flag,
            Role::Empty => false
        }
    }
}
