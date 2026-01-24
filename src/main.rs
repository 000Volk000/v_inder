use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");
static STARS_BACKGROUND: Asset = asset!("/assets/stars_background.svg");
static SPACE_GROTESK_BOLD_FONT: Asset = asset!("/assets/SpaceGrotesk-Bold.ttf");
static SPACE_MONO_REGULAR_FONT: Asset = asset!("/assets/SpaceMono-Regular.ttf");

mod match_class;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        document::Style {
            "body {{ --bg-stars: url('{STARS_BACKGROUND}'); }}"
        }
        LoadFonts {  }

        Header {  }
        NamesApp {  }
    }
}

#[component]
fn FontFace(family: &'static str, style: &'static str, weight: usize, asset: Asset) -> Element {
    rsx! {
        document::Style {{
            format!("
                @font-face {{
                    font-display: swap;
                    font-family: '{}';
                    font-style: {};
                    font-weight: {};
                    src: url('{}') format('woff2');
                }}
                ", family, style, weight, asset
            )
        }}
    }
}

#[component]
fn LoadFonts() -> Element {
    rsx! {
        FontFace {
            family: "Space Grotesk Bold",
            style: "normal",
            weight: 700,
            asset: SPACE_GROTESK_BOLD_FONT
        }
        FontFace {
            family: "Space Mono Regular",
            style: "normal",
            weight: 400,
            asset: SPACE_MONO_REGULAR_FONT
        }
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
