use std::collections::HashMap;

#[cfg(feature = "web")]
use gloo_storage::{LocalStorage, Storage};

#[cfg(feature = "web")]
pub fn get_content() -> HashMap<String, String> {
    LocalStorage::get_all().unwrap_or_default()
}

#[cfg(feature = "web")]
pub fn set(key: String, value: String) {
    LocalStorage::set(key, value).unwrap();
}

#[cfg(not(feature = "web"))]
pub fn get_content() -> HashMap<String, String> {
    serde_jsonlines::json_lines("v_inder.jsonl")
        .unwrap()
        .map(|a| a.unwrap())
        .collect()
}

#[cfg(not(feature = "web"))]
pub fn set(key: String, value: String) {
    //if !Path::new("v_inder.jsonl").exists() {
    //    File::create("v_inder.jsonl").expect("Couldn't create jsonl to save names");
    //}
    serde_jsonlines::append_json_lines("v_inder.jsonl", [(key, value)]).unwrap();
}
