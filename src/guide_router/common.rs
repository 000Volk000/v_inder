use super::Route;
use dioxus::prelude::*;

#[component]
pub fn header() -> Element {
    rsx! {
        div { id: "title_div",
            Link { class:"icon", to: Route::MatchView,
                svg { xmlns:"http://www.w3.org/2000/svg", fill:"none", view_box:"0 0 24 24", stroke_width:"1.5", stroke:"currentColor",
                    path {
                        stroke_linecap:"round",
                        stroke_linejoin:"round",
                        d:"m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"
                    }
                }
            }
            h1 { id: "title",
                "V_inder"
            }
            Link { class:"icon", to: Route::ListView,
                svg { xmlns:"http://www.w3.org/2000/svg", fill:"none", view_box:"0 0 24 24", stroke_width:"1.5", stroke:"currentColor",
                    path {
                        stroke_linecap:"round",
                        stroke_linejoin:"round",
                        d:"M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0ZM3.75 12h.007v.008H3.75V12Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm-.375 5.25h.007v.008H3.75v-.008Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z"
                    }
                }
            }

        }
    }
}
