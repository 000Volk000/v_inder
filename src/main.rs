use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");
static STARS_BACKGROUND: Asset = asset!("/assets/stars_background.svg");
static SPACE_GROTESK_BOLD_FONT: Asset = asset!("/assets/SpaceGrotesk-Bold.ttf");
static SPACE_MONO_REGULAR_FONT: Asset = asset!("/assets/SpaceMono-Regular.ttf");

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
        Buttons {  }
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
    rsx! {
        div { id: "image",
            img { id: "central_image",
                src: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fcdn.pixabay.com%2Fphoto%2F2024%2F08%2F16%2F11%2F47%2Fai-generated-8973514_1280.jpg",
            }
        }

    }
}

#[component]
fn Buttons() -> Element {
    rsx! {
        div { id: "buttons",
            button { id: "bad_button",
                "Bad!"
            }
            button { id:"good_button",
                "Good!"
            }
        }
    }
}
