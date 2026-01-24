pub mod match_class;
mod match_view;
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    MatchView,
}

#[component]
fn MatchView() -> Element {
    match_view::match_view()
}
