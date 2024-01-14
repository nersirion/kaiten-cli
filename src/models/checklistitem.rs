use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub id: Option<u32>,
    pub text: String,
    pub checked: bool
}

impl std::fmt::Display for ChecklistItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ChecklistItem {
    fn to_string(&self) -> String {
        let check = if self.checked { "[x]"} else {"[ ]"};
        let string_item = format!("{} {}", check, self.text);
        string_item

    }
    pub fn from_string(raw_text: String) -> Self {
        let text = raw_text.trim();
        let text = text.replace("[]", "[ ]");
        let check = match &text[0..3] {
            "[ ]" => false,
            "[x]" => true,
            _ => panic!("Text will be start with [ ] or [x]")
        };
        Self {
            id: None,
            text: if text.len()> 4 {text[4..].to_string()} else {"".to_string()},
            checked: check
        }
    }
}


