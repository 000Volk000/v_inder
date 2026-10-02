use crate::storage;
use dioxus::prelude::*;

#[component]
pub fn list_view() -> Element {
    let names = use_resource(storage::load);

    rsx! {
        div { id: "names_container",
            match &*names.read() {
                Some(list) => rsx! {
                    ul {
                        for name in list.iter() {
                            li { class: "name_element", "{name}" }
                        }
                    }
                },
                None => rsx! { p { "Cargando..." } },
            }
        }
    }
}
