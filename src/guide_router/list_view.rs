use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

use super::match_class;

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
    rsx! {
        p { "list_view" }
    }
}
