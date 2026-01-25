use crate::storage;
use dioxus::prelude::*;

#[component]
pub fn list_view() -> Element {
    let storage = storage::get_content();

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
