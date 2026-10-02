use dioxus::prelude::*;

const KEY: &str = "v_inder_names";

pub async fn load() -> Vec<String> {
    let js = format!(
        r#"
        let v = [];
        try {{ v = JSON.parse(localStorage.getItem("{KEY}") || "[]"); }} catch (e) {{}}
        dioxus.send(v);
        "#
    );
    document::eval(&js)
        .recv::<Vec<String>>()
        .await
        .unwrap_or_default()
}

pub fn add(name: String) {
    let js = format!(
        r#"
        const name = await dioxus.recv();
        try {{
            const v = JSON.parse(localStorage.getItem("{KEY}") || "[]");
            v.push(name);
            localStorage.setItem("{KEY}", JSON.stringify(v));
        }} catch (e) {{}}
        "#
    );
    let eval = document::eval(&js);
    let _ = eval.send(name);
}
