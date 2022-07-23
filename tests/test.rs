use kaiten_cli::kaiten::{Checklist, ChecklistItem};
use std::path::Path;
use std::fs::File;

#[test]
fn test_checklists_from_json() {
    let json_file_path = Path::new("./tests/test_files/checklist.json");
    let file = File::open(json_file_path).unwrap();
    let checklist: Checklist = serde_json::from_reader(file).unwrap();
    assert_eq!(checklist.name, "Готовность".to_string());
    assert_eq!(checklist.id, Some(1032486));
    assert!(checklist.items.is_some());
    assert_eq!(checklist.items.unwrap().len(), 4);
}

#[test]
fn test_checklists() {
    let json_file_path = Path::new("./tests/test_files/checklist.json");
    let file = File::open(json_file_path).unwrap();
    let checklist: Checklist = serde_json::from_reader(file).unwrap();
    let test_checklist_string = concat!(
        "### Готовность\n\n",
      "[ ] Новые креды админа лежат в lockbox\n",
      "[ ] Логин в админский аккаунт по новым кредам\n",
      "[ ] У сервиса графаны внешний ip\n",
      "[ ] Из кред обновлен secret grafana",
).to_string();
    assert_eq!(checklist.to_string(), test_checklist_string);
    let test_checklist = Checklist::from_string(test_checklist_string);
    assert_eq!(checklist.name, test_checklist.name);

}

#[test]
fn test_checklistsitem() {
    let checklistitem = ChecklistItem {
        id: Some(1),
        text: "Some text".to_string(),
        checked: false
    };
    let check_string = String::from("[ ] Some text");
    assert_eq!(checklistitem.to_string(), check_string);
    let check_item = ChecklistItem::from_string(check_string);
    assert_eq!(checklistitem.text, check_item.text);
    assert_eq!(checklistitem.checked, check_item.checked);
    assert_eq!(check_item.id, None);
}

#[test]
#[should_panic]
fn test_checklistsitem_fail() {
    let item = ChecklistItem::from_string("Bad string".to_string());
}
