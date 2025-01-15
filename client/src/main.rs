use dioxus::prelude::*;

use dioxus::desktop::{
    tao::dpi::PhysicalPosition, use_global_shortcut, LogicalSize, WindowBuilder,
};

use components::Navbar;
use views::{Blog, Home};

mod components;
mod views;
mod objects;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(make_config())
        .launch(App);
}

fn make_config() -> dioxus::desktop::Config {
    dioxus::desktop::Config::default().with_window(make_window())
}

fn make_window() -> WindowBuilder {
    WindowBuilder::new()
        .with_title("Pobierz bilet")
        .with_transparent(false)
        .with_decorations(true)
        .with_resizable(true)
        .with_always_on_top(false)
        .with_position(PhysicalPosition::new(0, 0))
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
