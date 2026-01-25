use dioxus::prelude::*;

#[component]
pub fn header() -> Element {
    rsx! {
        div { id: "title_div",
            h1 { id: "title",
                "V_inder"
            }
        }
    }
}
