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

#[cfg(feature = "web")]
pub fn length() -> u32 {
    LocalStorage::length()
}

#[cfg(not(feature = "web"))]
pub fn get_content() -> HashMap<String, String> {
    HashMap::<String, String>.new();
}

#[cfg(not(feature = "web"))]
pub fn set(key: String, value: String) {}

#[cfg(not(feature = "web"))]
pub fn length() -> u32 {
    0
}
