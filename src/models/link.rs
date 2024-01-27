use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;
#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct Link{
    #[serde(skip_serializing)]
    id: u32,
    #[tabled(display_with="Self::display_option_string")]
    url: Option<String>,
    #[tabled(display_with="Self::display_option_string")]
    description: Option<String>
}

impl Link {
    pub fn from(url: String, description: Option<String>) -> Self {
        Link {
            id: 0,
            url: Some(url),
            description
        }

    }
    fn display_option_string(s: &Option<String>) -> String {
        match s {
            Some(s) => {
                format!("{}", s)
            }
            None => format!(""),
        }
    }
}
