mod list_view;
pub mod match_class;
mod match_view;
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    MatchView,

    #[route("/list")]
    ListView,
}

#[component]
fn MatchView() -> Element {
    match_view::match_view()
}

#[component]
fn ListView() -> Element {
    list_view::list_view()
}
