use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChecklistItem {
    id: u32,
    text: String,
    checked: bool
}

impl std::fmt::Display for ChecklistItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let check = if self.checked { "[x]"} else {"[ ]"};
        write!(f, "{} {}", check, self.text)
    }
}

impl ChecklistItem {
    pub fn from_string(raw_text: String) -> Self {
        let text = raw_text.trim();
        let text = text.replace("[]", "[ ]");
        let check = match &text[0..3] {
            "[ ]" => false,
            "[x]" => true,
            _ => panic!("Text will be start with [ ] or [x]")
        };
        Self {
            id: 0,
            text: if text.len()> 4 {text[4..].to_string()} else {"".to_string()},
            checked: check
        }
    }
}


