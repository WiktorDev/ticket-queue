use dioxus::prelude::*;
use native_dialog::{MessageDialog, MessageType};
use crate::objects::TicketCase;
use rand::Rng;

#[derive(serde::Serialize)]
struct ApiMessage {
    message: String,
}

async fn downloadTicket(mut case: Signal<String>) {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(100..=999);

    let code = format!("{}{}", case.to_string(), random_number).to_string();

    let body = ApiMessage {
        message: format!("add:{}", code)
    };

    let client = reqwest::Client::new();
    let response = client.post("http://localhost:3000/send")
        .json(&body)
        .send()
        .await
        .unwrap();

    let status = response.status();
    let response_text = response.text().await.unwrap();
    println!("Status: {}", status);
    println!("Response: {}", response_text);

    case.set(String::new());
    std::thread::spawn(move || {
        MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("Drukowanie...")
            .set_text(&format!("{}\n^^^^^^^^\nTwój numer", code))
            .show_alert()
            .unwrap();
    });
}

#[component]
pub fn Home() -> Element {
    let cases = vec![
        TicketCase::new("A", "Rejestracja pojazdu", ""),
        TicketCase::new("B", "Odbiór dowowdu rejestracyjnego", ""),
        TicketCase::new("C", "Inna sprawa", "Lorem orem Ipsum is simply dummy text of the printing and typesetting industry.")
    ];
    let mut selected_case = use_signal(|| String::new());

    rsx! {
        main {
            class: "m-0 pt-[10vh] flex flex-col items-center justify-center text-center",
            h1 {
                class: "text-center text-3xl",
                "Pobierz bilet"
            }
            p { "Wybierz sprawe aby wygenerować bilet i zapisać się do kolejki" },
            div {
                class: "flex flex-col items-center gap-y-4 mt-4",
                for case in cases.into_iter() {
                    div {
                        class: format!(
                            "flex flex-col items-center bg-gray-800 bg-opacity-50 min-h-16 w-full px-4 py-6 rounded-md select-none cursor-pointer {}",
                            if selected_case.to_string() == case.code { "border border-green-500" } else { "" }
                        ),
                        onclick: move |_event| selected_case.set(case.clone().code),
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
                        // div { class: "w-1/4 border-t mt-3 mb-1" }
                        // span { "Oczekuje {case.awaiting} osób" }
                    }
                }
                div {
                    class: format!(
                        "bg-green-800 transition-all bg-opacity-50 min-h-16 w-full px-4 py-6 rounded-md select-none {}",
                        if selected_case.to_string().is_empty() {
                            "opacity-50 cursor-not-allowed"
                        } else { "cursor-pointer hover:bg-green-900 " }
                    ),
                    onclick: move |_| {
                        downloadTicket(selected_case)
                    },
                    h1 {
                        class: "text-2xl",
                        "Pobierz bilet"
                    }
                }
            }
        }
    }
}
