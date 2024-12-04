#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Default, Clone)]
struct TicketCase {
    code: String,
    name: String,
    description: String,
    awaiting: u32
}
impl TicketCase {
    fn new(code: &str, name: &str, description: &str) -> Self {
        Self {
            code: code.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            awaiting: 0
        }
    }
}
const TAILWIND_URL: Asset = asset!("/assets/tailwind.css");

pub fn App() -> Element {
    // let mut name = use_signal(|| String::new());
    // let mut greet_msg = use_signal(|| String::new());
    //
    // let greet = move |_: FormEvent| async move {
    //     if name.read().is_empty() {
    //         return;
    //     }
    //
    //     let name = name.read();
    //     let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &*name }).unwrap();
    //     // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    //     let new_msg = invoke("greet", args).await.as_string().unwrap();
    //     greet_msg.set(new_msg);
    // };

    let cases = vec![
        TicketCase::new("A", "Rejestracja pojazdu", ""),
        TicketCase::new("B", "Odbiór dowowdu rejestracyjnego", ""),
        TicketCase::new("C", "Inna sprawa", "Lorem orem Ipsum is simply dummy text of the printing and typesetting industry.")
    ];
    let mut selected_case = use_signal(|| String::new());

    rsx! {
        document::Link { rel: "stylesheet", href: "/assets/styles.css" }
        document::Link { rel: "stylesheet", href: TAILWIND_URL }
        main {
            class: "m-0 pt-[10vh] flex flex-col items-center justify-center text-center",
            div {
                class: "flex justify-center",
                a {
                    class: "text-[500] text-[#646cff]",
                    href: "https://tauri.app",
                    target: "_blank",
                    img {
                        src: "/assets/tauri.svg",
                        class: "logo tauri",
                         alt: "Tauri logo"
                    }
                }
            }
            h1 {
                class: "text-center text-3xl",
                "Pobierz bilet"
            }
            p { "Wybierz sprawe aby wygenerować bilet i zapisać się do kolejki" }
            div {
                class: "flex flex-col items-center gap-y-4 mt-4",
                for case in cases.into_iter() {
                    div {
                        class: format!(
                            "flex flex-col items-center bg-gray-800 bg-opacity-50 min-h-16 w-full px-4 py-6 rounded-md select-none cursor-pointer {}",
                            if selected_case.to_string() == case.code { "border border-green-500" } else { "" }
                        ),
                        onclick: move |event| selected_case.set(case.clone().code),
                        h1 {
                            class: "text-2xl",
                            "{case.name} ({case.code})"
                        }
                        if (!case.description.is_empty()) {
                            p {
                                class: "text-xs pt-3",
                                "{case.description}"
                            }
                        }
                        div { class: "w-1/4 border-t mt-3 mb-1" }
                        span { "Oczekuje {case.awaiting} osób" }
                    }
                }
                div {
                    class: format!(
                        "bg-green-800 transition-all bg-opacity-50 min-h-16 w-full px-4 py-6 rounded-md select-none {}",
                        if selected_case.to_string().is_empty() {
                            "opacity-50 cursor-not-allowed"
                        } else { "cursor-pointer hover:bg-green-900 " }
                    ),
                    h1 {
                        class: "text-2xl",
                        "Pobierz bilet"
                    }
                }
            }
        }
    }
}