use dioxus::prelude::*;
use rand::Rng;
use v_name_generator::generate_valid_name;

static TEMPLATES: Asset = asset!("/assets/templates");
static TEMPLATES_NUM: u8 = 10;

#[derive(Copy, Clone)]
pub struct Match {
    img: Signal<String>,
    name: Signal<String>,
}

impl Match {
    pub fn new() -> Self {
        Self {
            img: use_signal(Self::random_img),
            name: use_signal(generate_valid_name),
        }
    }

    pub fn get_img(&self) -> Signal<String> {
        self.img
    }

    pub fn get_name(&self) -> Signal<String> {
        self.name
    }

    pub fn next(&mut self) {
        self.img.set(self.random_img_path());
        self.name.set(generate_valid_name());
    }

    fn random_img() -> String {
        let rng = rand::rng().random::<u8>() % TEMPLATES_NUM;
        format!("{TEMPLATES}/{rng}.jpg")
    }

    fn random_img_path(&self) -> String {
        let mut rng = rand::rng().random::<u8>() % TEMPLATES_NUM;
        if *self.img.read() == format!("{TEMPLATES}/{rng}.jpg") {
            rng = (rng + 1) % TEMPLATES_NUM;
        }

        format!("{TEMPLATES}/{rng}.jpg")
    }
}
