use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use std::collections::HashMap;

#[component]
pub fn list_view() -> Element {
    rsx! {
         Header {  }
         ListApp {  }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        div { id: "title_div",
            h1 { id: "title",
                "V_inder"
            }
        }
    }
}

#[component]
fn ListApp() -> Element {
    let storage = LocalStorage::get_all::<HashMap<String, String>>().unwrap_or_default();

    rsx! {
        div { id: "names_container",
            ul {
                for i in 0..storage.len() {
                    li { class: "name_element", "{storage.get(&i.to_string()).unwrap()}" }
                }
            }
        }
    }
}
