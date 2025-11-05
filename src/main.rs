mod pages;
mod routers;
mod components;

use dioxus::prelude::*;
use routers::Route;

fn main() {
    dioxus::launch(App);
}



#[component]
fn App() -> Element {
    rsx! {
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0, viewport-fit=cover"
        }
        document::Meta {
            name: "apple-mobile-web-app-capable",
            content: "yes"
        }
        document::Stylesheet {
            href: asset!("/assets/main.css")
        }
        document:: Stylesheet {
            href: asset!( "/assets/tailwind.css" )
        }
        Router::<Route> {}
    }
}
