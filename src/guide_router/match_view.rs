use dioxus::prelude::*;

use super::match_class;
use crate::storage;

#[component]
pub fn match_view() -> Element {
    let mut match_component = match_class::Match::new();

    let good = move |_| {
        storage::set(
            storage::length().to_string(),
            match_component.get_name().read().clone(),
        );
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
