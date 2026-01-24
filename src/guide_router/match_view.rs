use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

use super::match_class;

#[component]
pub fn match_view() -> Element {
    rsx! {
         Header {  }
         NamesApp {  }
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
fn NamesApp() -> Element {
    let mut match_component = match_class::Match::new();

    let good = move |_| {
        LocalStorage::set(
            LocalStorage::length().to_string(),
            &*match_component.get_name().read(),
        )
        .unwrap();
        // {LocalStorage::get::<String>(\"name\").unwrap()}
        match_component.next();
    };
    let bad = move |_| {
        match_component.next();
    };

    rsx! {
        div { id: "match",
            img { id: "central_image",
                src: "{match_component.get_img()}",
            }
            p { id: "central_name", "{match_component.get_name()}" }
        }

        div { id: "buttons",
            button { onclick: bad, id: "bad_button",
                "Bad!"
            }
            button { onclick: good, id:"good_button",
                "Good!"
            }
        }
    }
}
